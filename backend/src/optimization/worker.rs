//! This module provides the worker that manages the optimization process.

use crate::clingo::control::Control;
use crate::clingo::model::Model;
use crate::clingo::symbol::SymbolType;
use crate::core::domain::item_slot::ItemSlot;
use crate::core::domain::{item::Item, template::Template};
use crate::optimization::instance::{
    class_atoms, gem_atoms, item_atoms, preference_atoms, slot_atoms, stat_atoms,
    stat_baseline_atoms,
};
use anyhow::{Context, Result, anyhow};
use std::collections::HashMap;
use std::thread;
use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::sync::mpsc::UnboundedSender;

pub struct OptimizationHandle {
    cancelled: Arc<AtomicBool>,
}

impl OptimizationHandle {
    pub fn cancel(self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }
}

pub struct OptimizationResult {
    pub template: Template,
    pub slotted_gem_ids: HashMap<i32, Vec<u32>>,
}

struct ModelSelection {
    chosen_items: Vec<(ItemSlot, i32)>,
    slotted_gem_ids: HashMap<i32, Vec<u32>>,
}

/// Represents the current status of the optimization process.
pub enum OptimizeStatus {
    /// Setup of the problem instance and loading of the encoding.
    Setup,
    /// Grounding of the problem instance.
    Grounding,
    /// Solving the optimization problem and retrieving models.
    Solving,
    /// A new model has been found during the optimization process.
    NewModel(OptimizationResult),
    /// The optimization process has finished, either because all models have been found or because it was stopped.
    Finished,
    /// An error occurred during the optimization process, with a message describing the error.
    Error(String),
}

/// Starts a worker for the optimization logic in a separate thread.
///
/// This function spawns a new thread that performs the optimization process using the provided template and items.
/// It communicates the status of the optimization process back to the main thread through the provided `UnboundedSender`.
/// The worker will check the `stop_flag` periodically to determine if it should stop the optimization process and exit gracefully.
pub fn start_optimization_worker(
    template: Template,
    items: Vec<Item>,
    status_sender: UnboundedSender<OptimizeStatus>,
) -> OptimizationHandle {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_for_worker = Arc::clone(&stop_flag);

    thread::spawn(move || {
        // The main thread simply calls the logic and handles the final result
        if let Err(e) =
            run_optimization_logic(&template, &items, &status_sender, &stop_flag_for_worker)
        {
            let _ = status_sender.send(OptimizeStatus::Error(e.to_string()));
        }
    });

    OptimizationHandle {
        cancelled: stop_flag,
    }
}

/// The main logic for the optimization process, which is run in a separate thread by `start_optimization_worker`.
///
/// # Parameters
/// - `template`: The initial template for the optimization process.
/// - `items`: A vector of available items to consider during optimization.
/// - `sender`: An unbounded sender for sending optimization status updates back to the main thread
/// - `stop_flag`: An atomic boolean flag that can be set to signal the worker to stop the optimization process.
///
/// # Returns
/// - `Ok(())` if the optimization process completed successfully or was stopped gracefully.
/// - `Err(anyhow::Error)` if an error occurred during the optimization process.
fn run_optimization_logic(
    template: &Template,
    items: &[Item],
    status_sender: &UnboundedSender<OptimizeStatus>,
    stop_flag: &Arc<AtomicBool>,
) -> Result<()> {
    let _ = status_sender.send(OptimizeStatus::Setup);

    let mut asp_data = String::new();
    asp_data.push_str(&class_atoms(template.class)?);
    asp_data.push_str(&slot_atoms(template)?);
    asp_data.push_str(&preference_atoms(&template.preferences)?);
    asp_data.push_str(&stat_baseline_atoms(template, items)?);
    asp_data.push_str(&gem_atoms(&template.preferences)?);
    asp_data.push_str(&stat_atoms()?);
    asp_data.push_str(&item_atoms(items, template)?);

    println!("{:?}", template.preferences);

    let file_path = Path::new("instance.lp");
    let _ = std::fs::write(file_path, &asp_data);

    let control = Control::new()?;
    control.load("instance.lp")?;
    control.load("backend/src/optimization/encoding.lp")?;

    status_sender.send(OptimizeStatus::Grounding)?;
    control.ground()?;

    if stop_flag.load(Ordering::Relaxed) {
        return Ok(());
    }

    let _ = status_sender.send(OptimizeStatus::Solving);

    let mut handle = control.solve()?;

    loop {
        if stop_flag.load(Ordering::Relaxed) {
            let _ = handle.cancel();
            let _ = status_sender.send(OptimizeStatus::Finished);
            break;
        }

        let ready = handle.wait(0.1);

        if ready {
            match handle.model() {
                Ok(Some(model_ref)) => {
                    let selection = model_selection_from_model(model_ref)?;

                    let mut new_template = template.clone();

                    for (slot, item_id) in selection.chosen_items {
                        new_template.slots.insert(slot, item_id);
                    }

                    status_sender.send(OptimizeStatus::NewModel(OptimizationResult {
                        template: new_template,
                        slotted_gem_ids: selection.slotted_gem_ids,
                    }))?;

                    handle.resume()?;
                }
                Ok(None) => {
                    status_sender.send(OptimizeStatus::Finished)?;
                    break;
                }
                Err(e) => {
                    status_sender.send(OptimizeStatus::Error(e.to_string()))?;
                    break;
                }
            }
        }
    }
    Ok(())
}

/// Extracts the chosen items and slotted gems from a given model.
///
/// # Parameters
/// - `model`: The model from which to extract the chosen items.
///
/// # Returns
/// - `Ok(ModelSelection)` containing the chosen item IDs per slot and any gem IDs assigned
///   to crafted items if successful.
///
/// # Errors
/// - `Err(anyhow::Error)` if an error occurs during the extraction process,
///   such as parsing errors or unexpected symbol types.
fn model_selection_from_model(model: Model) -> Result<ModelSelection> {
    let symbols = model.symbols(2)?;

    let mut chosen_items = Vec::new();
    let mut slotted_gem_ids: HashMap<i32, Vec<u32>> = HashMap::new();

    for symbol in symbols {
        if symbol.kind() != SymbolType::Function {
            continue;
        }

        let name = symbol.name()?;

        let arguments = symbol.arguments()?;

        match name.as_str() {
            "slot_chosen" => {
                let slot_symbol = arguments.first().ok_or_else(|| {
                    anyhow!("Expected an argument for `slot` at index 0 in slot_chosen")
                })?;

                let item_symbol = arguments.get(1).ok_or_else(|| {
                    anyhow!("Expected an argument for `item` at index 1 in slot_chosen")
                })?;

                let slot_number = slot_symbol.number().context("Failed to parse slot ID")?;

                let slot = ItemSlot::from_repr(slot_number as u16)
                    .ok_or_else(|| anyhow!("Invalid slot representation: {}", slot_number))?;

                chosen_items.push((slot, item_symbol.number()?));
            }
            "slotted_gem" => {
                let item_symbol = arguments.first().ok_or_else(|| {
                    anyhow!("Expected an argument for `item` at index 0 in slotted_gem")
                })?;

                let gem_symbol = arguments.get(1).ok_or_else(|| {
                    anyhow!("Expected an argument for `gem` at index 1 in slotted_gem")
                })?;

                let item_id = item_symbol.number().context("Failed to parse item ID")?;
                let gem_id = gem_symbol.number().context("Failed to parse gem ID")? as u32;

                slotted_gem_ids.entry(item_id).or_default().push(gem_id);
            }
            _ => {}
        }
    }

    Ok(ModelSelection {
        chosen_items,
        slotted_gem_ids,
    })
}
