use dioxus::prelude::*;

/// Scheduled downloads page
#[component]
pub fn Scheduled() -> Element {
    rsx! {
        div { class: "space-y-6",
            h1 { class: "text-3xl font-bold text-gray-900 dark:text-white",
                "Scheduled Downloads"
            }
            
            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow p-6",
                p { class: "text-gray-600 dark:text-gray-300",
                    "Scheduled downloads will be displayed here"
                }
            }
        }
    }
}
