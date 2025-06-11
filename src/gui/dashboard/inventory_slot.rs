use dioxus::prelude::*;
use crate::app_state::AppState;
use crate::core::domain::item_slot::ItemSlot;
use crate::gui::components::modal::ModalContext;
use crate::gui::dashboard::item_selection_modal::{ItemSelectionModal};

#[derive(Props, Clone, PartialEq)]
pub struct InventorySlotProps {
    class: Option<String>,
    slot_type: ItemSlot,
    icon: Element,
}

#[component]
pub fn InventorySlot(props: InventorySlotProps) -> Element {
    let mut modal_ctx = use_context::<Signal<ModalContext>>();
    let app_state = use_context::<Signal<AppState>>();

    let load_items = move |_| {
        let content = rsx! {ItemSelectionModal {slot_type: props.slot_type} };
        modal_ctx.write().content = Some(content);
    };

    let maybe_item = use_memo(move || {
        let binding = app_state.read().clone();
        let template_guard = binding.template.lock().unwrap();
        template_guard.as_ref().and_then(|template| template.get_item(&props.slot_type).cloned())
    });


    let has_item = maybe_item.read().is_some();

    rsx! {
        button {
            class: "flex flex-col items-center justify-center cursor-pointer",
            onclick: load_items,
            "{props.slot_type}"
            div {
                class: format!(
                    " rounded-b-full flex items-center justify-center border border-border transition-all duration-200 {} {}",
                    props.class.clone().unwrap_or_default(),
                    if has_item { 
                        "bg-accent/50 hover:bg-accent/20 hover:border-accent" 
                    } else { 
                        "bg-card hover:bg-accent/20 hover:border-accent" 
                    }
                ),
            }
        }
    }
}