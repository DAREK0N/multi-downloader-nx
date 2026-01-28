use dioxus::prelude::*;
use crate::models::Theme;
use crate::hooks::use_theme::use_theme;

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut theme_state = use_theme();
    let mut show_dropdown = use_signal(|| false);
    
    let current_theme = theme_state.read().current_theme.read().clone();
    
    rsx! {
        div { class: "relative",
            // Theme button
            button {
                r#type: "button",
                class: "flex items-center space-x-2 px-3 py-2 rounded-md text-sm font-medium transition-colors text-text-secondary hover:text-text-primary hover:bg-background",
                onclick: move |_| show_dropdown.set(!show_dropdown()),

                // Theme icon
                svg {
                    class: "h-5 w-5",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",

                    if matches!(
                        current_theme,
                        Theme::Dark | Theme::OledBlack | Theme::SolarizedDark | Theme::Dracula
                    )
                    {
                        // Moon icon for dark themes
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z",
                        }
                    } else {
                        // Sun icon for light themes
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z",
                        }
                    }
                }

                span { class: "hidden sm:inline", "{current_theme.display_name()}" }

                // Dropdown arrow
                svg {
                    class: "h-4 w-4 ml-1",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M19 9l-7 7-7-7",
                    }
                }
            }

            // Dropdown menu
            if show_dropdown() {
                div { class: "absolute right-0 mt-2 w-72 rounded-md shadow-lg bg-surface border border-border z-50",
                    div { class: "p-2 space-y-2",
                        for theme in Theme::all() {
                            ThemePreviewCard {
                                theme,
                                is_active: current_theme == theme,
                                on_select: move |selected_theme| {
                                    theme_state.write().set_theme(selected_theme);
                                    show_dropdown.set(false);
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ThemePreviewCard(theme: Theme, is_active: bool, on_select: EventHandler<Theme>) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "w-full text-left p-3 rounded-md border transition-colors",
            class: if is_active { "border-accent bg-accent/10" } else { "border-border hover:bg-background" },
            onclick: move |_| on_select.call(theme),

            div { class: "flex items-center justify-between mb-2",
                span { class: "font-semibold", "{theme.display_name()}" }
                if is_active {
                    span { class: "text-xs px-2 py-1 rounded bg-accent text-white", "Active" }
                }
            }

            // Mini palette preview
            div { class: "grid grid-cols-6 gap-1",
                div { class: "h-3 rounded bg-primary" }
                div { class: "h-3 rounded bg-surface" }
                div { class: "h-3 rounded bg-accent" }
                div { class: "h-3 rounded bg-text-primary" }
                div { class: "h-3 rounded bg-text-muted" }
                div { class: "h-3 rounded bg-border" }
            }
        }
    }
}
