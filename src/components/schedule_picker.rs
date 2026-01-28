use dioxus::prelude::*;

/// Schedule picker component
#[component]
pub fn SchedulePicker() -> Element {
    rsx! {
        div { class: "inline-block",
            "Schedule picker"
        }
    }
}
