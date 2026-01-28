use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Schedule recurrence type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Recurrence {
    Once,
    Daily,
    Weekly,
    Custom(String), // Cron expression
}

/// Schedule status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Scheduled download
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub source: String,
    pub quality: String,
    pub scheduled_time: DateTime<Utc>,
    pub recurrence: Recurrence,
    pub priority: i32,
    pub status: ScheduleStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_run: Option<DateTime<Utc>>,
    pub next_run: Option<DateTime<Utc>>,
}

impl Schedule {
    /// Create a new scheduled download
    pub fn new(
        title: String,
        url: String,
        source: String,
        quality: String,
        scheduled_time: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            url,
            source,
            quality,
            scheduled_time,
            recurrence: Recurrence::Once,
            priority: 0,
            status: ScheduleStatus::Pending,
            created_at: now,
            updated_at: now,
            last_run: None,
            next_run: Some(scheduled_time),
        }
    }
}
