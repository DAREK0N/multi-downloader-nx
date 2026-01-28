use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Schedule recurrence type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Recurrence {
    Once,
    Daily,
    Weekly,
    Custom(String), // Cron expression
}

impl std::fmt::Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Recurrence::Once => write!(f, "Once"),
            Recurrence::Daily => write!(f, "Daily"),
            Recurrence::Weekly => write!(f, "Weekly"),
            Recurrence::Custom(expr) => write!(f, "Custom ({})", expr),
        }
    }
}

/// Scheduled download
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduledDownload {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub service: String,
    pub scheduled_time: DateTime<Utc>,
    pub recurrence: Recurrence,
    pub enabled: bool,
    pub last_run: Option<DateTime<Utc>>,
    pub next_run: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl ScheduledDownload {
    pub fn new(title: String, url: String, service: String, scheduled_time: DateTime<Utc>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            url,
            service,
            scheduled_time,
            recurrence: Recurrence::Once,
            enabled: true,
            last_run: None,
            next_run: scheduled_time,
            created_at: now,
        }
    }
    
    /// Validate schedule
    pub fn validate(&self) -> Result<(), String> {
        if self.title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if self.url.is_empty() {
            return Err("URL cannot be empty".to_string());
        }
        let now = Utc::now();
        if self.enabled && self.next_run < now {
            return Err("Next run time cannot be in the past".to_string());
        }
        Ok(())
    }
    
    /// Check if schedule should run now
    pub fn should_run(&self) -> bool {
        self.enabled && self.next_run <= Utc::now()
    }
    
    /// Calculate next run time based on recurrence
    pub fn calculate_next_run(&self) -> DateTime<Utc> {
        use chrono::Duration;
        
        match &self.recurrence {
            Recurrence::Once => self.scheduled_time,
            Recurrence::Daily => {
                if let Some(last) = self.last_run {
                    last + Duration::days(1)
                } else {
                    self.scheduled_time
                }
            },
            Recurrence::Weekly => {
                if let Some(last) = self.last_run {
                    last + Duration::weeks(1)
                } else {
                    self.scheduled_time
                }
            },
            Recurrence::Custom(_cron) => {
                // TODO: Implement cron parsing
                self.scheduled_time
            },
        }
    }
    
    /// Mark as executed
    pub fn mark_executed(&mut self) {
        self.last_run = Some(Utc::now());
        self.next_run = self.calculate_next_run();
        
        if self.recurrence == Recurrence::Once {
            self.enabled = false;
        }
    }
}
