use std::sync::Arc;

use dioxus::prelude::*;

use crate::app_state::AppState;
use crate::core::domain::item::Item;
use crate::core::domain::item_slot::ItemSlot;

#[derive(Props, Clone, PartialEq)]
pub struct InventorySlotProps {
    class: Option<String>,
    slot_type: ItemSlot,
    items: Vec<Arc<Item>>,
    icon: Element,
}

#[component]
pub fn InventorySlot(props: InventorySlotProps) -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let load_items = move |_| {
        let binding = app_state.read().clone();
        let mut items_guard = binding.items.lock().expect("Failed to lock items");

        let some_items = items_guard.iter().filter(|item| {
            item.item_slot == props.slot_type
        }).collect::<Vec<&Item>>();

        for item in some_items {
            println!("Item: {:?} {:?}", item.name, item.weapon_hand);
        }
    };

    rsx! {
        button {
            class: "flex flex-col items-center justify-center cursor-pointer",
            onclick: load_items,
            {format!("{}", props.slot_type)}
            div {
                class: format!("w-[100px] h-[100px] bg-card rounded-b-full flex items-center justify-center border border-border hover:bg-accent/20 hover:border-accent transition-all duration-200 {}", props.class.unwrap_or("".to_string())),
                {props.icon}
            }
        }
    }
}
