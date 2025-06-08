use dioxus::prelude::*;

use crate::gui::{components::header::Header, routes::Route};

#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            Header {}
            Outlet::<Route> {}
        }
    }
}
