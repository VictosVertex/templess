//! This module contains the dashboard page and its components.
//!
//! The dashboard is the main interface for users to interact with their templates.
//! This includes viewing and selecting items, viewing stats, starting the
//! optimization and seeing the results.

pub mod dashboard_page;
pub use dashboard_page::DashboardPage;
pub mod circle;
pub mod inventory;
pub mod inventory_slot;
pub mod item_selection_details;
pub mod item_selection_modal;
