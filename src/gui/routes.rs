//! This module defines the routes, the possible URLs, for the GUI application.

use crate::gui::components::layout::Layout;
use crate::gui::dashboard::DashboardPage;
use crate::gui::home::HomePage;
use dioxus::prelude::*;

/// Structure of the GUI's routing system.
/// 
/// This enum contains all the routes available, their associated components,
/// and the layout they use.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    /// Initial route, the home page of the application.
    #[layout(Layout)]
    #[route("/")]
    HomePage {},

    /// Dashboard route, the main interactive page of the application.
    #[route("/dashboard")]
    DashboardPage {},
}
