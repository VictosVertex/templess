
use std::sync::Arc;

use dioxus::prelude::*;

use crate::{app_state::AppState, core::domain::{item::{self, Item}, item_slot::ItemSlot}};

#[derive(Props, Clone, PartialEq)]
pub struct ItemSelectionDetailsProps {
    item: Arc<Item>,
}


#[component]
pub fn ItemSelectionDetails(props: ItemSelectionDetailsProps) -> Element {
rsx! {
       div {
        class: "flex flex-col",
        h1 { 
            class: "text-xl font-bold mb-4",
            "{props.item.name}"
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
                        {
                            props.item.bonuses.iter().map(|bonus| {
                                rsx! {
                                    tr {
                                        class: "hover:bg-accent/20 transition-all duration-200",
                                        td {
                                            class: "text-left",
                                            "{bonus.stat:?}"
                                        }
                                        td {
                                            class: "text-right",
                                            "{bonus.value}"
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}