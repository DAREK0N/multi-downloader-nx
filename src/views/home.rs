use dioxus::prelude::*;
use crate::components::{Button, ButtonVariant, Card, Input, Modal};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let mut url_input = use_signal(|| String::new());
    let mut show_add_modal = use_signal(|| false);
    
    rsx! {
        div { class: "space-y-6",
            // Hero Section
            div { class: "text-center py-12 bg-gradient-to-r from-accent to-accent-dark rounded-lg shadow-lg",
                h1 { class: "text-4xl font-bold text-white mb-4", "Multi-Downloader NX" }
                p { class: "text-xl text-white/90 mb-8",
                    "Download anime from Crunchyroll, HIDIVE, and ADN"
                }

                Button {
                    variant: ButtonVariant::Secondary,
                    onclick: move |_| show_add_modal.set(true),
                    "Add New Download"
                }
            }

            // Quick Stats
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                Card { title: "Active Downloads".to_string(),
                    div { class: "text-center",
                        div { class: "text-4xl font-bold text-accent mb-2", "0" }
                        p { class: "text-text-secondary", "Currently downloading" }
                    }
                }

                Card { title: "Queue".to_string(),
                    div { class: "text-center",
                        div { class: "text-4xl font-bold text-blue-600 mb-2", "0" }
                        p { class: "text-text-secondary", "Items waiting" }
                    }
                }

                Card { title: "Completed".to_string(),
                    div { class: "text-center",
                        div { class: "text-4xl font-bold text-green-600 mb-2", "0" }
                        p { class: "text-text-secondary", "Successfully downloaded" }
                    }
                }
            }

            // Quick Actions
            Card { title: "Quick Actions".to_string(),
                div { class: "flex flex-wrap gap-3",
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| show_add_modal.set(true),
                        "Add Download"
                    }
                    Button { variant: ButtonVariant::Secondary, "View Queue" }
                    Button { variant: ButtonVariant::Secondary, "View Downloads" }
                }
            }

            // Add Download Modal
            Modal { show: show_add_modal, title: "Add New Download".to_string(),
                div { class: "space-y-4",
                    Input {
                        value: url_input,
                        label: "URL".to_string(),
                        placeholder: "Enter anime URL...".to_string(),
                        r#type: "url".to_string(),
                    }

                    div { class: "flex items-center justify-end space-x-3 pt-4",
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| show_add_modal.set(false),
                            "Cancel"
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            onclick: move |_| {
                                // TODO: Add download logic
                                show_add_modal.set(false);
                                url_input.set(String::new());
                            },
                            "Add to Queue"
                        }
                    }
                }
            }
        }
    }
}
