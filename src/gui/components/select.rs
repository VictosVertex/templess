//! This module defines a reusable (dropdown) Select component.

use std::fmt::Display;

use dioxus::prelude::*;

/// The struct that holds the properties for the Select component.
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps<T: Into<u16> + Clone + PartialEq + Display + 'static> {
    /// An optional label that shows above the select element.
    pub label: Option<String>,

    /// The options to be displayed in the select dropdown.
    pub options: Vec<T>,

    /// The event handler that is called when an item is selected.
    pub on_select: EventHandler<u16>,
}

/// A reusable Select component that displays a dropdown list of options.
#[component]
pub fn Select<T: Into<u16> + Clone + PartialEq + Display + 'static>(
    props: SelectProps<T>,
) -> Element {
    rsx! {
        div {
            {
                match props.label {
                    Some(label) => {
                        rsx! {
                            label {
                                r#for: "select",
                                class: "mr-2 text-xs text-foreground/70",
                                "{label}"
                            }
                        }
                    },
                    None => {rsx!{div{}}},
                }
            }


            div {
                class: "relative flex items-center after:content-['<'] after:absolute after:right-3 after:rotate-270 after:pointer-events-none",
                select {
                    class: "w-full p-3 rounded-md border border-border appearance-none bg-background text-xs focus:outline-none focus:ring-2 focus:ring-accent/50",
                    name: "select",
                    id: "select",
                    onchange: move |event| {
                        let value = event.value().parse::<u16>().unwrap_or(0);
                        props.on_select.call(value);
                    },
                    {props.options.iter().map(|item: &T| {
                        rsx! {
                            option {
                                value: "{item.clone().into() as u16}",
                                "{item}"
                            }
                        }
                    })}
                }
            }
        }
    }
}
