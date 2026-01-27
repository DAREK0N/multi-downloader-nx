use dioxus::prelude::*;

/// Theme switcher component
#[component]
pub fn ThemeSwitcher() -> Element {
    rsx! {
        div { class: "inline-block",
            "Theme switcher"
        }
    }
}
