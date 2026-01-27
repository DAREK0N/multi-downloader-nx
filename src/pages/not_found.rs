use dioxus::prelude::*;

/// 404 Not Found page
#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { class: "min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900",
            div { class: "text-center",
                h1 { class: "text-6xl font-bold text-gray-900 dark:text-white mb-4",
                    "404"
                }
                p { class: "text-xl text-gray-600 dark:text-gray-300 mb-8",
                    "Page not found: /{segments.join(\"/\")}"
                }
                Link {
                    to: crate::router::Route::Home {},
                    class: "bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    "Go Home"
                }
            }
        }
    }
}
