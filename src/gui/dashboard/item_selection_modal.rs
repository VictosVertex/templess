
use std::sync::Arc;

use dioxus::prelude::*;

use crate::app_state::AppState;
use crate::core::domain::{item::Item, item_slot::ItemSlot};
use crate::gui::{components::modal::ModalContext, dashboard::item_selection_details::ItemSelectionDetails};

#[derive(Props, Clone, PartialEq)]
pub struct ItemSelectionModalProps {
    slot_type: ItemSlot,
}


#[component]
pub fn ItemSelectionModal(props: ItemSelectionModalProps) -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let mut modal_context = use_context::<Signal<ModalContext>>();

    let mut selected_item = use_signal(|| Option::<Arc<Item>>::None);

    let items = use_memo(move || {
        let binding = app_state.read().clone();
        let items_guard = binding.items.lock().expect("Failed to lock items");
        let mut items = items_guard
            .iter()
            .filter(|item| item.item_slot == props.slot_type)
            .cloned()
            .collect::<Vec<Arc<Item>>>();

        items.sort_by(|a, b| {
            b.utility.partial_cmp(&a.utility).unwrap_or(std::cmp::Ordering::Equal)
        });

        items
    });

    let handle_close = move |_| {
        modal_context.write().content = None;
        selected_item.set(None);
    };

    let handle_select = move |_| {
        if let Some(item) = selected_item.read().clone() {
            let binding = app_state.write().clone();
            let mut template_guard = binding.template.lock().unwrap();

            template_guard.as_mut().unwrap().set_item(item.item_slot, item);
        }
        modal_context.write().content = None;
    };

rsx! {
    div {
        class: "flex flex-col gap-4",
        div {
            class: "flex gap-4 h-[30vh] w-[80vw] max-w-[800px]",  
            div {
                class: "flex flex-col w-2/3",
                h1 { 
                    class: "text-xl font-bold mb-4",
                    "Select Item for {props.slot_type}" 
                }
                div {
                    class: "overflow-y-auto h-full base-scrollbar",
                    table {
                        class: "w-full",
                        thead {
                            class: "sticky top-0 bg-card z-10",
                            tr {
                                class: "text-left text-foreground-secondary p-y-2",
                                th {
                                    class: "text-left bg-card",
                                    "Name" 
                                }
                                th {
                                    class: "text-right bg-card",
                                    "Utility" 
                                }
                            }
                        }
                        tbody {
                            {items.iter().map(|item_ref| {
                                let item = (*item_ref).clone();
                                let selected = selected_item.read().clone();
                                let color = if selected == Some(item.clone()) { "bg-accent/60" } else { "hover:bg-accent/20" };

                                rsx! {tr {
                                    class: format!("cursor-pointer {}", color),
                                    onclick: move |_| selected_item.set(Some(item.clone())),
                                    td {
                                        class: "text-left",
                                        "{item.name}"
                                    }
                                    td { 
                                        class: "text-right font-mono w-36",
                                        {format!("{:.2}", item_ref.utility)} 
                                    }
                                }}
                            })}
                        }
                    }
                }
            }
            div {  
                class: "flex flex-col w-1/3 h-full",
                {if let Some(item) = selected_item.read().clone() {
                        let item_ref = items.iter().find(|i| i.id == item.id).unwrap();
                        rsx!{
                            ItemSelectionDetails {
                                item: item_ref.clone(),
                            }
                        }
                    } else {
                        rsx! {
                            div { 
                                class: "flex items-center justify-center h-full",
                                "No item selected" 
                            }
                        
                        }
                    }
                }
            }
        }
        div {
            class: "flex",  
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