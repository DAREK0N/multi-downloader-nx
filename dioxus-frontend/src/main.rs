mod components;
mod models;
mod server;

use components::auth_button::AuthButton;
use components::download_selector::DownloadSelector;
use components::episode_listing::EpisodeListing;
use components::menu_bar::MenuBar;
use components::queue_view::QueueView;
use components::search_box::SearchBox;
use components::service_selector::ServiceSelector;
use dioxus::prelude::*;
use models::*;

fn main() {
    dioxus::launch(App);
}

/// Main application component
#[component]
fn App() -> Element {
    let mut service = use_signal(|| Option::<ServiceType>::None);
    let mut selected_item = use_signal(|| Option::<SearchItem>::None);
    let mut episodes = use_signal(Vec::<Episode>::new);
    let version = env!("CARGO_PKG_VERSION").to_string();

    // If no service selected, show the selector
    if service.read().is_none() {
        return rsx! {
            ServiceSelector {
                on_select: move |svc: ServiceType| {
                    service.set(Some(svc));
                },
            }
        };
    }

    let current_service = service.read().clone().unwrap();
    let service_for_menu = current_service.clone();
    let service_for_search = current_service.clone();
    let service_for_selector = current_service.clone();
    let service_for_queue = current_service.clone();
    let service_for_auth = current_service.clone();

    // Fetch episodes handler
    let service_for_episodes = current_service.clone();
    let handle_list_episodes = move |series_id: String| {
        let svc = service_for_episodes.clone();
        spawn(async move {
            match server::list_episodes(svc, series_id).await {
                Ok(ApiResponse::Ok(eps)) => {
                    episodes.set(eps);
                }
                Ok(ApiResponse::Err(_)) | Err(_) => {
                    episodes.set(Vec::new());
                }
            }
        });
    };

    // Add to queue handler
    let service_for_add = current_service.clone();
    let handle_add_to_queue = move |opts: DownloadOptions| {
        let svc = service_for_add.clone();
        let sel = selected_item.read().clone();
        spawn(async move {
            let item = QueueItem {
                title: sel.as_ref().map(|s| s.name.clone()).unwrap_or_default(),
                episode: opts.e.clone(),
                file_name: opts.file_name.clone(),
                dl_subs: opts.dl_subs.clone(),
                parent: ParentInfo {
                    title: sel.as_ref().map(|s| s.name.clone()).unwrap_or_default(),
                    season: "1".to_string(),
                },
                quality: opts.quality,
                dl_video_once: opts.dl_video_once,
                dub_lang: opts.dub_lang.clone(),
                image: sel.as_ref().map(|s| s.image.clone()).unwrap_or_default(),
                id: opts.id.clone(),
                all: opts.all,
                but: opts.but,
                no_vids: opts.no_vids,
                no_audio: opts.no_audio,
                e: opts.e.clone(),
            };
            let _ = server::add_to_queue(svc, item).await;
        });
    };

    rsx! {
        div { class: "app",
            MenuBar {
                service: service_for_menu,
                version,
                on_change_service: move |_| {
                    service.set(None);
                    selected_item.set(None);
                    episodes.set(Vec::new());
                },
            }

            main { class: "main-content",
                div { class: "content-grid",
                    // Left panel: Search & Configuration
                    div { class: "panel panel-left",
                        div { class: "panel-section",
                            div { class: "panel-header",
                                AuthButton { service: service_for_auth }
                            }
                        }

                        div { class: "panel-section",
                            SearchBox {
                                service: service_for_search,
                                on_select: move |item: SearchItem| {
                                    selected_item.set(Some(item));
                                    episodes.set(Vec::new());
                                },
                            }
                        }

                        // Selected item info
                        if let Some(ref item) = *selected_item.read() {
                            div { class: "panel-section selected-item-info",
                                div { class: "selected-item-card",
                                    if !item.image.is_empty() {
                                        img {
                                            class: "selected-item-img",
                                            src: "{item.image}",
                                            alt: "{item.name}",
                                        }
                                    }
                                    div { class: "selected-item-details",
                                        h3 { "{item.name}" }
                                        if let Some(ref desc) = item.desc {
                                            p { class: "selected-item-desc", "{desc}" }
                                        }
                                        div { class: "selected-item-meta",
                                            span { class: "rating-badge", "★ {item.rating:.1}" }
                                            span { "ID: {item.id}" }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "panel-section",
                            DownloadSelector {
                                service: service_for_selector,
                                selected_item: selected_item.read().clone(),
                                on_add_to_queue: handle_add_to_queue,
                                on_list_episodes: handle_list_episodes,
                            }
                        }

                        div { class: "panel-section",
                            EpisodeListing {
                                episodes: episodes.read().clone(),
                                on_select_episode: move |_ep_id: String| {
                                },
                            }
                        }
                    }

                    // Right panel: Download Queue
                    div { class: "panel panel-right",
                        QueueView { service: service_for_queue }
                    }
                }
            }
        }
    }
}
