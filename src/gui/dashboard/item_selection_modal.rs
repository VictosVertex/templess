//! This module defines the `ItemSelectionModal` component, which allows users to select an item
//! for a specific item slot in their template.
use std::sync::Arc;

use dioxus::prelude::*;

use crate::app_state::AppState;
use crate::core::domain::{item::Item, item_slot::ItemSlot};
use crate::gui::{
    components::modal::ModalContext, dashboard::item_selection_details::ItemSelectionDetails,
};

/// Properties for the `ItemSelectionModal` component.
#[derive(Props, Clone, PartialEq)]
pub struct ItemSelectionModalProps {
    /// The type of item slot for which items are being selected.
    slot_type: ItemSlot,
}

/// Modal component used to select an item for a specific item slot in the template.
#[component]
pub fn ItemSelectionModal(props: ItemSelectionModalProps) -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let mut modal_context = use_context::<Signal<ModalContext>>();

    let mut selected_item = use_signal(|| Option::<Arc<Item>>::None);

    let items = use_memo(move || {
        let binding = app_state.read().clone();
        match binding.items.lock() {
            Ok(items_guard) => {
                let target_type = match props.slot_type {
                    ItemSlot::Bracer2 => ItemSlot::Bracer,
                    ItemSlot::Ring2 => ItemSlot::Ring,
                    other => other,
                };
                let mut filtered_items = items_guard
                    .iter()
                    .filter(|item| item.item_slot == target_type)
                    .cloned()
                    .collect::<Vec<Arc<Item>>>();

                filtered_items.sort_by(|a, b| match b.utility.partial_cmp(&a.utility) {
                    Some(ordering) => ordering,
                    None => std::cmp::Ordering::Equal,
                });

                filtered_items
            }
            Err(_) => Vec::new(),
        }
    });

    let handle_close = move |_| {
        modal_context.write().content = None;
        selected_item.set(None);
    };

    let handle_select = move |_| {
        if let Some(item) = selected_item.read().clone() {
            let state = app_state.write().clone();
            if let Ok(mut template_guard) = state.template.lock() {
                if let Some(template) = template_guard.as_mut() {
                    template.set_item(props.slot_type, item);
                }
            }
        }
        modal_context.write().content = None;
    };

    let item_rows = items.iter().map(|item_ref| {
        let item = (*item_ref).clone();
        let selected = selected_item.read().clone();
        let is_selected = selected == Some(item.clone());
        let color = if is_selected {
            "bg-accent/60"
        } else {
            "hover:bg-accent/20"
        };

        rsx! {
            tr {
                key: "{item.id}",
                class: "cursor-pointer {color}",
                onclick: move |_| selected_item.set(Some(item.clone())),
                td { class: "text-left", "{item.name}" }
                td { class: "text-right font-mono w-36", {format!("{:.2}", item_ref.utility)} }
            }
        }
    });

    let details_view = {
        let selected_opt = selected_item.read();
        let valid_selection = selected_opt
            .as_ref()
            .and_then(|selected| items.iter().find(|i| i.id == selected.id));

        match valid_selection {
            Some(item_ref) => rsx! {
                ItemSelectionDetails { item: item_ref.clone() }
            },
            None => rsx! {
                div { class: "flex items-center justify-center h-full", "No item selected" }
            },
        }
    };

    rsx! {
        div { class: "flex flex-col gap-4",
            div { class: "flex gap-4 h-[30vh] w-[80vw] max-w-[800px]",
                div { class: "flex flex-col w-2/3",
                    h1 { class: "text-xl font-bold mb-4", "Select Item for {props.slot_type}" }
                    div { class: "overflow-y-auto h-full base-scrollbar",
                        table { class: "w-full",
                            thead { class: "sticky top-0 bg-card z-10",
                                tr { class: "text-left text-foreground-secondary p-y-2",
                                    th { class: "text-left bg-card", "Name" }
                                    th { class: "text-right bg-card", "Utility" }
                                }
                            }
                            tbody { {item_rows} }
                        }
                    }
                }
                div { class: "flex flex-col w-1/3 h-full", {details_view} }
            }
            div { class: "flex",
                button {
                    class: "text-foreground-secondary cursor-pointer hover:text-foreground/80",
                    onclick: handle_close,
                    "Close"
                }
                button {
                    class: "ml-auto cursor-pointer py-1 px-6 rounded-lg bg-accent hover:bg-accent/80",
                    onclick: handle_select,
                    "Select"
                }
            }
        }
    }
}
