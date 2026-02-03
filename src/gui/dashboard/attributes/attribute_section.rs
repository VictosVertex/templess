use dioxus::prelude::*;

use crate::gui::dashboard::attributes::stat_data::StatData;

#[component]
pub fn AttributeSection(title: String, stats: Vec<StatData>) -> Element {
    rsx! {
        div { class: "bg-card border border-border rounded-lg p-4 shadow-sm w-full",
            h3 { class: "text-accent font-bold border-b border-border pb-2 mb-3", "{title}" }
            div { class: "grid grid-cols-1 gap-x-6 gap-y-1 max-height-[100px] overflow-y-auto",
                {
                    stats
                        .into_iter()
                        .map(|data| {
                            let is_capped = data.value >= data.cap && data.cap > 0;

                            let pct = if data.cap > 0 {
                                (data.value as f32 / data.cap as f32).min(1.0) * 100.0
                            } else {
                                0.0
                            };
                            let text_color = if data.value > data.cap && data.cap > 0 {
                                "text-yellow-500"
                            } else if is_capped {
                                "text-accent"
                            } else {
                                "text-foreground-secondary"
                            };
                            rsx! {
                                div { class: "flex flex-col gap-1 mb-2",
                                    div { class: "flex justify-between items-end text-xs",
                                        span { class: "text-foreground/90 font-medium", "{data.stat.name()}" }
                                        span { class: "{text_color} font-mono", "{data.value} / {data.cap}" }
                                    }
                                    div { class: "h-1.5 w-full bg-secondary/30 rounded-full overflow-hidden",
                                        div {
                                            class: "h-full bg-accent transition-all duration-300 ease-out",
                                            style: "width: {pct}%",
                                        }
                                    }
                                }
                            }
                        })
                }
            }
        }
    }
}
