//! This module defines the home page, the first page users see when they open the application.

use std::sync::Arc;

use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::app_state::AppState;
use crate::core::database::item_sql::get_items_by_class;
use crate::core::domain::item::Item;
use crate::core::domain::template::Template;
use crate::core::domain::{class::Class, realm::Realm};
use crate::gui::components::select::Select;
use crate::gui::routes::Route;

/// The first page users can interact with when they open the application.
#[component]
pub fn HomePage() -> Element {
    let nav = navigator();
    let app_state = use_context::<Signal<AppState>>();
    let mut selected_realm = use_signal(|| Realm::Albion);
    let mut selected_class = use_signal(|| Class::Paladin);

    let realms = Realm::iter()
        .filter(|realm| realm.id() > 0)
        .collect::<Vec<_>>();
    let classes = Class::iter()
        .filter(|class| {
            let realm: &Realm = &selected_realm.read();
            class.realm() == realm
        })
        .collect::<Vec<_>>();

    let submit = {
        move |_| {
            let binding = app_state.read().clone();
            let mut items_guard = binding.items.lock().expect("Failed to lock items");
            let connection = binding
                .db_connection
                .lock()
                .expect("Failed to lock database connection");
            let items: Vec<Item> = get_items_by_class(&connection, *selected_class.read())
                .expect("Failed to get items by class");
            *items_guard = items.into_iter().map(Arc::new).collect();
            println!("Selected Items count: {}", items_guard.len());
            println!("Selected Realm: {:?}", selected_realm.read());
            println!("Selected Class: {:?}", selected_class.read());

            let template = Template::new(*selected_class.read());

            let mut template_guard = binding.template.lock().expect("Failed to lock template");
            *template_guard = Some(template);
            println!("Template created: {template_guard:?}");

            nav.push(Route::DashboardPage {});
        }
    };

    rsx! {
        div { class: "flex flex-col w-80 gap-4 border-border p-4 mt-50 text-foreground mx-auto",
            h1 { class: "text-xl font-bold", "New Template" }
            Select {
                options: realms,
                on_select: move |realm_id| {
                    selected_realm.set(Realm::from_repr(realm_id).unwrap_or(Realm::Albion));
                },
            }
            Select {
                options: classes,
                on_select: move |class_id| {
                    selected_class.set(Class::from_repr(class_id).unwrap_or(Class::Paladin));
                },
            }
            button {
                class: "mt-4 p-3 rounded-md bg-accent/80 hover:bg-accent hover:scale-102 transition-all",
                onclick: submit,
                h1 { class: "blur-none", "Create Template" }
            }
        }
    }
}
