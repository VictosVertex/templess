//! This module defines the Inventory component for the dashboard.

use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdPlus};
use std::vec;
use tokio::task::spawn_blocking;

use crate::app_state::AppState;
use crate::core::domain::item_slot::ItemSlot;
use crate::gui::dashboard::{
    circle::{Circle, Point},
    inventory_slot::InventorySlot,
};
use crate::optimization::optimize::optimize;

/// The Inventory component displays the player's inventory.
///
/// It is used to visualize and select items for different slots in the player's template.
/// The items selected in the inventory will be used in the optimization process where the goal
/// is to fill the unspecified slots with the best possible items.
///
/// Thus each inventory, a set of items associated with a character class, represents
/// a specific problem instance of the optimization problem this application is trying to solve.
#[component]
pub fn Inventory() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    
    let inner_slots = 8;
    let inner_radius = 120.0;

    let middle_slots = 6;
    let middle_radius = 240.0;
    let center = Point { x: 350.0, y: 350.0 };

    let armor_map = vec![
        (ItemSlot::Hands, LdPlus),
        (ItemSlot::Feet, LdPlus),
        (ItemSlot::Legs, LdPlus),
        (ItemSlot::Arms, LdPlus),
        (ItemSlot::Chest, LdPlus),
        (ItemSlot::Head, LdPlus),
    ];

    let jewelry_map = vec![
        (ItemSlot::Ring2, LdPlus),
        (ItemSlot::Bracer2, LdPlus),
        (ItemSlot::Bracer, LdPlus),
        (ItemSlot::Ring, LdPlus),
        (ItemSlot::Jewel, LdPlus),
        (ItemSlot::Necklace, LdPlus),
        (ItemSlot::Cloak, LdPlus),
        (ItemSlot::Belt, LdPlus),
    ];

    let weapon_maps = vec![
        (ItemSlot::RightHand, LdPlus),
        (ItemSlot::LeftHand, LdPlus),
        (ItemSlot::TwoHanded, LdPlus),
        (ItemSlot::Ranged, LdPlus),
    ];

    let armor_slots: Vec<Element> = armor_map
        .into_iter()
        .map(|(slot_type, icon)| {
            rsx! {
                InventorySlot {
                    class: "h-[70px] w-[70px]",
                    slot_type,
                    icon: rsx! {
                        Icon { icon, class: "text-foreground-secondary text-3xl" }
                    },
                }
            }
        })
        .collect();

    let jewelry_slots: Vec<Element> = jewelry_map
        .into_iter()
        .map(|(slot_type, icon)| {
            rsx! {
                InventorySlot {
                    class: "h-[50px] w-[50px] !rounded-full",
                    slot_type,
                    icon: rsx! {
                        Icon { icon, class: "text-foreground-secondary text-3xl" }
                    },
                }
            }
        })
        .collect();

    let weapon_slots: Vec<Element> = weapon_maps
        .into_iter()
        .map(|(slot_type, icon)| {
            rsx! {
                InventorySlot {
                    class: "h-[70px] w-[70px] !rounded-none",
                    slot_type,
                    icon: rsx! {
                        Icon { icon, class: "text-foreground-secondary text-3xl" }
                    },
                }
            }
        })
        .collect();
    
    let on_optimize = move |_| {
        let state = app_state.read();

        let template = match state.template.lock() {
            Ok(guard) => match guard.clone() {
                Some(tmp) => tmp,
                None => {
                    println!("Optimization aborted: No active template.");
                    return;
                }
            },
            Err(_) => {
                println!("Warning: Template mutex was poisoned.");
                return;
            }
        };

        let items = match state.items.lock() {
            Ok(guard) => guard.clone(),
            Err(_) => {
                println!("Warning: Items mutex was poisoned.");
                return;
            }
        };

        spawn(async move {
            println!("Starting optimization...");

            let result = spawn_blocking(move || optimize(template, items)).await;

            match result {
                Ok(optimization_result) => match optimization_result {
                    Ok(new_template) => {
                        println!("Optimization complete. Updating UI.");
                        if let Ok(mut guard) = app_state.write().template.lock() {
                            *guard = Some(new_template);
                        }
                    }
                    Err(e) => println!("Optimization Error: {}", e),
                },
                Err(e) => println!("Worker Thread Crashed: {}", e),
            }
        });
    };

    rsx! {
        div { class: "flex flex-col",
            div { class: "relative w-[700px] h-[700px] mx-auto flex flex-col",
                div {
                    button {
                        class: "absolute w-24 h-24 bg-accent/40 rounded-full flex items-center justify-center
                                top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2
                                transition-all duration-300 ease-in-out z-10 cursor-pointer
                                border-2 border-accent text-accent
                                hover:bg-accent/20
                                ",
                        onclick: on_optimize,

                        span { class: "font-bold text-sm tracking-wider", "OPTIMIZE" }
                    }
                    Circle {
                        total_slots: inner_slots,
                        radius: inner_radius,
                        slots: jewelry_slots,
                        center: center.clone(),
                        rotation: 0.5,
                        slot_size: 100.0,
                    }
                    Circle {
                        total_slots: middle_slots,
                        radius: middle_radius,
                        slots: armor_slots,
                        center,
                        rotation: 0.0,
                        slot_size: 100.0,
                    }
                }
            }
            div { class: "flex gap-8 items-center justrify-center mx-auto",
                {
                    weapon_slots
                        .into_iter()
                        .map(|slot| {
                            rsx! {
                                div { {slot} }
                            }
                        })
                }
            }
        }
    }
}
