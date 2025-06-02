//! This crate aims to provide a comprehensive solution to the Dark Age of Camelot
//! Templating Problem.

#![warn(missing_docs)]

// pub mod app_state;
pub mod core;
pub mod initialization;
pub mod gui;
pub mod app_state;

/// Starts the application.
pub fn start() -> Result<(), Box<dyn std::error::Error>> {

    gui::app::launch();

    Ok(())
}
