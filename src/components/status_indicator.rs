use dioxus::prelude::*;

/// Status indicator component
#[component]
pub fn StatusIndicator() -> Element {
    rsx! {
        div { class: "inline-block",
            "Status indicator"
        }
    }
}
