use dioxus::prelude::*;

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "space-y-6",
            h1 { class: "text-3xl font-bold text-gray-900 dark:text-white",
                "Multi-Downloader NX"
            }
            
            p { class: "text-gray-600 dark:text-gray-300",
                "Download anime from Crunchyroll, Hidive, and AnimationDigitalNetwork"
            }
            
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                // Quick stats cards
                div { class: "bg-white dark:bg-gray-800 rounded-lg shadow p-6",
                    h3 { class: "text-lg font-semibold text-gray-900 dark:text-white mb-2",
                        "Active Downloads"
                    }
                    p { class: "text-3xl font-bold text-blue-600 dark:text-blue-400",
                        "0"
                    }
                }
                
                div { class: "bg-white dark:bg-gray-800 rounded-lg shadow p-6",
                    h3 { class: "text-lg font-semibold text-gray-900 dark:text-white mb-2",
                        "Scheduled"
                    }
                    p { class: "text-3xl font-bold text-purple-600 dark:text-purple-400",
                        "0"
                    }
                }
                
                div { class: "bg-white dark:bg-gray-800 rounded-lg shadow p-6",
                    h3 { class: "text-lg font-semibold text-gray-900 dark:text-white mb-2",
                        "Completed"
                    }
                    p { class: "text-3xl font-bold text-green-600 dark:text-green-400",
                        "0"
                    }
                }
            }
        }
    }
}
