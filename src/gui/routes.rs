use crate::gui::components::layout::Layout;
use crate::gui::home::HomePage;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    HomePage {},
}
