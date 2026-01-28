use dioxus::prelude::*;

/// Custom hook for internationalization
pub fn use_i18n() -> String {
    // Get current language from context or settings
    use_signal(|| "en".to_string())()
}

/// Translate a key - placeholder for now
pub fn t(key: &str) -> String {
    // Placeholder - will use rust_i18n::t! once i18n is fully initialized
    key.to_string()
}
