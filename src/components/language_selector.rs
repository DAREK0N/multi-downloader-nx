use dioxus::prelude::*;

/// Language selector component
#[component]
pub fn LanguageSelector() -> Element {
    rsx! {
        div { class: "inline-block",
            "Language selector"
        }
    }
}
