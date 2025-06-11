//! This module defines the modal component and its context.
//!
//! The modal component is used to display content in a dialog box on top of everything else.
//! In order to not interfere with styling and layout of the rest of the application,
//! the modal is designed such that it can be used near the root of the component tree.
//!
//! To achieve this, it uses a context to manage the content displayed in the modal.
//! The state of the context determines whether the modal is visible and what content it displays.

use dioxus::prelude::*;

/// The context for the modal component.
///
/// It holds the content to be displayed in the modal and
/// thereby determines the visibility and content of the modal.
#[derive(Clone)]
pub struct ModalContext {
    /// The content to be displayed in the modal.
    /// If `None`, the modal is not visible.
    pub content: Option<Element>,
}

/// The general modal scaffolding to be used throughout the application.
#[component]
pub fn Modal() -> Element {
    let mut modal_context = use_context::<Signal<ModalContext>>();

    rsx! {
        match modal_context.read().content.clone() {
            None => rsx! {},
            Some(content) => rsx! {
                div {
                    class: "fixed inset-0 flex items-center justify-center bg-black/50 z-50",
                    onclick: move |_| {
                        modal_context.write().content = None;
                    },
                    div {
                        class: "bg-card p-6 rounded-lg shadow-lg relative",
                        onclick: |e| e.stop_propagation(),
                        {content}
                    }
                }
            },
        }
    }
}
