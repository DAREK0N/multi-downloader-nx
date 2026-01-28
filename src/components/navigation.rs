use dioxus::prelude::*;
use crate::router::Route;
use crate::components::{theme_switcher::ThemeSwitcher, language_selector::LanguageSelector};

/// Navigation component
#[component]
pub fn Navigation() -> Element {
    let mut show_mobile_menu = use_signal(|| false);

    rsx! {
        nav { class: "bg-white dark:bg-gray-800 shadow-lg",
            div { class: "container mx-auto px-4",
                div { class: "flex justify-between items-center h-16",
                    // Logo
                    div { class: "flex items-center",
                        Link { 
                            to: Route::Home {},
                            class: "text-xl font-bold text-gray-900 dark:text-white",
                            "Multi-Downloader NX"
                        }
                    }
                    
                    // Desktop navigation
                    div { class: "hidden md:flex items-center space-x-4",
                        Link { 
                            to: Route::Home {},
                            class: "text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 px-3 py-2 rounded-md",
                            "Home"
                        }
                        Link { 
                            to: Route::Downloads {},
                            class: "text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 px-3 py-2 rounded-md",
                            "Downloads"
                        }
                        Link { 
                            to: Route::Scheduled {},
                            class: "text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 px-3 py-2 rounded-md",
                            "Scheduled"
                        }
                        Link { 
                            to: Route::Webhooks {},
                            class: "text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 px-3 py-2 rounded-md",
                            "Webhooks"
                        }
                        Link { 
                            to: Route::Settings {},
                            class: "text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 px-3 py-2 rounded-md",
                            "Settings"
                        }
                        
                        // Theme and Language Selectors
                        div { class: "flex items-center space-x-2 ml-4",
                            ThemeSwitcher {}
                            LanguageSelector {}
                        }
                    }
                    
                    // Mobile menu button
                    button {
                        class: "md:hidden text-gray-700 dark:text-gray-300 text-2xl",
                        onclick: move |_| show_mobile_menu.set(!show_mobile_menu()),
                        "☰"
                    }
                }
                
                // Mobile navigation
                if show_mobile_menu() {
                    div { class: "md:hidden pb-4",
                        div { class: "flex flex-col space-y-2",
                            Link { 
                                to: Route::Home {},
                                class: "text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 px-3 py-2 rounded-md",
                                onclick: move |_| show_mobile_menu.set(false),
                                "Home"
                            }
                            Link { 
                                to: Route::Downloads {},
                                class: "text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 px-3 py-2 rounded-md",
                                onclick: move |_| show_mobile_menu.set(false),
                                "Downloads"
                            }
                            Link { 
                                to: Route::Scheduled {},
                                class: "text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 px-3 py-2 rounded-md",
                                onclick: move |_| show_mobile_menu.set(false),
                                "Scheduled"
                            }
                            Link { 
                                to: Route::Webhooks {},
                                class: "text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 px-3 py-2 rounded-md",
                                onclick: move |_| show_mobile_menu.set(false),
                                "Webhooks"
                            }
                            Link { 
                                to: Route::Settings {},
                                class: "text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 px-3 py-2 rounded-md",
                                onclick: move |_| show_mobile_menu.set(false),
                                "Settings"
                            }
                            
                            // Mobile theme and language selectors
                            div { class: "flex flex-col space-y-2 pt-2 border-t border-gray-300 dark:border-gray-600",
                                ThemeSwitcher {}
                                LanguageSelector {}
                            }
                        }
                    }
                }
            }
        }
    }
}
