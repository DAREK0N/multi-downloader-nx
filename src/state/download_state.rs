use dioxus::prelude::*;
use crate::models::{Download, QueueItem};
use crate::models::queue::Priority;
#[cfg(not(target_arch = "wasm32"))]
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use uuid::Uuid;

/// Download queue state
#[derive(Clone, PartialEq)]
pub struct DownloadState {
    pub queue: Signal<Vec<QueueItem>>,
    pub active_downloads: Signal<Vec<Download>>,
    pub is_running: Signal<bool>,
}

impl DownloadState {
    pub fn new() -> Self {
        Self {
            queue: Signal::new(Vec::new()),
            active_downloads: Signal::new(Vec::new()),
            is_running: Signal::new(false),
        }
    }
    
    /// Add item to queue
    pub fn add_to_queue(&mut self, download_id: Uuid, priority: Priority) {
        let item = QueueItem::with_priority(download_id, priority);
        self.queue.write().push(item);
        self.reorder_queue();
    }
    
    /// Remove item from queue
    pub fn remove_from_queue(&mut self, id: Uuid) {
        self.queue.write().retain(|item| item.id != id);
    }
    
    /// Clear entire queue
    pub fn clear_queue(&mut self) {
        self.queue.write().clear();
    }
    
    /// Reorder queue by priority and order
    pub fn reorder_queue(&mut self) {
        self.queue.write().sort_by(|a, b| {
            b.priority.value()
                .cmp(&a.priority.value())
                .then_with(|| a.order.cmp(&b.order))
        });
    }
    
    /// Get queue length
    pub fn queue_length(&self) -> usize {
        self.queue.read().len()
    }
    
    /// Get active downloads count
    pub fn active_count(&self) -> usize {
        self.active_downloads.read().len()
    }
    
    /// Start queue processing
    pub fn start(&mut self) {
        *self.is_running.write() = true;
    }
    
    /// Stop queue processing
    pub fn stop(&mut self) {
        *self.is_running.write() = false;
    }
    
    /// Check if queue is running
    pub fn is_queue_running(&self) -> bool {
        *self.is_running.read()
    }
}

impl Default for DownloadState {
    fn default() -> Self {
        Self::new()
    }
}
