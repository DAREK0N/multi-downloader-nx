use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Download status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownloadStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Paused,
}

/// Download item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    pub id: Uuid,
    pub title: String,
    pub series: Option<String>,
    pub episode: Option<String>,
    pub season: Option<String>,
    pub quality: String,
    pub source: String,
    pub url: String,
    pub file_size: Option<u64>,
    pub downloaded_size: u64,
    pub status: DownloadStatus,
    pub error: Option<String>,
    pub thumbnail: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Download {
    /// Create a new download
    pub fn new(title: String, url: String, source: String, quality: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            series: None,
            episode: None,
            season: None,
            quality,
            source,
            url,
            file_size: None,
            downloaded_size: 0,
            status: DownloadStatus::Pending,
            error: None,
            thumbnail: None,
            created_at: now,
            updated_at: now,
            completed_at: None,
        }
    }

    /// Get download progress percentage
    pub fn progress(&self) -> f64 {
        if let Some(total) = self.file_size {
            if total > 0 {
                return (self.downloaded_size as f64 / total as f64) * 100.0;
            }
        }
        0.0
    }
}
