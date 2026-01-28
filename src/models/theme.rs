use serde::{Deserialize, Serialize};

/// Theme configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub colors: ThemeColors,
}

/// Theme color definitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeColors {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub surface: String,
    pub border: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_muted: String,
}

/// Built-in themes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    OledBlack,
    SolarizedLight,
    SolarizedDark,
    Dracula,
}

impl Theme {
    pub fn to_config(&self) -> ThemeConfig {
        match self {
            Theme::Light => ThemeConfig {
                name: "light".to_string(),
                colors: ThemeColors {
                    primary: "#3b82f6".to_string(),
                    secondary: "#64748b".to_string(),
                    accent: "#8b5cf6".to_string(),
                    background: "#ffffff".to_string(),
                    surface: "#f8fafc".to_string(),
                    border: "#e2e8f0".to_string(),
                    text_primary: "#1e293b".to_string(),
                    text_secondary: "#475569".to_string(),
                    text_muted: "#94a3b8".to_string(),
                },
            },
            Theme::Dark => ThemeConfig {
                name: "dark".to_string(),
                colors: ThemeColors {
                    primary: "#60a5fa".to_string(),
                    secondary: "#94a3b8".to_string(),
                    accent: "#a78bfa".to_string(),
                    background: "#0f172a".to_string(),
                    surface: "#1e293b".to_string(),
                    border: "#334155".to_string(),
                    text_primary: "#f1f5f9".to_string(),
                    text_secondary: "#cbd5e1".to_string(),
                    text_muted: "#64748b".to_string(),
                },
            },
            Theme::OledBlack => ThemeConfig {
                name: "oled-black".to_string(),
                colors: ThemeColors {
                    primary: "#60a5fa".to_string(),
                    secondary: "#94a3b8".to_string(),
                    accent: "#a78bfa".to_string(),
                    background: "#000000".to_string(),
                    surface: "#0a0a0a".to_string(),
                    border: "#1a1a1a".to_string(),
                    text_primary: "#ffffff".to_string(),
                    text_secondary: "#d4d4d4".to_string(),
                    text_muted: "#737373".to_string(),
                },
            },
            Theme::SolarizedLight => ThemeConfig {
                name: "solarized-light".to_string(),
                colors: ThemeColors {
                    primary: "#268bd2".to_string(),
                    secondary: "#657b83".to_string(),
                    accent: "#d33682".to_string(),
                    background: "#fdf6e3".to_string(),
                    surface: "#eee8d5".to_string(),
                    border: "#93a1a1".to_string(),
                    text_primary: "#657b83".to_string(),
                    text_secondary: "#839496".to_string(),
                    text_muted: "#93a1a1".to_string(),
                },
            },
            Theme::SolarizedDark => ThemeConfig {
                name: "solarized-dark".to_string(),
                colors: ThemeColors {
                    primary: "#268bd2".to_string(),
                    secondary: "#93a1a1".to_string(),
                    accent: "#d33682".to_string(),
                    background: "#002b36".to_string(),
                    surface: "#073642".to_string(),
                    border: "#586e75".to_string(),
                    text_primary: "#839496".to_string(),
                    text_secondary: "#93a1a1".to_string(),
                    text_muted: "#586e75".to_string(),
                },
            },
            Theme::Dracula => ThemeConfig {
                name: "dracula".to_string(),
                colors: ThemeColors {
                    primary: "#8be9fd".to_string(),
                    secondary: "#6272a4".to_string(),
                    accent: "#ff79c6".to_string(),
                    background: "#282a36".to_string(),
                    surface: "#44475a".to_string(),
                    border: "#6272a4".to_string(),
                    text_primary: "#f8f8f2".to_string(),
                    text_secondary: "#f8f8f2".to_string(),
                    text_muted: "#6272a4".to_string(),
                },
            },
        }
    }
    
    /// Get all available themes
    pub fn all() -> Vec<Theme> {
        vec![
            Theme::Light,
            Theme::Dark,
            Theme::OledBlack,
            Theme::SolarizedLight,
            Theme::SolarizedDark,
            Theme::Dracula,
        ]
    }
    
    /// Parse theme from string
    pub fn from_str(s: &str) -> Option<Theme> {
        match s.to_lowercase().as_str() {
            "light" => Some(Theme::Light),
            "dark" => Some(Theme::Dark),
            "oled-black" | "oled" => Some(Theme::OledBlack),
            "solarized-light" => Some(Theme::SolarizedLight),
            "solarized-dark" => Some(Theme::SolarizedDark),
            "dracula" => Some(Theme::Dracula),
            _ => None,
        }
    }
    
    /// Get theme display name
    pub fn display_name(&self) -> &str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::OledBlack => "OLED Black",
            Theme::SolarizedLight => "Solarized Light",
            Theme::SolarizedDark => "Solarized Dark",
            Theme::Dracula => "Dracula",
        }
    }
}
