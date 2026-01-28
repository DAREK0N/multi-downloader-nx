use dioxus::prelude::*;
use crate::components::{Button, ButtonVariant, Card, CardHeader, CardBody, Input, Modal, ModalFooter};
use crate::models::{WebhookConfig, WebhookTrigger};

#[component]
pub fn Webhooks() -> Element {
    let mut show_add_modal = use_signal(|| false);
    let mut webhook_url = use_signal(|| String::new());
    let mut webhook_name = use_signal(|| String::new());
    let mut selected_triggers = use_signal(|| Vec::<WebhookTrigger>::new());
    
    // Mock data - will be replaced with real state later
    let webhooks = use_signal(|| Vec::<WebhookConfig>::new());
    
    let all_triggers = use_memo(move || vec![
        WebhookTrigger::DownloadStarted,
        WebhookTrigger::DownloadCompleted,
        WebhookTrigger::DownloadFailed,
        WebhookTrigger::QueueCompleted,
    ]);
    
    rsx! {
        div { class: "space-y-6",
            // Header
            div { class: "flex items-center justify-between",
                div {
                    h1 { class: "text-3xl font-bold mb-2", "Discord Webhooks" }
                    p { class: "text-content-secondary",
                        "Configure Discord webhook notifications for download events."
                    }
                }
                Button { onclick: move |_| show_add_modal.set(true), "Add Webhook" }
            }

            // Webhooks List
            if webhooks().is_empty() {
                Card {
                    div { class: "text-center py-12",
                        p { class: "text-content-secondary mb-4", "No webhooks configured" }
                        Button { onclick: move |_| show_add_modal.set(true), "Add Your First Webhook" }
                    }
                }
            } else {
                div { class: "space-y-3",
                    for webhook in webhooks() {
                        Card {
                            CardHeader {
                                h3 { class: "text-lg font-semibold", "{webhook.name}" }
                            }
                            CardBody {
                                div { class: "space-y-3",
                                    div {
                                        p { class: "text-sm text-content-secondary mb-1",
                                            "Webhook URL"
                                        }
                                        p { class: "text-sm font-mono bg-surface-elevated px-3 py-2 rounded",
                                            "{webhook.url}"
                                        }
                                    }

                                    div {
                                        p { class: "text-sm text-content-secondary mb-2",
                                            "Triggers"
                                        }
                                        div { class: "flex flex-wrap gap-2",
                                            for trigger in &webhook.triggers {
                                                span { class: "px-3 py-1 bg-primary/20 text-primary rounded-full text-sm",
                                                    "{trigger}"
                                                }
                                            }
                                        }
                                    }

                                    div { class: "flex items-center space-x-2 pt-2",
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            onclick: move |_| {
                                                // TODO: Test webhook
                                            },
                                            "Test Webhook"
                                        }
                                        Button {
                                            variant: ButtonVariant::Danger,
                                            onclick: move |_| {
                                                // TODO: Delete webhook
                                            },
                                            "Delete"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Add Webhook Modal
            if show_add_modal() {
                Modal {
                    show: show_add_modal,
                    title: "Add Discord Webhook".to_string(),

                    div { class: "space-y-4",
                        Input {
                            value: webhook_name,
                            label: "Webhook Name".to_string(),
                            placeholder: "My Webhook".to_string(),
                        }
                        Input {
                            value: webhook_url,
                            label: "Discord Webhook URL".to_string(),
                            placeholder: "https://discord.com/api/webhooks/...".to_string(),
                        }

                        div {
                            label { class: "block text-sm font-medium mb-2", "Triggers" }
                            div { class: "space-y-2",
                                {
                                    all_triggers()
                                        .into_iter()
                                        .map(|trigger| {
                                            let trigger_clone = trigger.clone();
                                            rsx! {
                                                label {
                                                    key: "{trigger_clone}",
                                                    class: "flex items-center space-x-2 cursor-pointer",
                                                    input {
                                                        r#type: "checkbox",
                                                        class: "rounded border-border",
                                                        checked: selected_triggers().contains(&trigger_clone),
                                                        onchange: move |e| {
                                                            let tc = trigger_clone.clone();
                                                            if e.checked() {
                                                                selected_triggers.write().push(tc);
                                                            } else {
                                                                selected_triggers.write().retain(|t| t != &tc);
                                                            }
                                                        },
                                                    }
                                                    span { "{trigger_clone}" }
                                                }
                                            }
                                        })
                                }
                            }
                        }
                    }

                    ModalFooter {
                        Button {
                            variant: ButtonVariant::Ghost,
                            onclick: move |_| {
                                show_add_modal.set(false);
                                webhook_url.set(String::new());
                                webhook_name.set(String::new());
                                selected_triggers.set(Vec::new());
                            },
                            "Cancel"
                        }
                        Button {
                            onclick: move |_| {
                                // TODO: Add webhook logic
                                show_add_modal.set(false);
                                webhook_url.set(String::new());
                                webhook_name.set(String::new());
                                selected_triggers.set(Vec::new());
                            },
                            "Add Webhook"
                        }
                    }
                }
            }
        }
    }
}
