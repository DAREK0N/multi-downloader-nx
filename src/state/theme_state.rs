use dioxus::prelude::*;
use crate::models::{Theme, ThemeConfig};

/// Theme state management
#[derive(Clone, PartialEq)]
pub struct ThemeState {
    pub current_theme: Signal<Theme>,
    pub config: Signal<ThemeConfig>,
}

impl ThemeState {
    pub fn new() -> Self {
        let theme = Theme::Light;
        Self {
            current_theme: Signal::new(theme),
            config: Signal::new(theme.to_config()),
        }
    }
    
    /// Set theme and update configuration
    pub fn set_theme(&mut self, theme: Theme) {
        *self.current_theme.write() = theme;
        *self.config.write() = theme.to_config();
        
        // Save to localStorage
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("theme", theme.display_name());
                }
            }
        }
        
        self.apply_theme_class(theme);
    }
    
    /// Apply theme CSS class to document
    fn apply_theme_class(&self, theme: Theme) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        let class_list = body.class_list();
                        
                        // Remove all theme classes
                        let _ = class_list.remove_6(
                            "dark", 
                            "theme-oled-black", 
                            "theme-solarized-light", 
                            "theme-solarized-dark",
                            "theme-dracula",
                            "theme-light"
                        );
                        
                        // Add appropriate theme class
                        match theme {
                            Theme::Light => { let _ = class_list.add_1("theme-light"); },
                            Theme::Dark => { let _ = class_list.add_1("dark"); },
                            Theme::OledBlack => { let _ = class_list.add_1("theme-oled-black"); },
                            Theme::SolarizedLight => { let _ = class_list.add_1("theme-solarized-light"); },
                            Theme::SolarizedDark => { let _ = class_list.add_1("theme-solarized-dark"); },
                            Theme::Dracula => { let _ = class_list.add_1("theme-dracula"); },
                        }
                    }
                }
            }
        }
    }
    
    /// Get current theme name
    pub fn get_theme_name(&self) -> String {
        self.current_theme.read().display_name().to_string()
    }
    
    /// Load theme from string
    pub fn load_theme_from_string(&mut self, theme_str: &str) {
        if let Some(theme) = Theme::from_str(theme_str) {
            self.set_theme(theme);
        }
    }
}

impl Default for ThemeState {
    fn default() -> Self {
        Self::new()
    }
}
