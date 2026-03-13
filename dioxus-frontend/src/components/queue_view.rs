use crate::models::*;
use crate::server::{
    clear_queue, get_current_item, get_download_queue_state, get_progress, get_queue,
    remove_from_queue, set_download_queue,
};
use dioxus::prelude::*;

/// Queue view component - shows download progress and queued items
#[component]
pub fn QueueView(service: ServiceType) -> Element {
    let mut queue = use_signal(Vec::<QueueItem>::new);
    let mut current_item = use_signal(|| Option::<QueueItem>::None);
    let mut progress = use_signal(|| Option::<ExtendedProgress>::None);
    let mut queue_enabled = use_signal(|| false);

    // Poll for updates
    let service_poll = service.clone();
    use_effect(move || {
        let svc = service_poll.clone();
        spawn(async move {
            loop {
                // Fetch queue state
                if let Ok(q) = get_queue(svc.clone()).await {
                    queue.set(q);
                }
                if let Ok(p) = get_progress(svc.clone()).await {
                    progress.set(p);
                }
                if let Ok(c) = get_current_item(svc.clone()).await {
                    current_item.set(c);
                }
                if let Ok(state) = get_download_queue_state(svc.clone()).await {
                    queue_enabled.set(state);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        });
    });

    let service_toggle = service.clone();
    let service_remove = service.clone();
    let service_clear = service.clone();

    rsx! {
        div { class: "queue-view",
            // Queue header with controls
            div { class: "queue-header",
                h3 { class: "section-title", "📥 Download Queue" }
                div { class: "queue-controls",
                    button {
                        class: if *queue_enabled.read() { "btn btn-danger btn-sm" } else { "btn btn-success btn-sm" },
                        onclick: {
                            let svc = service_toggle.clone();
                            move |_| {
                                let svc = svc.clone();
                                let enabled = *queue_enabled.read();
                                spawn(async move {
                                    let _ = set_download_queue(svc, !enabled).await;
                                    queue_enabled.set(!enabled);
                                });
                            }
                        },
                        if *queue_enabled.read() {
                            "⏸ Pause"
                        } else {
                            "▶ Start"
                        }
                    }
                    button {
                        class: "btn btn-outline btn-sm",
                        onclick: {
                            let svc = service_clear.clone();
                            move |_| {
                                let svc = svc.clone();
                                spawn(async move {
                                    let _ = clear_queue(svc).await;
                                });
                            }
                        },
                        "🗑 Clear"
                    }
                }
            }

            // Current download progress
            if let Some(ref prog) = *progress.read() {
                div { class: "current-download",
                    div { class: "download-info-card",
                        div { class: "download-info-details",
                            h4 { class: "download-title",
                                "{prog.download_info.parent_title} - {prog.download_info.title}"
                            }
                            p { class: "download-meta",
                                span { class: "lang-tag", "{prog.download_info.language.name}" }
                                span { class: "download-filename", "{prog.download_info.file_name}" }
                            }
                        }
                        div { class: "progress-container",
                            div { class: "progress-bar",
                                div {
                                    class: "progress-fill",
                                    style: "width: {prog.progress.percent}%",
                                }
                            }
                            div { class: "progress-stats",
                                span { "{prog.progress.percent:.1}%" }
                                span { "{format_bytes(prog.progress.bytes)}" }
                                span { "{format_speed(prog.progress.download_speed)}" }
                                span { "{prog.progress.cur}/{prog.progress.total} parts" }
                            }
                        }
                    }
                }
            } else if current_item.read().is_some() {
                // Item started but no progress yet
                div { class: "current-download",
                    div { class: "download-info-card loading",
                        div { class: "download-preparing",
                            span { class: "spinner", "" }
                            span { " Preparing download..." }
                        }
                    }
                }
            }

            // Queue items
            div { class: "queue-items",
                if queue.read().is_empty() && progress.read().is_none() {
                    div { class: "queue-empty",
                        div { class: "queue-empty-icon", "📭" }
                        p { "No items in the queue" }
                        p { class: "queue-empty-hint", "Search and add anime episodes above" }
                    }
                }

                for (idx, item) in queue.read().iter().enumerate() {
                    {
                        let svc = service_remove.clone();
                        rsx! {
                            div { class: "queue-item",
                                if !item.image.is_empty() {
                                    img {
                                        class: "queue-item-img",
                                        src: "{item.image}",
                                        alt: "{item.title}",
                                    }
                                } else {
                                    div { class: "queue-item-img queue-item-placeholder",
                                        "🎬"
                                    }
                                }
                                div { class: "queue-item-info",
                                    h5 { class: "queue-item-title", "{item.parent.title}" }
                                    p { class: "queue-item-episode",
                                        "{item.title} - {item.episode}"
                                    }
                                    div { class: "queue-item-meta",
                                        for lang in item.dub_lang.iter() {
                                            span { class: "lang-tag lang-tag-sm", "🔊 {lang}" }
                                        }
                                        span { class: "quality-tag",
                                            if item.quality == 0 { "Best" } else { "" }
                                            if item.quality != 0 { "{item.quality}p" } else { "" }
                                        }
                                    }
                                }
                                button {
                                    class: "btn btn-icon btn-danger-icon",
                                    title: "Remove from queue",
                                    onclick: move |_| {
                                        let svc = svc.clone();
                                        spawn(async move {
                                            let _ = remove_from_queue(svc, idx).await;
                                        });
                                    },
                                    "✕"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn format_speed(bytes_per_sec: f64) -> String {
    if bytes_per_sec < 1024.0 {
        format!("{bytes_per_sec:.0} B/s")
    } else if bytes_per_sec < 1024.0 * 1024.0 {
        format!("{:.1} KB/s", bytes_per_sec / 1024.0)
    } else {
        format!("{:.1} MB/s", bytes_per_sec / (1024.0 * 1024.0))
    }
}
