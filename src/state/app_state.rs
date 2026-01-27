use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::{download::Download, schedule::Schedule, webhook::Webhook, settings::Settings};

/// Global application state
#[derive(Clone)]
pub struct AppState {
    pub downloads: Arc<RwLock<Vec<Download>>>,
    pub schedules: Arc<RwLock<Vec<Schedule>>>,
    pub webhooks: Arc<RwLock<Vec<Webhook>>>,
    pub settings: Arc<RwLock<Settings>>,
}

impl AppState {
    /// Create a new AppState with default values
    pub fn new() -> Self {
        Self {
            downloads: Arc::new(RwLock::new(Vec::new())),
            schedules: Arc::new(RwLock::new(Vec::new())),
            webhooks: Arc::new(RwLock::new(Vec::new())),
            settings: Arc::new(RwLock::new(Settings::default())),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
