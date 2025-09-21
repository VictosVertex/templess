//! This module defines the `ItemSelectionDetails` component, which displays details of a selected item.
use std::sync::Arc;

use dioxus::prelude::*;

use crate::core::domain::item::Item;

/// Properties for the `ItemSelectionDetails` component.
#[derive(Props, Clone, PartialEq)]
pub struct ItemSelectionDetailsProps {
    /// The item to display details for.
    item: Arc<Item>,
}

/// Component used to display detailed information about a selected item.
#[component]
pub fn ItemSelectionDetails(props: ItemSelectionDetailsProps) -> Element {
    rsx! {
       div {
        class: "flex flex-col",
        h1 {
            class: "text-xl font-bold mb-4",
            "Item Details"
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
                                "Stat"
                            }
                            th {
                                class: "text-right bg-card",
                                "Value"
                            }
                        }
                    }
                    tbody {
                        {
                            props.item.bonuses.iter().map(|bonus| {
                                rsx! {
                                    tr {
                                        class: "hover:bg-accent/20",
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
