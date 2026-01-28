use dioxus::prelude::*;
use crate::models::Download;
use crate::components::DownloadCard;

#[derive(Clone, Copy, PartialEq)]
pub enum DownloadFilter {
    All,
    Downloading,
    Completed,
    Failed,
}

#[component]
pub fn DownloadList(
    downloads: Vec<Download>,
    #[props(default = DownloadFilter::All)] filter: DownloadFilter,
    on_pause: Option<EventHandler<uuid::Uuid>>,
    on_resume: Option<EventHandler<uuid::Uuid>>,
    on_cancel: Option<EventHandler<uuid::Uuid>>,
) -> Element {
    let filtered_downloads: Vec<Download> = downloads
        .into_iter()
        .filter(|d| match filter {
            DownloadFilter::All => true,
            DownloadFilter::Downloading => d.is_active(),
            DownloadFilter::Completed => d.is_complete(),
            DownloadFilter::Failed => d.is_failed(),
        })
        .collect();
    
    if filtered_downloads.is_empty() {
        return rsx! {
            div { class: "text-center py-12",
                svg {
                    class: "mx-auto h-12 w-12 text-text-secondary",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10",
                    }
                }
                p { class: "mt-2 text-sm text-text-secondary", "No downloads" }
            }
        };
    }
    
    rsx! {
        div { class: "grid gap-4 md:grid-cols-2 lg:grid-cols-3",
            for download in filtered_downloads {
                DownloadCard {
                    key: "{download.id}",
                    download: download.clone(),
                    on_pause: move |_| {
                        if let Some(handler) = &on_pause {
                            handler.call(download.id);
                        }
                    },
                    on_resume: move |_| {
                        if let Some(handler) = &on_resume {
                            handler.call(download.id);
                        }
                    },
                    on_cancel: move |_| {
                        if let Some(handler) = &on_cancel {
                            handler.call(download.id);
                        }
                    },
                }
            }
        }
    }
}
