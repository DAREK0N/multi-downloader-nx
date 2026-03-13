pub mod adn;
pub mod base;
pub mod crunchyroll;
pub mod hidive;

use crate::models::*;

/// Trait for service-specific handlers
pub trait ServiceActions {
    fn auth(
        &self,
        data: &AuthData,
    ) -> impl std::future::Future<Output = ApiResponse<String>> + Send;

    fn check_token(&self) -> impl std::future::Future<Output = ApiResponse<bool>> + Send;

    fn search(
        &self,
        data: &SearchData,
    ) -> impl std::future::Future<Output = ApiResponse<Vec<SearchItem>>> + Send;

    fn list_episodes(
        &self,
        series_id: &str,
    ) -> impl std::future::Future<Output = ApiResponse<Vec<Episode>>> + Send;

    fn available_dub_codes(&self) -> impl std::future::Future<Output = Vec<LanguageItem>> + Send;

    fn available_sub_codes(&self) -> impl std::future::Future<Output = Vec<LanguageItem>> + Send;

    fn get_default(
        &self,
        name: &str,
    ) -> impl std::future::Future<Output = ApiResponse<String>> + Send;
}

/// Unified service handler that delegates to the appropriate service
pub struct ServiceHandler {
    service: ServiceType,
}

impl ServiceHandler {
    pub fn for_service(service: &ServiceType) -> Self {
        Self {
            service: service.clone(),
        }
    }

    fn get_handler(&self) -> Box<dyn ServiceActions + Send + Sync> {
        match self.service {
            ServiceType::Crunchyroll => Box::new(crunchyroll::CrunchyrollService),
            ServiceType::Hidive => Box::new(hidive::HidiveService),
            ServiceType::Adn => Box::new(adn::AdnService),
        }
    }

    pub async fn auth(&self, data: &AuthData) -> ApiResponse<String> {
        match self.service {
            ServiceType::Crunchyroll => crunchyroll::CrunchyrollService.auth(data).await,
            ServiceType::Hidive => hidive::HidiveService.auth(data).await,
            ServiceType::Adn => adn::AdnService.auth(data).await,
        }
    }

    pub async fn check_token(&self) -> ApiResponse<bool> {
        match self.service {
            ServiceType::Crunchyroll => crunchyroll::CrunchyrollService.check_token().await,
            ServiceType::Hidive => hidive::HidiveService.check_token().await,
            ServiceType::Adn => adn::AdnService.check_token().await,
        }
    }

    pub async fn search(&self, data: &SearchData) -> ApiResponse<Vec<SearchItem>> {
        match self.service {
            ServiceType::Crunchyroll => crunchyroll::CrunchyrollService.search(data).await,
            ServiceType::Hidive => hidive::HidiveService.search(data).await,
            ServiceType::Adn => adn::AdnService.search(data).await,
        }
    }

    pub async fn list_episodes(&self, series_id: &str) -> ApiResponse<Vec<Episode>> {
        match self.service {
            ServiceType::Crunchyroll => {
                crunchyroll::CrunchyrollService
                    .list_episodes(series_id)
                    .await
            }
            ServiceType::Hidive => hidive::HidiveService.list_episodes(series_id).await,
            ServiceType::Adn => adn::AdnService.list_episodes(series_id).await,
        }
    }

    pub async fn available_dub_codes(&self) -> Vec<LanguageItem> {
        match self.service {
            ServiceType::Crunchyroll => crunchyroll::CrunchyrollService.available_dub_codes().await,
            ServiceType::Hidive => hidive::HidiveService.available_dub_codes().await,
            ServiceType::Adn => adn::AdnService.available_dub_codes().await,
        }
    }

    pub async fn available_sub_codes(&self) -> Vec<LanguageItem> {
        match self.service {
            ServiceType::Crunchyroll => crunchyroll::CrunchyrollService.available_sub_codes().await,
            ServiceType::Hidive => hidive::HidiveService.available_sub_codes().await,
            ServiceType::Adn => adn::AdnService.available_sub_codes().await,
        }
    }

    pub async fn get_default(&self, name: &str) -> ApiResponse<String> {
        match self.service {
            ServiceType::Crunchyroll => crunchyroll::CrunchyrollService.get_default(name).await,
            ServiceType::Hidive => hidive::HidiveService.get_default(name).await,
            ServiceType::Adn => adn::AdnService.get_default(name).await,
        }
    }

    pub async fn add_to_queue(&self, item: QueueItem) -> ApiResponse<()> {
        base::QUEUE_MANAGER.add_to_queue(&self.service, item).await
    }

    pub async fn get_queue(&self) -> Vec<QueueItem> {
        base::QUEUE_MANAGER.get_queue(&self.service).await
    }

    pub async fn remove_from_queue(&self, index: usize) -> ApiResponse<()> {
        base::QUEUE_MANAGER
            .remove_from_queue(&self.service, index)
            .await
    }

    pub async fn clear_queue(&self) -> ApiResponse<()> {
        base::QUEUE_MANAGER.clear_queue(&self.service).await
    }

    pub async fn set_download_queue(&self, enabled: bool) -> ApiResponse<()> {
        base::QUEUE_MANAGER
            .set_download_queue(&self.service, enabled)
            .await
    }

    pub async fn get_download_queue_state(&self) -> bool {
        base::QUEUE_MANAGER
            .get_download_queue_state(&self.service)
            .await
    }

    pub async fn is_downloading(&self) -> bool {
        base::QUEUE_MANAGER.is_downloading(&self.service).await
    }

    pub async fn get_progress(&self) -> Option<ExtendedProgress> {
        base::QUEUE_MANAGER.get_progress(&self.service).await
    }

    pub async fn get_current_item(&self) -> Option<QueueItem> {
        base::QUEUE_MANAGER.get_current_item(&self.service).await
    }
}
