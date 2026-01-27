use dioxus::prelude::*;
use crate::router::Route;
use crate::state::AppState;

/// Root application component
#[component]
pub fn App() -> Element {
    // Initialize application state
    use_context_provider(|| AppState::new());

    rsx! {
        // Global styles
        document::Stylesheet { href: asset!("/assets/styles/tailwind.css") }
        
        // Router
        Router::<Route> {}
    }
}
