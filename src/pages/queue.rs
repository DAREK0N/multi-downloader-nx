use dioxus::prelude::*;
use crate::components::{Button, ButtonVariant, Card, ProgressBar};
use crate::models::QueueItem;

#[component]
pub fn QueuePage() -> Element {
    let queue = use_signal(|| Vec::<QueueItem>::new());

    rsx! {
        div { class: "space-y-6",
            div { class: "flex items-center justify-between",
                h1 { class: "text-3xl font-bold", "Queue" }
                div { class: "flex space-x-2",
                    Button { variant: ButtonVariant::Secondary, "Clear Queue" }
                    Button { variant: ButtonVariant::Primary, "Start Queue" }
                }
            }

            if queue().is_empty() {
                Card {
                    div { class: "text-center py-8 text-text-secondary", "Queue is empty" }
                }
            } else {
                div { class: "space-y-3",
                    for item in queue() {
                        Card {
                            div { class: "flex items-center justify-between",
                                div {
                                    p { class: "font-semibold", "Download {item.download_id}" }
                                    p { class: "text-sm text-text-secondary", "Priority: {item.priority.value()}" }
                                }
                                ProgressBar { value: 0.0 }
                            }
                        }
                    }
                }
            }
        }
    }
}
