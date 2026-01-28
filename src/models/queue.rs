use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use uuid::Uuid;

/// Queue item priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Normal,
    High,
}

/// Item in the download queue
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: Uuid,
    pub download_id: Uuid,
    pub priority: Priority,
    pub order: i32,
}

impl QueueItem {
    pub fn new(download_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            download_id,
            priority: Priority::Normal,
            order: 0,
        }
    }
    
    /// Create with specific priority
    pub fn with_priority(download_id: Uuid, priority: Priority) -> Self {
        Self {
            id: Uuid::new_v4(),
            download_id,
            priority,
            order: 0,
        }
    }
    
    /// Update priority
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }
    
    /// Update order position
    pub fn set_order(&mut self, order: i32) {
        self.order = order;
    }
}

impl Priority {
    /// Get numeric value for sorting
    pub fn value(&self) -> i32 {
        match self {
            Priority::Low => 1,
            Priority::Normal => 2,
            Priority::High => 3,
        }
    }
}
