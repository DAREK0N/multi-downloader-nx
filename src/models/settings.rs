use serde::{Deserialize, Serialize};

/// Application settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub download_path: String,
    pub concurrent_downloads: usize,
    pub auto_start_queue: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            language: "en".to_string(),
            download_path: "./videos".to_string(),
            concurrent_downloads: 3,
            auto_start_queue: false,
        }
    }
}

impl AppSettings {
    /// Validate settings
    pub fn validate(&self) -> Result<(), String> {
        if self.download_path.is_empty() {
            return Err("Download path cannot be empty".to_string());
        }
        if self.concurrent_downloads == 0 || self.concurrent_downloads > 10 {
            return Err("Concurrent downloads must be between 1 and 10".to_string());
        }
        Ok(())
    }
    
    /// Create settings with validation
    pub fn new(
        theme: String,
        language: String,
        download_path: String,
        concurrent_downloads: usize,
        auto_start_queue: bool,
    ) -> Result<Self, String> {
        let settings = Self {
            theme,
            language,
            download_path,
            concurrent_downloads,
            auto_start_queue,
        };
        settings.validate()?;
        Ok(settings)
    }
}
