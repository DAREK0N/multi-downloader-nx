use dioxus::prelude::*;

/// Application routes
#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(crate::pages::Layout)]
        #[route("/")]
        Home {},
        
        #[route("/downloads")]
        Downloads {},
        
        #[route("/scheduled")]
        Scheduled {},
        
        #[route("/settings")]
        Settings {},
        
        #[route("/webhooks")]
        Webhooks {},
    
    #[end_layout]
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

#[component]
fn Home() -> Element {
    crate::pages::home::Home()
}

#[component]
fn Downloads() -> Element {
    crate::pages::downloads::Downloads()
}

#[component]
fn Scheduled() -> Element {
    crate::pages::scheduled::Scheduled()
}

#[component]
fn Settings() -> Element {
    crate::pages::settings::Settings()
}

#[component]
fn Webhooks() -> Element {
    crate::pages::webhooks::Webhooks()
}

#[component]
fn PageNotFound(segments: Vec<String>) -> Element {
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
                    to: Route::Home {},
                    class: "bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    "Go Home"
                }
            }
        }
    }
}
