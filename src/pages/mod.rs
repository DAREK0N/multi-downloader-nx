//! Page components

// Re-export Home from views for compatibility
pub use crate::views::Home;

// Additional pages - inline implementations for Phase 1
pub mod downloads {
    use dioxus::prelude::*;
    use crate::components::{DownloadList, DownloadFilter, Button, ButtonVariant, Card, Input};
    use crate::models::Download;
    
    #[component]
    pub fn Downloads() -> Element {
        let mut filter = use_signal(|| DownloadFilter::All);
        let mut search = use_signal(|| String::new());
        
        // Mock data - will be replaced with real state later
        let downloads = use_signal(|| Vec::<Download>::new());
        
        rsx! {
            div { class: "space-y-6",
                // Header
                div { class: "flex items-center justify-between",
                    h1 { class: "text-3xl font-bold", "Downloads" }
                    div { class: "flex items-center space-x-3",
                        Button { variant: ButtonVariant::Secondary, "Clear Completed" }
                        Button { variant: ButtonVariant::Danger, "Cancel All" }
                    }
                }

                // Search and Filter
                Card {
                    div { class: "flex flex-col md:flex-row gap-4",
                        div { class: "flex-1",
                            Input {
                                value: search,
                                placeholder: "Search downloads...".to_string(),
                                label: String::new(),
                            }
                        }

                        div { class: "flex items-end space-x-2",
                            Button {
                                variant: if filter() == DownloadFilter::All { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                                onclick: move |_| filter.set(DownloadFilter::All),
                                "All"
                            }
                            Button {
                                variant: if filter() == DownloadFilter::Downloading { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                                onclick: move |_| filter.set(DownloadFilter::Downloading),
                                "Active"
                            }
                            Button {
                                variant: if filter() == DownloadFilter::Completed { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                                onclick: move |_| filter.set(DownloadFilter::Completed),
                                "Completed"
                            }
                            Button {
                                variant: if filter() == DownloadFilter::Failed { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                                onclick: move |_| filter.set(DownloadFilter::Failed),
                                "Failed"
                            }
                        }
                    }
                }

                // Downloads List
                DownloadList {
                    downloads: downloads().clone(),
                    filter: filter(),
                    on_pause: move |id| {
                        // TODO: Implement pause logic
                        tracing::info!("Pause download: {}", id);
                    },
                    on_resume: move |id| {
                        // TODO: Implement resume logic
                        tracing::info!("Resume download: {}", id);
                    },
                    on_cancel: move |id| {
                        // TODO: Implement cancel logic
                        tracing::info!("Cancel download: {}", id);
                    },
                }
            }
        }
    }
}

pub mod scheduled {
    use dioxus::prelude::*;
    use crate::components::{Button, ButtonVariant, Card, Input, Modal, ModalFooter};
    use crate::models::{ScheduledDownload, Recurrence};
    
    #[component]
    pub fn Scheduled() -> Element {
        let mut show_add_modal = use_signal(|| false);
        let mut url = use_signal(|| String::new());
        let mut name = use_signal(|| String::new());
        let mut schedule_time = use_signal(|| String::new());
        
        // Mock data - will be replaced with real state later
        let schedules = use_signal(|| Vec::<ScheduledDownload>::new());
        
        rsx! {
            div { class: "space-y-6",
                // Header
                div { class: "flex items-center justify-between",
                    h1 { class: "text-3xl font-bold", "Scheduled Downloads" }
                    Button { onclick: move |_| show_add_modal.set(true), "Schedule New Download" }
                }

                // Stats
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                    Card {
                        div { class: "text-center",
                            p { class: "text-sm text-content-secondary mb-1", "Total Scheduled" }
                            p { class: "text-3xl font-bold text-primary", "{schedules().len()}" }
                        }
                    }
                    Card {
                        div { class: "text-center",
                            p { class: "text-sm text-content-secondary mb-1", "Active" }
                            p { class: "text-3xl font-bold text-success",
                                "{schedules().iter().filter(|s| s.enabled).count()}"
                            }
                        }
                    }
                    Card {
                        div { class: "text-center",
                            p { class: "text-sm text-content-secondary mb-1", "Recurring" }
                            p { class: "text-3xl font-bold text-info",
                                "{schedules().iter().filter(|s| !matches!(s.recurrence, Recurrence::Once)).count()}"
                            }
                        }
                    }
                }

                // Scheduled Downloads List
                if schedules().is_empty() {
                    Card {
                        div { class: "text-center py-12",
                            p { class: "text-content-secondary mb-4", "No scheduled downloads yet" }
                            Button { onclick: move |_| show_add_modal.set(true),
                                "Schedule Your First Download"
                            }
                        }
                    }
                } else {
                    div { class: "space-y-3",
                        for schedule in schedules() {
                            Card {
                                div { class: "flex items-center justify-between",
                                    div { class: "flex-1",
                                        h3 { class: "font-semibold text-lg mb-1", "{schedule.title}" }
                                        p { class: "text-sm text-content-secondary mb-2",
                                            "{schedule.url}"
                                        }
                                        div { class: "flex items-center space-x-4 text-sm",
                                            span { class: "text-content-secondary",
                                                "Next run: {schedule.next_run}"
                                            }
                                            span { class: "px-2 py-1 bg-surface-elevated rounded",
                                                "{schedule.recurrence}"
                                            }
                                            if schedule.enabled {
                                                span { class: "px-2 py-1 bg-success/20 text-success rounded",
                                                    "Active"
                                                }
                                            } else {
                                                span { class: "px-2 py-1 bg-content-secondary/20 text-content-secondary rounded",
                                                    "Paused"
                                                }
                                            }
                                        }
                                    }
                                    div { class: "flex items-center space-x-2",
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            onclick: move |_| {}, // TODO: Toggle enabled
                                            if schedule.enabled {
                                                "Pause"
                                            } else {
                                                "Resume"
                                            }
                                        }
                                        Button {
                                            variant: ButtonVariant::Danger,
                                            onclick: move |_| {}, // TODO: Delete schedule
                                            "Delete"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Add Schedule Modal
                if show_add_modal() {
                    Modal {
                        show: show_add_modal,
                        title: "Schedule New Download".to_string(),

                        div { class: "space-y-4",
                            Input {
                                value: name,
                                label: "Download Name".to_string(),
                                placeholder: "My Scheduled Download".to_string(),
                            }
                            Input {
                                value: url,
                                label: "URL".to_string(),
                                placeholder: "https://example.com/video".to_string(),
                            }
                            Input {
                                value: schedule_time,
                                label: "Schedule Time".to_string(),
                                placeholder: "YYYY-MM-DD HH:MM".to_string(),
                            }
                        }

                        ModalFooter {
                            Button {
                                variant: ButtonVariant::Ghost,
                                onclick: move |_| {
                                    show_add_modal.set(false);
                                    url.set(String::new());
                                    name.set(String::new());
                                    schedule_time.set(String::new());
                                },
                                "Cancel"
                            }
                            Button {
                                onclick: move |_| {
                                    // TODO: Add schedule logic
                                    show_add_modal.set(false);
                                    url.set(String::new());
                                    name.set(String::new());
                                    schedule_time.set(String::new());
                                },
                                "Schedule Download"
                            }
                        }
                    }
                }
            }
        }
    }
}

pub mod settings {
    use dioxus::prelude::*;
    use crate::components::{Button, ButtonVariant, Card, CardHeader, CardBody, Input, ThemeSwitcher};
    use crate::models::ServiceType;
    use crate::state::AuthState;
    
    #[component]
    pub fn Settings() -> Element {
        let mut auth_state = use_context::<Signal<AuthState>>();
        
        let mut download_dir = use_signal(|| String::from("./downloads"));
        let mut temp_dir = use_signal(|| String::from("./temp"));
        let mut concurrent_downloads = use_signal(|| String::from("3"));
        let mut max_retries = use_signal(|| String::from("3"));
        
        // Service credentials
        let mut cr_username = use_signal(|| String::new());
        let mut cr_password = use_signal(|| String::new());
        let mut hidive_email = use_signal(|| String::new());
        let mut hidive_password = use_signal(|| String::new());
        let mut adn_username = use_signal(|| String::new());
        let mut adn_password = use_signal(|| String::new());
        
        rsx! {
            div { class: "space-y-6",
                h1 { class: "text-3xl font-bold mb-6", "Settings" }

                // Theme Settings
                Card {
                    CardHeader {
                        h3 { class: "text-lg font-semibold", "Appearance" }
                    }
                    CardBody {
                        div { class: "space-y-4",
                            div {
                                label { class: "block text-sm font-medium mb-2", "Theme" }
                                ThemeSwitcher {}
                            }
                        }
                    }
                }

                // Download Settings
                Card {
                    CardHeader {
                        h3 { class: "text-lg font-semibold", "Download Settings" }
                    }
                    CardBody {
                        div { class: "space-y-4",
                            Input {
                                value: download_dir,
                                label: "Download Directory".to_string(),
                                placeholder: "./downloads".to_string(),
                            }
                            Input {
                                value: temp_dir,
                                label: "Temporary Directory".to_string(),
                                placeholder: "./temp".to_string(),
                            }
                            Input {
                                value: concurrent_downloads,
                                label: "Concurrent Downloads".to_string(),
                                placeholder: "3".to_string(),
                            }
                            Input {
                                value: max_retries,
                                label: "Max Retries".to_string(),
                                placeholder: "3".to_string(),
                            }
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        onclick: move |_| {}, // TODO: Save download settings
                                        "Save Download Settings"
                                    }
                        }
                    }
                }

                // Service Authentication
                Card {
                    CardHeader {
                        h3 { class: "text-lg font-semibold", "Service Authentication" }
                    }
                    CardBody {
                        div { class: "space-y-6",
                            // Crunchyroll
                            div { class: "space-y-3",
                                h3 { class: "font-semibold flex items-center justify-between",
                                    span { "Crunchyroll" }
                                    if auth_state().is_authenticated(ServiceType::Crunchyroll) {
                                        span { class: "text-sm text-success", "✓ Connected" }
                                    } else {
                                        span { class: "text-sm text-content-secondary",
                                            "Not connected"
                                        }
                                    }
                                }
                                Input {
                                    value: cr_username,
                                    label: "Username/Email".to_string(),
                                    placeholder: "username".to_string(),
                                }
                                Input {
                                    value: cr_password,
                                    label: "Password".to_string(),
                                    placeholder: "password".to_string(),
                                }
                                div { class: "flex space-x-2",
                                    Button { onclick: move |_| {}, "Connect" } // TODO: Login to Crunchyroll
                                    if auth_state().is_authenticated(ServiceType::Crunchyroll) {
                                        Button {
                                            variant: ButtonVariant::Danger,
                                            onclick: move |_| {
                                                auth_state.write().logout(ServiceType::Crunchyroll);
                                            },
                                            "Disconnect"
                                        }
                                    }
                                }
                            }

                            // HIDIVE
                            div { class: "space-y-3",
                                h3 { class: "font-semibold flex items-center justify-between",
                                    span { "HIDIVE" }
                                    if auth_state().is_authenticated(ServiceType::Hidive) {
                                        span { class: "text-sm text-success", "✓ Connected" }
                                    } else {
                                        span { class: "text-sm text-content-secondary",
                                            "Not connected"
                                        }
                                    }
                                }
                                Input {
                                    value: hidive_email,
                                    label: "Email".to_string(),
                                    placeholder: "email@example.com".to_string(),
                                }
                                Input {
                                    value: hidive_password,
                                    label: "Password".to_string(),
                                    placeholder: "password".to_string(),
                                }
                                div { class: "flex space-x-2",
                                    Button { onclick: move |_| {}, "Connect" } // TODO: Login to HIDIVE
                                    if auth_state().is_authenticated(ServiceType::Hidive) {
                                        Button {
                                            variant: ButtonVariant::Danger,
                                            onclick: move |_| {
                                                auth_state.write().logout(ServiceType::Hidive);
                                            },
                                            "Disconnect"
                                        }
                                    }
                                }
                            }

                            // ADN
                            div { class: "space-y-3",
                                h3 { class: "font-semibold flex items-center justify-between",
                                    span { "Animation Digital Network" }
                                    if auth_state().is_authenticated(ServiceType::Adn) {
                                        span { class: "text-sm text-success", "✓ Connected" }
                                    } else {
                                        span { class: "text-sm text-content-secondary",
                                            "Not connected"
                                        }
                                    }
                                }
                                Input {
                                    value: adn_username,
                                    label: "Username".to_string(),
                                    placeholder: "username".to_string(),
                                }
                                Input {
                                    value: adn_password,
                                    label: "Password".to_string(),
                                    placeholder: "password".to_string(),
                                }
                                div { class: "flex space-x-2",
                                    Button { onclick: move |_| {}, "Connect" } // TODO: Login to ADN
                                    if auth_state().is_authenticated(ServiceType::Adn) {
                                        Button {
                                            variant: ButtonVariant::Danger,
                                            onclick: move |_| {
                                                auth_state.write().logout(ServiceType::Adn);
                                            },
                                            "Disconnect"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub mod webhooks;
