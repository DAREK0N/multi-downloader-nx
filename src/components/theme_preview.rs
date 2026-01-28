use dioxus::prelude::*;
use crate::models::Theme;
use crate::state::ThemeState;

#[component]
pub fn ThemePreview(theme: Theme) -> Element {
    let current_theme = use_context::<Signal<ThemeState>>();
    let is_active = current_theme.read().current_theme.read().clone() == theme;

    rsx! {
        div {
            class: format!(
                "p-3 rounded-lg border transition-colors {}",
                if is_active { "border-accent bg-accent/10" } else { "border-border bg-surface" }
            ),
            div { class: "flex items-center justify-between mb-2",
                span { class: "font-semibold", "{theme.display_name()}" }
                if is_active {
                    span { class: "text-xs px-2 py-1 rounded bg-accent text-white", "Active" }
                }
            }
            div { class: "grid grid-cols-3 gap-2",
                // Primary swatch
                ColorSwatch { label: "Primary", class: "bg-primary" }
                ColorSwatch { label: "Surface", class: "bg-surface" }
                ColorSwatch { label: "Accent", class: "bg-accent" }
                ColorSwatch { label: "Text", class: "bg-text-primary" }
                ColorSwatch { label: "Muted", class: "bg-text-muted" }
                ColorSwatch { label: "Border", class: "bg-border" }
            }
        }
    }
}

#[component]
fn ColorSwatch(label: &'static str, class: &'static str) -> Element {
    rsx! {
        div { class: "flex flex-col items-start space-y-1",
            div { class: "h-8 w-full rounded {class} border border-border" }
            span { class: "text-xs text-text-secondary", "{label}" }
        }
    }
}
