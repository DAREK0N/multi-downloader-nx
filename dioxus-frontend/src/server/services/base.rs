use crate::models::*;
use std::collections::HashMap;
use std::sync::LazyLock;
use tokio::sync::RwLock;

/// Global queue manager instance
pub static QUEUE_MANAGER: LazyLock<QueueManager> = LazyLock::new(QueueManager::new);

/// Per-service download state
struct ServiceDownloadState {
    queue: Vec<QueueItem>,
    is_downloading: bool,
    queue_enabled: bool,
    current_item: Option<QueueItem>,
    progress: Option<ExtendedProgress>,
}

impl Default for ServiceDownloadState {
    fn default() -> Self {
        Self {
            queue: Vec::new(),
            is_downloading: false,
            queue_enabled: false,
            current_item: None,
            progress: None,
        }
    }
}

/// Queue manager for handling download operations across services
pub struct QueueManager {
    states: RwLock<HashMap<String, ServiceDownloadState>>,
}

impl QueueManager {
    pub fn new() -> Self {
        Self {
            states: RwLock::new(HashMap::new()),
        }
    }

    fn service_key(service: &ServiceType) -> String {
        match service {
            ServiceType::Crunchyroll => "crunchyroll".to_string(),
            ServiceType::Hidive => "hidive".to_string(),
            ServiceType::Adn => "adn".to_string(),
        }
    }

    pub async fn add_to_queue(&self, service: &ServiceType, item: QueueItem) -> ApiResponse<()> {
        let key = Self::service_key(service);
        let mut states = self.states.write().await;
        let state = states.entry(key).or_default();
        state.queue.push(item);
        ApiResponse::ok(())
    }

    pub async fn get_queue(&self, service: &ServiceType) -> Vec<QueueItem> {
        let key = Self::service_key(service);
        let states = self.states.read().await;
        states
            .get(&key)
            .map(|s| s.queue.clone())
            .unwrap_or_default()
    }

    pub async fn remove_from_queue(&self, service: &ServiceType, index: usize) -> ApiResponse<()> {
        let key = Self::service_key(service);
        let mut states = self.states.write().await;
        let state = states.entry(key).or_default();
        if index < state.queue.len() {
            state.queue.remove(index);
            ApiResponse::ok(())
        } else {
            ApiResponse::err("Index out of bounds")
        }
    }

    pub async fn clear_queue(&self, service: &ServiceType) -> ApiResponse<()> {
        let key = Self::service_key(service);
        let mut states = self.states.write().await;
        let state = states.entry(key).or_default();
        state.queue.clear();
        ApiResponse::ok(())
    }

    pub async fn set_download_queue(
        &self,
        service: &ServiceType,
        enabled: bool,
    ) -> ApiResponse<()> {
        let key = Self::service_key(service);
        let key_for_spawn = key.clone();
        let mut states = self.states.write().await;
        let state = states.entry(key).or_default();
        state.queue_enabled = enabled;

        // If enabling and there are items in queue and not currently downloading,
        // start processing the next item
        if enabled && !state.queue.is_empty() && !state.is_downloading {
            let item = state.queue.remove(0);
            state.current_item = Some(item.clone());
            state.is_downloading = true;

            // Simulate starting download in background
            tokio::spawn(simulate_download(key_for_spawn));
        }

        ApiResponse::ok(())
    }

    pub async fn get_download_queue_state(&self, service: &ServiceType) -> bool {
        let key = Self::service_key(service);
        let states = self.states.read().await;
        states.get(&key).map(|s| s.queue_enabled).unwrap_or(false)
    }

    pub async fn is_downloading(&self, service: &ServiceType) -> bool {
        let key = Self::service_key(service);
        let states = self.states.read().await;
        states.get(&key).map(|s| s.is_downloading).unwrap_or(false)
    }

    pub async fn get_progress(&self, service: &ServiceType) -> Option<ExtendedProgress> {
        let key = Self::service_key(service);
        let states = self.states.read().await;
        states.get(&key).and_then(|s| s.progress.clone())
    }

    pub async fn get_current_item(&self, service: &ServiceType) -> Option<QueueItem> {
        let key = Self::service_key(service);
        let states = self.states.read().await;
        states.get(&key).and_then(|s| s.current_item.clone())
    }
}

/// Simulate a download process for demonstration purposes.
/// In a full implementation, this would call the actual download logic
/// from the service handlers (HLS download, DRM decryption, video merging, etc.)
fn simulate_download(
    service_key: String,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
    Box::pin(async move {
        // Simulate download phases
        for i in 0..=100 {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            let progress = ExtendedProgress {
                progress: ProgressData {
                    total: 100,
                    cur: i,
                    percent: i as f64,
                    time: i as f64 * 0.1,
                    download_speed: 2_500_000.0,
                    bytes: i * 250_000,
                },
                download_info: DownloadInfo {
                    image: String::new(),
                    parent_title: "Series".to_string(),
                    title: "Episode".to_string(),
                    language: LanguageItem {
                        code: "jpn".to_string(),
                        name: "Japanese".to_string(),
                    },
                    file_name: "output.mkv".to_string(),
                },
            };

            let mut states = QUEUE_MANAGER.states.write().await;
            if let Some(state) = states.get_mut(&service_key) {
                state.progress = Some(progress);
            }
        }

        // Finish download
        let mut states = QUEUE_MANAGER.states.write().await;
        if let Some(state) = states.get_mut(&service_key) {
            state.is_downloading = false;
            state.current_item = None;
            state.progress = None;

            // Process next item if queue is enabled
            if state.queue_enabled && !state.queue.is_empty() {
                let item = state.queue.remove(0);
                state.current_item = Some(item);
                state.is_downloading = true;
                let key_clone = service_key.clone();
                drop(states);

                tokio::spawn(simulate_download(key_clone));
            }
        }
    })
}
