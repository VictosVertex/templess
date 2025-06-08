use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdLineChart, LdSave, LdSettings, LdUpload};

#[component]
pub fn Header() -> Element {
    rsx! {
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
                class: "flex justify-end items-center gap-4 ml-auto text-foreground-secondary",
                button {
                    class: "flex items-center gap-2 hover:text-accent cursor-pointer",
                    Icon {
                        width: 18,
                        height: 18,
                        icon: LdUpload,
                    }
                }
                button {
                    class: "flex items-center gap-2 hover:text-accent cursor-pointer",
                    Icon {
                        width: 18,
                        height: 18,
                        icon: LdSave,
                    }
                }
                button {
                    class: "flex items-center gap-2 hover:text-accent cursor-pointer",
                    Icon {
                        width: 18,
                        height: 18,
                        icon: LdSettings,
                    }
                }
            }
        }
    }
}
