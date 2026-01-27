use dioxus::prelude::*;
use crate::router::Route;
use crate::state::{AppState, ThemeState};

/// Root application component
#[component]
pub fn App() -> Element {
    // Initialize application state
    use_context_provider(|| AppState::new());
    use_context_provider(|| Signal::new(ThemeState::new()));

    rsx! {
        // Global styles
        document::Stylesheet { href: asset!("assets/styles/tailwind.css") }
        
        // Router
        Router::<Route> {}
    }
}
