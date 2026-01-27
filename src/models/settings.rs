use serde::{Deserialize, Serialize};

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,  // Will store theme mode as string
    pub language: String,
    pub download_path: String,
    pub concurrent_downloads: usize,
    pub auto_start_downloads: bool,
    pub notifications_enabled: bool,
    pub drm_setup_complete: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: "Light".to_string(),
            language: "en".to_string(),
            download_path: "./downloads".to_string(),
            concurrent_downloads: 3,
            auto_start_downloads: false,
            notifications_enabled: true,
            drm_setup_complete: false,
        }
    }
}
