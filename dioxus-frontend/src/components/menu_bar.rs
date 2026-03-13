use crate::models::ServiceType;
use dioxus::prelude::*;

/// Top menu bar component
#[component]
pub fn MenuBar(
    service: ServiceType,
    version: String,
    on_change_service: EventHandler<()>,
) -> Element {
    let mut show_settings = use_signal(|| false);
    let mut show_about = use_signal(|| false);

    rsx! {
        header { class: "menu-bar",
            div { class: "menu-bar-left",
                div { class: "menu-bar-brand",
                    span { class: "brand-icon", "📥" }
                    span { class: "brand-name", "Multi DL NX" }
                }
                div { class: "menu-bar-service",
                    span { class: "service-badge",
                        style: "background: {service_color(&service)}",
                        "{service.icon()} {service.label()}"
                    }
                }
            }
            div { class: "menu-bar-right",
                button {
                    class: "menu-btn",
                    title: "Settings",
                    onclick: move |_| show_settings.set(!show_settings()),
                    "⚙️"
                }
                button {
                    class: "menu-btn",
                    title: "About",
                    onclick: move |_| show_about.set(!show_about()),
                    "ℹ️"
                }
                button {
                    class: "menu-btn menu-btn-switch",
                    title: "Switch Service",
                    onclick: move |_| on_change_service.call(()),
                    "🔄"
                }
            }
        }

        // Settings Modal
        if *show_settings.read() {
            div { class: "modal-overlay",
                onclick: move |_| show_settings.set(false),
                div {
                    class: "modal-content settings-modal",
                    onclick: move |e| e.stop_propagation(),
                    div { class: "modal-header",
                        h2 { "⚙️ Settings" }
                        button {
                            class: "modal-close",
                            onclick: move |_| show_settings.set(false),
                            "✕"
                        }
                    }
                    div { class: "settings-list",
                        SettingsItem { icon: "📁", label: "Open Content Folder", shortcut: "" }
                        SettingsItem { icon: "⚙️", label: "Open Config Folder", shortcut: "" }
                        SettingsItem { icon: "🔧", label: "Binary Paths", shortcut: "" }
                        SettingsItem { icon: "📋", label: "CLI Defaults", shortcut: "" }
                    }
                }
            }
        }

        // About Modal
        if *show_about.read() {
            div { class: "modal-overlay",
                onclick: move |_| show_about.set(false),
                div {
                    class: "modal-content about-modal",
                    onclick: move |e| e.stop_propagation(),
                    div { class: "modal-header",
                        h2 { "About" }
                        button {
                            class: "modal-close",
                            onclick: move |_| show_about.set(false),
                            "✕"
                        }
                    }
                    div { class: "about-content",
                        h3 { "Multi Downloader NX" }
                        p { class: "about-version", "Version {version}" }
                        p { "A multi-service anime downloader built with Rust and Dioxus." }
                        div { class: "about-links",
                            a {
                                href: "https://github.com/anidl/multi-downloader-nx",
                                target: "_blank",
                                class: "about-link",
                                "📦 GitHub Repository"
                            }
                            a {
                                href: "https://github.com/anidl/multi-downloader-nx/issues",
                                target: "_blank",
                                class: "about-link",
                                "🐛 Report Bug"
                            }
                        }
                        p { class: "about-license", "MIT License" }
                    }
                }
            }
        }
    }
}

#[component]
fn SettingsItem(icon: &'static str, label: &'static str, shortcut: &'static str) -> Element {
    rsx! {
        button { class: "settings-item",
            span { class: "settings-item-icon", "{icon}" }
            span { class: "settings-item-label", "{label}" }
            if !shortcut.is_empty() {
                span { class: "settings-item-shortcut", "{shortcut}" }
            }
        }
    }
}

fn service_color(service: &ServiceType) -> &'static str {
    match service {
        ServiceType::Crunchyroll => "#f47521",
        ServiceType::Hidive => "#00bfff",
        ServiceType::Adn => "#6c5ce7",
    }
}
