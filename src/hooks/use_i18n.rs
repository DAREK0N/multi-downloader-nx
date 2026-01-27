/// Hook for accessing internationalization functions
/// 
/// Note: The full i18n macro is initialized in lib.rs
/// This hook provides a simple interface to language switching
pub fn use_i18n() -> I18n {
    I18n
}

/// I18n helper struct
#[derive(Clone, Copy)]
pub struct I18n;

impl I18n {
    /// Get current locale (defaults to "en" for now)
    pub fn locale(&self) -> String {
        // For now, return default locale
        // In a real implementation, this would check browser settings
        "en".to_string()
    }
    
    /// Set locale (placeholder for now)
    pub fn set_locale(&self, _locale: &str) {
        // Placeholder - in a real implementation, this would update the locale
        // and trigger a re-render
    }
    
    /// Get available locales
    pub fn available_locales(&self) -> Vec<&'static str> {
        vec!["en", "de", "es", "fr", "it", "pt", "pt-BR", "ru", "zh-CN", "zh-TW", "ja", "ko", "pl", "nl", "tr", "ar"]
    }
}
