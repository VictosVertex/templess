//! This module defines the home page, the first page users see when they open the application.

use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::core::domain::{class::Class, realm::Realm};
use crate::gui::components::select::Select;

/// The first page users can interact with when they open the application.
#[component]
pub fn HomePage() -> Element {
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

    rsx! {
        div {
            class: "flex flex-col w-80 gap-4 border-border p-4 mt-50 text-foreground mx-auto",
            h1 {
                class: "text-xl font-bold",
                "New Template"
            }
            Select {
                options: realms,
                on_select: move |realm_id| {
                    selected_realm.set(Realm::from_repr(realm_id).unwrap_or(Realm::Albion));
                }
            }
            Select {
                options: classes,
                on_select: move |class_id| {
                    selected_class.set(Class::from_repr(class_id).unwrap_or(Class::Paladin));
                }
            }
            button {
                class: "mt-4 p-3 rounded-md bg-accent/80 hover:bg-accent hover:scale-102 transition-all",
                onclick: move |_| {
                    println!("Template created!");
                },
                h1 {
                    class: "blur-none",
                    "Create Template"}
            }
        }
    }
}
