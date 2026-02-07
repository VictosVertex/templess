use crate::clingo::control::Control;
use crate::clingo::model::Model;
use crate::clingo::symbol::SymbolType;
use crate::core::domain::item_slot::ItemSlot;
use crate::core::domain::{item::Item, template::Template};
use crate::optimization::instance::{class_atoms, item_atoms, slot_atoms, stat_atoms};
use std::thread;
use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::sync::mpsc::UnboundedSender;

pub enum OptimizeStatus {
    Setup,
    Grounding,
    Solving,
    NewModel(Template),
    Finished,
    Error(String),
}

pub fn start_optimization_worker(
    template: Template,
    items: Vec<Arc<Item>>,
    tx: UnboundedSender<OptimizeStatus>,
    stop_flag: Arc<AtomicBool>,
) {
    thread::spawn(move || {
        let _ = tx.send(OptimizeStatus::Setup);

        let mut asp_data = String::new();

        if let Ok(atoms) = class_atoms(template.class) {
            asp_data.push_str(&atoms);
        }
        if let Ok(atoms) = slot_atoms(&template) {
            asp_data.push_str(&atoms);
        }
        if let Ok(atoms) = stat_atoms() {
            asp_data.push_str(&atoms);
        }
        if let Ok(atoms) = item_atoms(&items) {
            asp_data.push_str(&atoms);
        }

        let file_path = Path::new("instance.lp");
        std::fs::write(file_path, &asp_data);

        let mut control = match Control::new() {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(OptimizeStatus::Error(e.to_string()));
                return;
            }
        };

        if let Err(e) = control.load("instance.lp") {
            let _ = tx.send(OptimizeStatus::Error(format!(
                "Loading instance failed: {}",
                e
            )));
            return;
        }

        if let Err(e) = control.load("src/optimization/encoding.lp") {
            let _ = tx.send(OptimizeStatus::Error(format!(
                "Loading encoding failed: {}",
                e
            )));
            return;
        }

        let _ = tx.send(OptimizeStatus::Grounding);

        if let Err(e) = control.ground() {
            let _ = tx.send(OptimizeStatus::Error(format!("Grounding failed: {}", e)));
            return;
        }

        if stop_flag.load(Ordering::Relaxed) {
            return;
        }

        let _ = tx.send(OptimizeStatus::Solving);

        let mut handle = match control.solve() {
            Ok(h) => h,
            Err(e) => {
                let _ = tx.send(OptimizeStatus::Error(e.to_string()));
                return;
            }
        };

        let mut count = 0;

        loop {
            if stop_flag.load(Ordering::Relaxed) {
                let _ = handle.cancel();
                let _ = tx.send(OptimizeStatus::Finished);
                break;
            }

            let ready = handle.wait(0.1);

            if ready {
                match handle.model() {
                    Ok(Some(model_ref)) => {
                        let chosen_items = match chosen_items_from_model(model_ref) {
                            Ok(s) => s,
                            Err(e) => {
                                println!("{}", e);
                                break;
                            }
                        };

                        let mut new_template = template.clone();

                        for (slot, item_id) in chosen_items {
                            if let Some(item) = items.iter().find(|i| i.id == item_id) {
                                new_template.slots.insert(slot, item.clone());
                            }
                        }

                        let _ = tx.send(OptimizeStatus::NewModel(new_template));

                        let _ = handle.resume();
                    }
                    Ok(None) => {
                        let _ = tx.send(OptimizeStatus::Finished);
                        break;
                    }
                    Err(e) => {
                        let _ = tx.send(OptimizeStatus::Error(e.to_string()));
                        break;
                    }
                }
            }
        }
    });
}

fn chosen_items_from_model(model: Model) -> Result<Vec<(ItemSlot, i32)>, String> {
    let symbols = model.symbols(2)?;

    let mut items = Vec::new();

    for symbol in symbols {
        if symbol.kind() != SymbolType::Function {
            continue;
        }

        let name = symbol.name()?;

        if name != "slot_chosen" {
            continue;
        }

        let arguments = symbol.arguments()?;

        let slot_symbol = arguments
            .get(0)
            .ok_or("Expected an argument for `slot` at index 0 in slot_chosen".to_string())?;

        let item_symbol = arguments
            .get(1)
            .ok_or("Expected an argument for `item` at index 1 in slot_chosen".to_string())?;

        let slot_number = slot_symbol.number()?;

        let slot = ItemSlot::from_repr(slot_number as u16)
            .ok_or(format!("Invalid slot representation: {}", slot_number))?;

        items.push((slot, item_symbol.number()?));
    }

    Ok(items)
}
