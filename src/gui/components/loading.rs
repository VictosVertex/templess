//! This module defines a loading component to indicate loading states in the GUI.

use dioxus::prelude::*;

/// A simple loading component that displays a loading message.
#[component]
pub fn Loading() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center gap-4 bg-background",
            div { class: "flex text-xl text-accent animate-pulse", "Loading ..." }
        }
    }
}