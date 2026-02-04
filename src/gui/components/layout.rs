//! This module defines the main layout component for the GUI.

use dioxus::prelude::*;

use crate::gui::{components::header::Header, routes::Route};

/// The application layout that includes the header and a content area.
#[component]
pub fn AppLayout(children: Element) -> Element {
    rsx! {
        div { class: "grid grid-rows-[auto_1fr] min-h-screen",
            Header {}
            {children}
        }
    }
}

/// The main layout component that wraps the application routes.
#[component]
pub fn Layout() -> Element {
    rsx! {
        AppLayout { Outlet::<Route> {} }
    }
}
