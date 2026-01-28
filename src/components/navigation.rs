use dioxus::prelude::*;
use crate::router::Route;

#[component]
pub fn NavBar() -> Element {
    let mut show_mobile_menu = use_signal(|| false);
    
    rsx! {
        nav { class: "bg-surface border-b border-border",
            div { class: "container mx-auto px-4",
                div { class: "flex items-center justify-between h-16",
                    // Logo/Brand
                    div { class: "flex-shrink-0",
                        Link {
                            to: Route::Home {},
                            class: "text-xl font-bold text-accent",
                            "Multi-Downloader NX"
                        }
                    }

                // Desktop Navigation
                div { class: "hidden md:flex md:items-center md:space-x-4",
                    NavLink { to: Route::Home {}, "Home" }
                    NavLink { to: Route::Downloads {}, "Downloads" }
                    NavLink { to: Route::Scheduled {}, "Scheduled" }
                    NavLink { to: Route::Webhooks {}, "Webhooks" }
                    NavLink { to: Route::Settings {}, "Settings" }

                    // Theme switcher
                    crate::components::ThemeSwitcher {}
                }

                    // Mobile menu button
                    div { class: "md:hidden",
                        button {
                            r#type: "button",
                            class: "inline-flex items-center justify-center p-2 rounded-md text-text-secondary hover:text-text-primary hover:bg-background focus:outline-none",
                            onclick: move |_| show_mobile_menu.set(!show_mobile_menu()),
                            "aria-expanded": "{show_mobile_menu}",
                            "aria-label": "Toggle navigation menu",

                            // Hamburger icon
                            svg {
                                class: "h-6 w-6",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                if !show_mobile_menu() {
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M4 6h16M4 12h16M4 18h16",
                                    }
                                } else {
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M6 18L18 6M6 6l12 12",
                                    }
                                }
                            }
                        }
                    }
                }

                // Mobile Navigation Menu
                if show_mobile_menu() {
                    div { class: "md:hidden pb-3 pt-2 space-y-1",
                        MobileNavLink {
                            to: Route::Home {},
                            onclick: move |_| show_mobile_menu.set(false),
                            "Home"
                        }
                        MobileNavLink {
                            to: Route::Downloads {},
                            onclick: move |_| show_mobile_menu.set(false),
                            "Downloads"
                        }
                        MobileNavLink {
                            to: Route::Scheduled {},
                            onclick: move |_| show_mobile_menu.set(false),
                            "Scheduled"
                        }
                        MobileNavLink {
                            to: Route::Webhooks {},
                            onclick: move |_| show_mobile_menu.set(false),
                            "Webhooks"
                        }
                        MobileNavLink {
                            to: Route::Settings {},
                            onclick: move |_| show_mobile_menu.set(false),
                            "Settings"
                        }
                    }
                }
            }
        }
    }
}

/// Desktop navigation link with active state styling
#[component]
fn NavLink(to: Route, children: Element) -> Element {
    rsx! {
        Link {
            to,
            class: "px-3 py-2 rounded-md text-sm font-medium transition-colors text-text-secondary hover:text-text-primary hover:bg-background",
            active_class: "!bg-accent !text-white",
            {children}
        }
    }
}

/// Mobile navigation link
#[component]
fn MobileNavLink(to: Route, onclick: EventHandler<MouseEvent>, children: Element) -> Element {
    rsx! {
        Link {
            to,
            onclick: move |e| onclick.call(e),
            class: "block px-3 py-2 rounded-md text-base font-medium transition-colors text-text-secondary hover:text-text-primary hover:bg-background",
            active_class: "!bg-accent !text-white",
            {children}
        }
    }
}
