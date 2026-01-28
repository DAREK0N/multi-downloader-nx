use serde::{Deserialize, Serialize};

/// Theme modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    OledBlack,
    SolarizedLight,
    SolarizedDark,
    Dracula,
    Custom(usize), // Index into custom themes
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Light
    }
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub surface: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_muted: String,
    pub border: String,
    pub border_radius: String,
    pub shadow: String,
}

impl Theme {
    /// Create a light theme
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            primary: "#3b82f6".to_string(),
            secondary: "#8b5cf6".to_string(),
            accent: "#10b981".to_string(),
            background: "#ffffff".to_string(),
            surface: "#f3f4f6".to_string(),
            text_primary: "#111827".to_string(),
            text_secondary: "#4b5563".to_string(),
            text_muted: "#9ca3af".to_string(),
            border: "#e5e7eb".to_string(),
            border_radius: "0.5rem".to_string(),
            shadow: "0 1px 3px 0 rgb(0 0 0 / 0.1)".to_string(),
        }
    }

    /// Create a dark theme
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            primary: "#3b82f6".to_string(),
            secondary: "#8b5cf6".to_string(),
            accent: "#10b981".to_string(),
            background: "#111827".to_string(),
            surface: "#1f2937".to_string(),
            text_primary: "#f9fafb".to_string(),
            text_secondary: "#d1d5db".to_string(),
            text_muted: "#6b7280".to_string(),
            border: "#374151".to_string(),
            border_radius: "0.5rem".to_string(),
            shadow: "0 1px 3px 0 rgb(0 0 0 / 0.3)".to_string(),
        }
    }

    /// Create an OLED Black theme
    pub fn oled_black() -> Self {
        Self {
            name: "OLED Black".to_string(),
            primary: "#3b82f6".to_string(),
            secondary: "#8b5cf6".to_string(),
            accent: "#10b981".to_string(),
            background: "#000000".to_string(),
            surface: "#0a0a0a".to_string(),
            text_primary: "#ffffff".to_string(),
            text_secondary: "#d1d5db".to_string(),
            text_muted: "#6b7280".to_string(),
            border: "#1f2937".to_string(),
            border_radius: "0.5rem".to_string(),
            shadow: "0 1px 3px 0 rgb(0 0 0 / 0.5)".to_string(),
        }
    }

    /// Create a Solarized Light theme
    pub fn solarized_light() -> Self {
        Self {
            name: "Solarized Light".to_string(),
            primary: "#268bd2".to_string(),
            secondary: "#6c71c4".to_string(),
            accent: "#859900".to_string(),
            background: "#fdf6e3".to_string(),
            surface: "#eee8d5".to_string(),
            text_primary: "#002b36".to_string(),
            text_secondary: "#586e75".to_string(),
            text_muted: "#93a1a1".to_string(),
            border: "#93a1a1".to_string(),
            border_radius: "0.5rem".to_string(),
            shadow: "0 1px 3px 0 rgb(0 0 0 / 0.1)".to_string(),
        }
    }

    /// Create a Solarized Dark theme
    pub fn solarized_dark() -> Self {
        Self {
            name: "Solarized Dark".to_string(),
            primary: "#268bd2".to_string(),
            secondary: "#6c71c4".to_string(),
            accent: "#859900".to_string(),
            background: "#002b36".to_string(),
            surface: "#073642".to_string(),
            text_primary: "#fdf6e3".to_string(),
            text_secondary: "#93a1a1".to_string(),
            text_muted: "#657b83".to_string(),
            border: "#586e75".to_string(),
            border_radius: "0.5rem".to_string(),
            shadow: "0 1px 3px 0 rgb(0 0 0 / 0.3)".to_string(),
        }
    }

    /// Create a Dracula theme
    pub fn dracula() -> Self {
        Self {
            name: "Dracula".to_string(),
            primary: "#bd93f9".to_string(),
            secondary: "#ff79c6".to_string(),
            accent: "#50fa7b".to_string(),
            background: "#282a36".to_string(),
            surface: "#44475a".to_string(),
            text_primary: "#f8f8f2".to_string(),
            text_secondary: "#f8f8f2".to_string(),
            text_muted: "#6272a4".to_string(),
            border: "#6272a4".to_string(),
            border_radius: "0.5rem".to_string(),
            shadow: "0 1px 3px 0 rgb(0 0 0 / 0.3)".to_string(),
        }
    }
}

/// Theme state management
#[derive(Clone)]
pub struct ThemeState {
    pub mode: ThemeMode,
    pub custom_themes: Vec<Theme>,
}

impl ThemeState {
    pub fn new() -> Self {
        Self {
            mode: ThemeMode::default(),
            custom_themes: Vec::new(),
        }
    }

    /// Get the current theme
    pub fn current_theme(&self) -> Theme {
        match self.mode {
            ThemeMode::Light => Theme::light(),
            ThemeMode::Dark => Theme::dark(),
            ThemeMode::OledBlack => Theme::oled_black(),
            ThemeMode::SolarizedLight => Theme::solarized_light(),
            ThemeMode::SolarizedDark => Theme::solarized_dark(),
            ThemeMode::Dracula => Theme::dracula(),
            ThemeMode::Custom(idx) => {
                self.custom_themes
                    .get(idx)
                    .cloned()
                    .unwrap_or_else(Theme::light)
            }
        }
    }
}

impl Default for ThemeState {
    fn default() -> Self {
        Self::new()
    }
}
