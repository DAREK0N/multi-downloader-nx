use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Webhook trigger events
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WebhookTrigger {
    DownloadStarted,
    DownloadCompleted,
    DownloadFailed,
    ScheduledDownloadReminder,
    QueueCompleted,
    CdmStatusChange,
    ApplicationError,
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub triggers: Vec<WebhookTrigger>,
    pub templates: HashMap<WebhookTrigger, String>,
}

impl Webhook {
    /// Create a new webhook
    pub fn new(name: String, url: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            url,
            enabled: true,
            triggers: vec![
                WebhookTrigger::DownloadCompleted,
                WebhookTrigger::DownloadFailed,
            ],
            templates: Self::default_templates(),
        }
    }

    /// Get default message templates
    pub fn default_templates() -> HashMap<WebhookTrigger, String> {
        let mut templates = HashMap::new();
        
        templates.insert(
            WebhookTrigger::DownloadCompleted,
            r#"{
  "embeds": [{
    "title": "✅ Download Complete",
    "description": "**{{title}}**",
    "color": 5763719,
    "fields": [
      {"name": "Quality", "value": "{{quality}}", "inline": true},
      {"name": "Size", "value": "{{size}}", "inline": true},
      {"name": "Duration", "value": "{{duration}}", "inline": true}
    ],
    "thumbnail": {"url": "{{thumbnail}}"},
    "timestamp": "{{timestamp}}"
  }]
}"#.to_string(),
        );
        
        templates.insert(
            WebhookTrigger::DownloadFailed,
            r#"{
  "embeds": [{
    "title": "❌ Download Failed",
    "description": "**{{title}}**\n\nError: {{error}}",
    "color": 15548997,
    "timestamp": "{{timestamp}}"
  }]
}"#.to_string(),
        );
        
        templates
    }
}
