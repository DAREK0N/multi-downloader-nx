#[cfg(feature = "server")]
pub mod services;

use crate::models::*;
use dioxus::prelude::*;

/// Authenticate with a service provider
#[server]
pub async fn authenticate(
    service: ServiceType,
    auth: AuthData,
) -> Result<ApiResponse<String>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.auth(&auth).await)
}

/// Check if the current token is valid
#[server]
pub async fn check_token(service: ServiceType) -> Result<ApiResponse<bool>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.check_token().await)
}

/// Search for anime/content on a service
#[server]
pub async fn search_content(
    service: ServiceType,
    data: SearchData,
) -> Result<ApiResponse<Vec<SearchItem>>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.search(&data).await)
}

/// List episodes for a series
#[server]
pub async fn list_episodes(
    service: ServiceType,
    series_id: String,
) -> Result<ApiResponse<Vec<Episode>>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.list_episodes(&series_id).await)
}

/// Get available dub language codes
#[server]
pub async fn available_dub_codes(service: ServiceType) -> Result<Vec<LanguageItem>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.available_dub_codes().await)
}

/// Get available subtitle language codes
#[server]
pub async fn available_sub_codes(service: ServiceType) -> Result<Vec<LanguageItem>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.available_sub_codes().await)
}

/// Add item to the download queue
#[server]
pub async fn add_to_queue(
    service: ServiceType,
    item: QueueItem,
) -> Result<ApiResponse<()>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.add_to_queue(item).await)
}

/// Get the current download queue
#[server]
pub async fn get_queue(service: ServiceType) -> Result<Vec<QueueItem>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.get_queue().await)
}

/// Remove item from queue by index
#[server]
pub async fn remove_from_queue(
    service: ServiceType,
    index: usize,
) -> Result<ApiResponse<()>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.remove_from_queue(index).await)
}

/// Clear the entire download queue
#[server]
pub async fn clear_queue(service: ServiceType) -> Result<ApiResponse<()>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.clear_queue().await)
}

/// Set download queue processing state
#[server]
pub async fn set_download_queue(
    service: ServiceType,
    enabled: bool,
) -> Result<ApiResponse<()>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.set_download_queue(enabled).await)
}

/// Get download queue processing state
#[server]
pub async fn get_download_queue_state(service: ServiceType) -> Result<bool, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.get_download_queue_state().await)
}

/// Check if a download is in progress
#[server]
pub async fn is_downloading(service: ServiceType) -> Result<bool, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.is_downloading().await)
}

/// Get the application version
#[server]
pub async fn get_version() -> Result<String, ServerFnError> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

/// Get default setting value
#[server]
pub async fn get_default_setting(
    service: ServiceType,
    name: String,
) -> Result<ApiResponse<String>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.get_default(&name).await)
}

/// Get current download progress
#[server]
pub async fn get_progress(service: ServiceType) -> Result<Option<ExtendedProgress>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.get_progress().await)
}

/// Get current downloading item
#[server]
pub async fn get_current_item(service: ServiceType) -> Result<Option<QueueItem>, ServerFnError> {
    use services::ServiceHandler;
    let handler = ServiceHandler::for_service(&service);
    Ok(handler.get_current_item().await)
}
