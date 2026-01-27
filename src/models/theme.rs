use serde::{Deserialize, Serialize};

/// Theme information for the theme model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeInfo {
    pub name: String,
    pub is_dark: bool,
}
