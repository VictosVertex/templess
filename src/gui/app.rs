//! This module defines the main application component of the GUI.

use std::path::Path;
use std::sync::{Arc, Mutex};

use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use tokio::task::spawn_blocking;

use crate::app_state::AppState;
use crate::core::config::load_config;
use crate::core::database::schema::create_tables;
use crate::gui::components::error::Error;
use crate::gui::components::layout::AppLayout;
use crate::gui::components::loading::Loading;
use crate::gui::components::modal::{Modal, ModalContext};
use crate::gui::routes::Route;
use crate::initialization::item_init::initialize_items;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const POPPINS_FONT: Asset = asset!("/assets/fonts/Poppins-Regular.ttf");

/// Main application component that initializes the GUI.
///
/// It sets up the application state, loads assets and instantiates the router.
#[component]
fn App() -> Element {
    let config = Arc::new(load_config("config.toml").expect("Failed to load configuration"));

    let db_path_str = config.database.path.clone();
    let db_exists = Path::new(&db_path_str).exists();

    let connection = rusqlite::Connection::open(&db_path_str).expect("Failed to open database");

    let db_connection = Arc::new(Mutex::new(connection));

    let app_state = AppState {
        config: config.clone(),
        db_connection: db_connection.clone(),
        template: Arc::new(Mutex::new(None)),
        items: Arc::new(Mutex::new(Vec::new())),
    };

    use_context_provider(|| Signal::new(app_state));
    use_context_provider(|| Signal::new(ModalContext { content: None }));

    let initialization = use_resource(move || {
        let db = db_connection.clone();
        let items_path = config.data.items_path.clone();
        async move {
            match tokio::time::timeout(
                std::time::Duration::from_secs(10),
                spawn_blocking(move || -> Result<(), anyhow::Error> {
                    let mut conn = db
                        .lock()
                        .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

                    if !db_exists {
                        println!("First run detected. Initializing...");
                        create_tables(&conn)?;
                        if std::path::Path::new(&items_path).exists() {
                            initialize_items(&mut conn, items_path)?;
                        } else {
                            return Err(anyhow::anyhow!("Data missing: {}", items_path));
                        }
                    }
                    Ok(())
                }),
            )
            .await
            {
                Ok(Ok(join_handle)) => join_handle,
                Ok(Err(join_err)) => Err(anyhow::anyhow!("Thread join failed: {}", join_err)),
                Err(timeout_err) => Err(anyhow::anyhow!("Startup timed out: {}", timeout_err)),
            }
        }
    });

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

        match &*initialization.read() {
            Some(Ok(_count)) => rsx! {
                Router::<Route> {}
                Modal {}
            },
            Some(Err(e)) => rsx! {
                AppLayout {
                    Error { title: "Initialization Failed", error: "{e}" }
                }
            },
            None => rsx! {
                AppLayout { Loading {} }
            },
        }
    }
}
/// Launches the graphical user interface
pub fn launch() {
    let head_html = r#"<link rel="stylesheet" href="assets/tailwind.css">"#;
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default()
                .with_menu(None)
                .with_window(WindowBuilder::new())
                .with_custom_head(head_html.to_string()),
        )
        .launch(App);
}
