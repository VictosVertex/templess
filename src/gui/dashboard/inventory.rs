//! This module defines the Inventory component for the dashboard.

use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdPlus};
use std::vec;

use crate::core::domain::item_slot::ItemSlot;
use crate::gui::dashboard::{
    circle::{Circle, Point},
    inventory_slot::InventorySlot,
};

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
                    slot_type: slot_type,
                    icon: rsx! {
                        Icon {
                            icon: icon,
                            class: "text-foreground-secondary text-3xl"
                        }
                    }
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
                    slot_type: slot_type,
                    icon: rsx! {
                        Icon {
                            icon: icon,
                            class: "text-foreground-secondary text-3xl"
                        }
                    }
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
                    slot_type: slot_type,
                    icon: rsx! {
                        Icon {
                            icon: icon,
                            class: "text-foreground-secondary text-3xl"
                        }
                    }
                }
            }
        })
        .collect();

    rsx! {
        div {
            class: "relative w-[700px] h-[700px] mx-auto flex flex-col",
            div {
                div {
                    class: "absolute w-16 h-16 bg-accent rounded-full flex items-center justify-center top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2",
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
                    center: center,
                    rotation: 0.0,
                    slot_size: 100.0,
                }
            }
        }
        div {
            class: "flex gap-8 items-center justrify-center mx-auto",
            {
                weapon_slots.iter().map(|slot| {
                    rsx! {
                            div {
                                {slot}
                            }
                        }
                    }
                )
            }
        }
    }
}
