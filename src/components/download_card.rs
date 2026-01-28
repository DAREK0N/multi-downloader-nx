use dioxus::prelude::*;

/// Download card component
#[component]
pub fn DownloadCard() -> Element {
    rsx! {
        div { class: "bg-white dark:bg-gray-800 rounded-lg shadow p-4",
            "Download card component"
        }
    }
}
