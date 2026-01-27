use dioxus::prelude::*;

/// Downloads page
#[component]
pub fn Downloads() -> Element {
    rsx! {
        div { class: "space-y-6",
            h1 { class: "text-3xl font-bold text-gray-900 dark:text-white",
                "Downloads"
            }
            
            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow p-6",
                p { class: "text-gray-600 dark:text-gray-300",
                    "Download queue will be displayed here"
                }
            }
        }
    }
}
