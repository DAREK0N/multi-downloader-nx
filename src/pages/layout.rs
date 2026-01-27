use dioxus::prelude::*;
use crate::components::navigation;

/// Main layout component
#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-50 dark:bg-gray-900",
            // Navigation
            navigation::Navigation {}
            
            // Main content
            main { class: "container mx-auto px-4 py-8",
                Outlet::<crate::router::Route> {}
            }
        }
    }
}
