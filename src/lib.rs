//! This crate aims to provide a comprehensive solution to the Dark Age of Camelot
//! Templating Problem.

#![warn(missing_docs)]

// pub mod app_state;
pub mod app_state;
pub mod core;
pub mod gui;
pub mod initialization;

/// Starts the application.
pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    gui::app::launch();

    Ok(())
}
