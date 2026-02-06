//! This crate aims to provide a comprehensive solution to the Dark Age of Camelot
//! Templating Problem.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![warn(missing_docs)]

pub mod app_state;
pub mod clingo;
pub mod core;
pub mod gui;
pub mod initialization;
pub mod optimization;

/// Starts the application.
pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    gui::app::launch();

    Ok(())
}
