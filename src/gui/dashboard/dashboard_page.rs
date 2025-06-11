//! This module defines the dashboard page, the main interaction point
//! for users.

use dioxus::prelude::*;

use crate::gui::dashboard::inventory::Inventory;

/// The main page the users can interact with when they opened a template.
///
/// This page should contain the main data representation and functionality
/// for the template, such as viewing stats, adding items and starting the
/// optimization process.
#[component]
pub fn DashboardPage() -> Element {
    rsx! {
        div {
            class: "flex flex-col border-border mx-auto",
            Inventory {  }
        }
    }
}
