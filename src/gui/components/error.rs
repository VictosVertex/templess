//! This module defines an error component to display error messages in the GUI.

use dioxus::prelude::*;

/// A simple error component that displays an error message.
#[component]
pub fn Error(title: String, error: String) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center gap-4 bg-background",
            h1 { class: "text-2xl font-bold text-error", "{title}" }
            p { class: "text-error", "{error}" }
        }
    }
}
