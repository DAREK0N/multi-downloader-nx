use dioxus::prelude::*;
use crate::router::Route;
use crate::state::app_state::AppState;
use crate::state::theme_state::ThemeState;

/// Root application component
#[component]
pub fn App() -> Element {
    // Initialize global application state
    let mut theme_state = use_signal(|| ThemeState::new());
    let _language = use_signal(|| "en".to_string());
    
    // Initialize theme from localStorage or system preference
    #[cfg(target_arch = "wasm32")]
    use_hook(|| {
        use crate::models::Theme;
        
        // Try to load from localStorage first
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(saved_theme)) = storage.get_item("theme") {
                    if let Some(theme) = Theme::from_str(&saved_theme) {
                        theme_state.write().set_theme(theme);
                    }
                }
            }
        }
    });
    
    // Provide state to child components
    use_context_provider(|| theme_state);
    use_context_provider(|| AppState::new());
    
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        // Router handles all navigation
        Router::<Route> {}
    }
}
