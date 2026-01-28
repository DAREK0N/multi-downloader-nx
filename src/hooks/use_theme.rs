use dioxus::prelude::*;
use crate::state::ThemeState;

/// Custom hook for theme management
pub fn use_theme() -> Signal<ThemeState> {
    use_context::<Signal<ThemeState>>()
}
