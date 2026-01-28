use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use uuid::Uuid;

/// Discord webhook configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub triggers: Vec<WebhookTrigger>,
    pub template: String,
}

/// Webhook trigger events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WebhookTrigger {
    DownloadStarted,
    DownloadCompleted,
    DownloadFailed,
    ScheduledDownloadReminder,
    QueueCompleted,
    CdmStatusChange,
    ApplicationError,
}

impl std::fmt::Display for WebhookTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebhookTrigger::DownloadStarted => write!(f, "Download Started"),
            WebhookTrigger::DownloadCompleted => write!(f, "Download Completed"),
            WebhookTrigger::DownloadFailed => write!(f, "Download Failed"),
            WebhookTrigger::ScheduledDownloadReminder => write!(f, "Scheduled Download Reminder"),
            WebhookTrigger::QueueCompleted => write!(f, "Queue Completed"),
            WebhookTrigger::CdmStatusChange => write!(f, "CDM Status Change"),
            WebhookTrigger::ApplicationError => write!(f, "Application Error"),
        }
    }
}

impl WebhookConfig {
    pub fn new(name: String, url: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            url,
            enabled: true,
            triggers: vec![WebhookTrigger::DownloadCompleted],
            template: Self::default_template(),
        }
    }
    
    fn default_template() -> String {
        r#"{
  "embeds": [{
    "title": "✅ Download Complete",
    "description": "**{{title}}**",
    "color": 5763719,
    "fields": [
      {"name": "Quality", "value": "{{quality}}", "inline": true},
      {"name": "Size", "value": "{{size}}", "inline": true}
    ],
    "timestamp": "{{timestamp}}"
  }]
}"#.to_string()
    }
    
    /// Validate webhook configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Webhook name cannot be empty".to_string());
        }
        if self.url.is_empty() {
            return Err("Webhook URL cannot be empty".to_string());
        }
        if !self.url.starts_with("https://discord.com/api/webhooks/") &&
           !self.url.starts_with("https://discordapp.com/api/webhooks/") {
            return Err("Invalid Discord webhook URL".to_string());
        }
        Ok(())
    }
    
    /// Check if trigger is enabled
    pub fn has_trigger(&self, trigger: &WebhookTrigger) -> bool {
        self.triggers.contains(trigger)
    }
    
    /// Add trigger
    pub fn add_trigger(&mut self, trigger: WebhookTrigger) {
        if !self.has_trigger(&trigger) {
            self.triggers.push(trigger);
        }
    }
    
    /// Remove trigger
    pub fn remove_trigger(&mut self, trigger: &WebhookTrigger) {
        self.triggers.retain(|t| t != trigger);
    }
}
