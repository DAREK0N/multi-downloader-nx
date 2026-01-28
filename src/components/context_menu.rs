use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MenuItem {
    pub label: String,
    pub disabled: bool,
}

#[component]
pub fn ContextMenu(items: Vec<MenuItem>, on_select: Option<EventHandler<String>>) -> Element {
    rsx! {
        ul { class: "min-w-[160px] rounded-md border border-border bg-surface shadow-md py-1 text-sm",
            for item in items {
                {
                    let disabled = item.disabled;
                    let label = item.label.clone();
                    rsx! {
                        li {
                            class: format!(
                                "px-3 py-2 cursor-pointer transition-colors {}",
                                if disabled { "text-text-muted cursor-not-allowed" } else { "hover:bg-background" }
                            ),
                            onclick: move |_| {
                                if disabled {
                                    return;
                                }
                                if let Some(handler) = &on_select {
                                    handler.call(label.clone());
                                }
                            },
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}
