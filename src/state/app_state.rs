use dioxus::prelude::*;
use crate::models::{AppSettings, Download};

/// Global application state
#[derive(Clone, PartialEq)]
pub struct AppState {
    pub settings: Signal<AppSettings>,
    pub downloads: Signal<Vec<Download>>,
    pub is_loading: Signal<bool>,
    pub error: Signal<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            settings: Signal::new(AppSettings::default()),
            downloads: Signal::new(Vec::new()),
            is_loading: Signal::new(false),
            error: Signal::new(None),
        }
    }
    
    /// Add a download to the list
    pub fn add_download(&mut self, download: Download) {
        self.downloads.write().push(download);
    }
    
    /// Remove a download by ID
    pub fn remove_download(&mut self, id: uuid::Uuid) {
        self.downloads.write().retain(|d| d.id != id);
    }
    
    /// Get download by ID
    pub fn get_download(&self, id: uuid::Uuid) -> Option<Download> {
        self.downloads.read().iter().find(|d| d.id == id).cloned()
    }
    
    /// Update settings
    pub fn update_settings(&mut self, settings: AppSettings) {
        if settings.validate().is_ok() {
            *self.settings.write() = settings;
        }
    }
    
    /// Set loading state
    pub fn set_loading(&mut self, loading: bool) {
        *self.is_loading.write() = loading;
    }
    
    /// Set error
    pub fn set_error(&mut self, error: Option<String>) {
        *self.error.write() = error;
    }
    
    /// Clear error
    pub fn clear_error(&mut self) {
        *self.error.write() = None;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
