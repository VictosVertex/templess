//! This module defines the main application component of the GUI.

use std::sync::{Arc, Mutex};
use std::time::Instant;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdLineChart, LdPlay};

use crate::app_state::AppState;
use crate::core::config::load_config;
use crate::core::database::schema::create_tables;
use crate::initialization::item_init::initialize_items;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const POPPINS_FONT: Asset = asset!("/assets/fonts/Poppins-Regular.ttf");

#[component]
fn App() -> Element {
    let config = load_config("config.toml").expect("Failed to load configuration");

    let connection =
        rusqlite::Connection::open(config.database.path.clone()).expect("Failed to open database");

    let app_state = AppState {
        config: Arc::new(Mutex::new(config)),
        db_connection: Arc::new(Mutex::new(connection)),
    };

    use_context_provider(|| Signal::new(app_state));

    let font_face_style = format!(
        r#"
        @font-face {{
            font-family: 'Poppins';
            src: url('{}');
            font-weight: normal;
            font-style: normal;
            font-display: swap;
        }}
        "#,
        POPPINS_FONT,
    );

    rsx! {
        document::Title { "TempLess - Play More" }
        style { "{font_face_style}" }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
/// Launches the graphical user interface
pub fn launch() {
    dioxus::launch(App);
}

/// Home page
#[component]
fn Home() -> Element {
    

    let initialize = move |_| {
        let app_state = use_context::<Signal<AppState>>()();

        let config = app_state.config.lock().expect("Failed to lock database connection");
        let mut connection = app_state.db_connection.lock().expect("Failed to lock database connection");

        let start = Instant::now();
        print!("Creating tables ... ");
        create_tables(&connection).expect("Failed to create database tables");
        println!("done in {:.2?}", start.elapsed());

        let start = Instant::now();
        println!("Initializing items ... ");
        initialize_items(&mut connection, config.data.items_path.clone())
            .expect("Failed to initialize items");
        println!("done in {:.2?}", start.elapsed());

    };


    rsx! {
        div {
            header {
                class: "flex bg-card p-4 border-b border-border gap-2",
                div {
                    class: "flex items-center",
                    div {  
                        class:"bg-accent/20 text-accent w-8 h-8 rounded-sm flex items-center justify-center",
                        Icon {
                            width: 18,
                            height: 18,
                            icon: LdLineChart,
                        },
                    }
                }
                
                div {
                    h1 {class:"text-xl","TempLess"}
                    h2 {class:"text-sm text-foreground-secondary","ASP powered template optimizer"}
                }
                div {  
                    class: "ml-auto justify-center items-center flex",
                    div {
                        class: "bg-accent/60 py-1 px-6 rounded-md cursor-pointer flex gap-2",
                        class: "items-center justify-center",
                        onclick: initialize,
                    
                    Icon {
                        width: 14,
                        height: 14,
                        icon: LdPlay,
                    },
                    p {"Initialize"}  }
                }
            
            }
            div {
                p {
                    "some random text"
                }
            }
        }
    }
}