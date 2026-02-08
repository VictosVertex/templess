//! This module defines the inventory slot component.

use crate::app_state::AppState;
use crate::core::domain::item_slot::ItemSlot;
use crate::gui::components::modal::ModalContext;
use crate::gui::dashboard::item_selection_modal::ItemSelectionModal;
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdX};

/// Properties for the InventorySlot component.
#[derive(Props, Clone, PartialEq)]
pub struct InventorySlotProps {
    /// Optional CSS class for additional styling.
    #[props(default)]
    class: String,

    /// The type of item slot this inventory slot represents.
    slot_type: ItemSlot,

    /// The icon to be displayed in the inventory slot.
    icon: Element,
}

/// The InventorySlot component represents a single slot in the inventory.
///
/// It allows users to view and select items for that slot as it is used in the template.
#[component]
pub fn InventorySlot(props: InventorySlotProps) -> Element {
    let mut modal_ctx = use_context::<Signal<ModalContext>>();
    let mut app_state = use_context::<Signal<AppState>>();

    let load_items = move |_| {
        let content = rsx! {
            ItemSelectionModal { slot_type: props.slot_type }
        };
        modal_ctx.write().content = Some(content);
    };

    let maybe_item = use_memo(move || {
        let binding = app_state.read().clone();
        let template_guard = binding.template.lock().unwrap();
        template_guard
            .as_ref()
            .and_then(|template| template.get_item(&props.slot_type).cloned())
    });

    let has_item = maybe_item.read().is_some();
    let slot_name = props.slot_type.name().replace('_', " ");

    let state_classes = if has_item {
        "bg-accent/50 hover:bg-accent/20 hover:border-accent"
    } else {
        "bg-card hover:bg-accent/20 hover:border-accent"
    };

    let container_classes = format!(
        "rounded-b-full flex items-center justify-center border border-border transition-all duration-200 relative {} {}",
        state_classes,
        props.class
    );

    let remove_item = move |event: Event<MouseData>| {
        event.stop_propagation();

        let state = app_state.write();

        if let Ok(mut guard) = state.template.lock()
            && let Some(template) = guard.as_mut()
        {
            template.remove_item(&props.slot_type);
        }
    };

    rsx! {
        div {
            class: "flex flex-col items-center justify-center cursor-pointer group relative",
            onclick: load_items,

            span { class: "capitalize mb-1", "{slot_name}" }
            div { class: container_classes,
                {props.icon}

                if has_item {
                    div { class: "absolute -top-1 -right-1 z-10 hidden group-hover:flex",

                        button {
                            class: "rounded-full p-0.5 w-5 h-5 flex items-center cursor-pointer justify-center hover:text-error",
                            onclick: remove_item,
                            title: "Unequip Item",

                            Icon { icon: LdX }
                        }
                    }
                }
            }
        }
    }
}
