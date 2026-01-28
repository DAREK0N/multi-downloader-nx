use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Download status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Processing,
    Completed,
    Failed,
    Paused,
    Cancelled,
}

/// Download item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Download {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub status: DownloadStatus,
    pub progress: f32,
    pub speed: Option<f64>,
    pub size: Option<u64>,
    pub downloaded: u64,
    pub quality: Option<String>,
    pub thumbnail: Option<String>,
    pub service: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub error: Option<String>,
}

impl Download {
    pub fn new(title: String, url: String, service: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            url,
            status: DownloadStatus::Pending,
            progress: 0.0,
            speed: None,
            size: None,
            downloaded: 0,
            quality: None,
            thumbnail: None,
            service,
            created_at: now,
            updated_at: now,
            error: None,
        }
    }
    
    /// Validate URL format
    pub fn validate_url(&self) -> Result<(), String> {
        if self.url.is_empty() {
            return Err("URL cannot be empty".to_string());
        }
        if !self.url.starts_with("http://") && !self.url.starts_with("https://") {
            return Err("URL must start with http:// or https://".to_string());
        }
        Ok(())
    }
    
    /// Check if download is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, DownloadStatus::Downloading | DownloadStatus::Processing)
    }
    
    /// Check if download is complete
    pub fn is_complete(&self) -> bool {
        self.status == DownloadStatus::Completed
    }
    
    /// Check if download has failed
    pub fn is_failed(&self) -> bool {
        self.status == DownloadStatus::Failed
    }
    
    /// Update progress
    pub fn update_progress(&mut self, progress: f32, downloaded: u64, speed: Option<f64>) {
        self.progress = progress.clamp(0.0, 100.0);
        self.downloaded = downloaded;
        self.speed = speed;
        self.updated_at = Utc::now();
    }
    
    /// Mark as failed with error message
    pub fn mark_failed(&mut self, error: String) {
        self.status = DownloadStatus::Failed;
        self.error = Some(error);
        self.updated_at = Utc::now();
    }
}
