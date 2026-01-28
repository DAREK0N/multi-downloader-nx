//! Data models and types

pub mod download;
pub mod queue;
pub mod schedule;
pub mod webhook;
pub mod settings;
pub mod theme;
pub mod service;

// Re-export commonly used types
pub use download::{Download, DownloadStatus};
pub use queue::{QueueItem, Priority};
pub use schedule::{ScheduledDownload, Recurrence};
pub use webhook::{WebhookConfig, WebhookTrigger};
pub use settings::AppSettings;
pub use theme::{Theme, ThemeConfig};
pub use service::ServiceType;
