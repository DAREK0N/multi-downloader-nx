use crate::models::ServiceType;
use dioxus::prelude::*;

/// Service selector component - shown when no service is selected
#[component]
pub fn ServiceSelector(on_select: EventHandler<ServiceType>) -> Element {
    rsx! {
        div { class: "service-selector-overlay",
            div { class: "service-selector-card",
                div { class: "service-selector-header",
                    h1 { class: "service-selector-title", "Multi Downloader NX" }
                    p { class: "service-selector-subtitle", "Select a streaming service to get started" }
                }
                div { class: "service-selector-grid",
                    ServiceOption {
                        service: ServiceType::Crunchyroll,
                        color: "#f47521",
                        on_click: move |_| on_select.call(ServiceType::Crunchyroll),
                    }
                    ServiceOption {
                        service: ServiceType::Hidive,
                        color: "#00bfff",
                        on_click: move |_| on_select.call(ServiceType::Hidive),
                    }
                    ServiceOption {
                        service: ServiceType::Adn,
                        color: "#1a1a2e",
                        on_click: move |_| on_select.call(ServiceType::Adn),
                    }
                }
            }
        }
    }
}

#[component]
fn ServiceOption(service: ServiceType, color: &'static str, on_click: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "service-option",
            style: "border-color: {color}",
            onclick: move |_| on_click.call(()),
            div { class: "service-option-icon",
                span { class: "service-emoji", "{service.icon()}" }
            }
            div { class: "service-option-info",
                h3 { class: "service-option-name", "{service.label()}" }
                p { class: "service-option-desc",
                    {match service {
                        ServiceType::Crunchyroll => "World's largest anime library",
                        ServiceType::Hidive => "Exclusive anime streaming",
                        ServiceType::Adn => "Anime Digital Network (FR)",
                    }}
                }
            }
        }
    }
}
