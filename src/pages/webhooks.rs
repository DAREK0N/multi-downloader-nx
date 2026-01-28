use dioxus::prelude::*;

/// Webhooks configuration page
#[component]
pub fn Webhooks() -> Element {
    rsx! {
        div { class: "space-y-6",
            h1 { class: "text-3xl font-bold text-gray-900 dark:text-white",
                "Discord Webhooks"
            }
            
            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow p-6",
                p { class: "text-gray-600 dark:text-gray-300",
                    "Webhook configuration will be displayed here"
                }
            }
        }
    }
}
