use dioxus::prelude::*;
use crate::state::theme_state::{ThemeMode, ThemeState};

/// Theme switcher component
#[component]
pub fn ThemeSwitcher() -> Element {
    let mut show_dropdown = use_signal(|| false);
    let mut theme_state = use_context::<Signal<ThemeState>>();
    
    let current_mode = theme_state().mode;
    let theme_name = match current_mode {
        ThemeMode::Light => "Light",
        ThemeMode::Dark => "Dark",
        ThemeMode::OledBlack => "OLED Black",
        ThemeMode::SolarizedLight => "Solarized Light",
        ThemeMode::SolarizedDark => "Solarized Dark",
        ThemeMode::Dracula => "Dracula",
        ThemeMode::Custom(_) => "Custom",
    };

    rsx! {
        div { class: "relative inline-block",
            button {
                class: "px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-50 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500",
                onclick: move |_| show_dropdown.set(!show_dropdown()),
                "🎨 {theme_name}"
            }
            
            if show_dropdown() {
                div { class: "absolute right-0 mt-2 w-48 bg-white dark:bg-gray-800 rounded-md shadow-lg z-10",
                    button {
                        class: "block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
                        onclick: move |_| {
                            theme_state.write().mode = ThemeMode::Light;
                            show_dropdown.set(false);
                        },
                        "☀️ Light"
                    }
                    button {
                        class: "block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
                        onclick: move |_| {
                            theme_state.write().mode = ThemeMode::Dark;
                            show_dropdown.set(false);
                        },
                        "🌙 Dark"
                    }
                    button {
                        class: "block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
                        onclick: move |_| {
                            theme_state.write().mode = ThemeMode::OledBlack;
                            show_dropdown.set(false);
                        },
                        "⚫ OLED Black"
                    }
                    button {
                        class: "block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
                        onclick: move |_| {
                            theme_state.write().mode = ThemeMode::SolarizedLight;
                            show_dropdown.set(false);
                        },
                        "☀️ Solarized Light"
                    }
                    button {
                        class: "block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
                        onclick: move |_| {
                            theme_state.write().mode = ThemeMode::SolarizedDark;
                            show_dropdown.set(false);
                        },
                        "🌙 Solarized Dark"
                    }
                    button {
                        class: "block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
                        onclick: move |_| {
                            theme_state.write().mode = ThemeMode::Dracula;
                            show_dropdown.set(false);
                        },
                        "🧛 Dracula"
                    }
                }
            }
        }
    }
}
