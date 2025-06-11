//! This module defines the main layout component for the GUI.

use dioxus::prelude::*;

use crate::gui::{components::header::Header, routes::Route};

/// The main layout component that makes some components available
/// to all routes.
#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            Header {}
            Outlet::<Route> {}
        }
    }
}
