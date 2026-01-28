use dioxus::prelude::*;
use crate::models::Download;
use crate::components::{Card, ProgressBar, DownloadStatusBadge, IconButton, ButtonVariant};

#[component]
pub fn DownloadCard(
    download: Download,
    on_pause: Option<EventHandler<()>>,
    on_resume: Option<EventHandler<()>>,
    on_cancel: Option<EventHandler<()>>,
) -> Element {
    let is_active = download.is_active();
    let is_complete = download.is_complete();
    
    rsx! {
        Card {
            div { class: "space-y-3",
                // Header with title and status
                div { class: "flex items-start justify-between",
                    div { class: "flex-1 min-w-0",
                        h4 { class: "text-lg font-semibold text-text-primary truncate",
                            {download.title.clone()}
                        }
                        p { class: "text-sm text-text-secondary truncate mt-1",
                            {download.service.clone()}
                        }
                    }

                    DownloadStatusBadge { status: download.status.clone() }
                }

                // Thumbnail (if available)
                if let Some(thumbnail) = &download.thumbnail {
                    img {
                        src: "{thumbnail}",
                        alt: "Thumbnail",
                        class: "w-full h-32 object-cover rounded",
                    }
                }

                // Progress bar
                if !is_complete {
                    ProgressBar {
                        value: download.progress,
                        max: 100.0,
                        show_label: true,
                        label: format!(
                            "{} / {}",
                            format_bytes(download.downloaded),
                            download.size.map(format_bytes).unwrap_or("Unknown".to_string()),
                        ),
                    }
                }

                // Metadata row
                div { class: "flex items-center justify-between text-sm text-text-secondary",
                    div { class: "flex items-center space-x-4",
                        if let Some(quality) = &download.quality {
                            span { class: "font-medium", "{quality}" }
                        }

                        if let Some(speed) = download.speed {
                            span { "{format_speed(speed)}/s" }
                        }
                    }

                    if let Some(size) = download.size {
                        span { {format_bytes(size)} }
                    }
                }

                // Action buttons
                div { class: "flex items-center space-x-2 pt-2 border-t border-border",
                    if is_active {
                        IconButton {
                            variant: ButtonVariant::Ghost,
                            onclick: move |_| {
                                if let Some(handler) = &on_pause {
                                    handler.call(());
                                }
                            },
                            aria_label: "Pause download".to_string(),

                            svg {
                                class: "h-5 w-5",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M10 9v6m4-6v6m7-3a9 9 0 11-18 0 9 9 0 0118 0z",
                                }
                            }
                        }
                    } else if !is_complete {
                        IconButton {
                            variant: ButtonVariant::Primary,
                            onclick: move |_| {
                                if let Some(handler) = &on_resume {
                                    handler.call(());
                                }
                            },
                            aria_label: "Resume download".to_string(),

                            svg {
                                class: "h-5 w-5",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                                }
                            }
                        }
                    }

                    if !is_complete {
                        IconButton {
                            variant: ButtonVariant::Danger,
                            onclick: move |_| {
                                if let Some(handler) = &on_cancel {
                                    handler.call(());
                                }
                            },
                            aria_label: "Cancel download".to_string(),

                            svg {
                                class: "h-5 w-5",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
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
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn format_speed(bytes_per_sec: f64) -> String {
    format_bytes(bytes_per_sec as u64)
}
