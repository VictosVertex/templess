//! This module defines the main application component of the GUI.

use std::sync::{Arc, Mutex};

use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

use crate::app_state::AppState;
use crate::core::config::load_config;
use crate::gui::components::modal::{Modal, ModalContext};
use crate::gui::routes::Route;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const POPPINS_FONT: Asset = asset!("/assets/fonts/Poppins-Regular.ttf");

/// Main application component that initializes the GUI.
///
/// It sets up the application state, loads assets and instantiates the router.
#[component]
fn App() -> Element {
    let config = load_config("config.toml").expect("Failed to load configuration");

    let connection =
        rusqlite::Connection::open(config.database.path.clone()).expect("Failed to open database");

    let app_state = AppState {
        config: Arc::new(config),
        db_connection: Arc::new(Mutex::new(connection)),
        template: Arc::new(Mutex::new(None)),
        items: Arc::new(Mutex::new(Vec::new())),
    };

    use_context_provider(|| Signal::new(app_state));
    use_context_provider(|| Signal::new(ModalContext { content: None }));

    let font_face_style = format!(
        r#"
        @font-face {{
            font-family: 'Poppins';
            src: url('{POPPINS_FONT}');
            font-weight: normal;
            font-style: normal;
            font-display: swap;
        }}
        "#,
    );

    rsx! {
        document::Title { "TempLess - Play More" }
        style { "{font_face_style}" }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
        Modal {  }
    }
}
/// Launches the graphical user interface
pub fn launch() {
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default()
                .with_menu(None)
                .with_window(WindowBuilder::new()),
        )
        .launch(App);
}
