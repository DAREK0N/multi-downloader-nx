use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
}

#[component]
pub fn MultiSelect(
    options: Vec<SelectOption>,
    #[props(default = Vec::new())] selected: Vec<String>,
    on_change: Option<EventHandler<Vec<String>>>,
) -> Element {
    let mut current = use_signal(|| selected);

    rsx! {
        div { class: "space-y-2",
            for option in options {
                {
                    let value = option.value.clone();
                    let label = option.label.clone();

                    rsx! {
                        label { class: "flex items-center space-x-2 cursor-pointer",
                            input {
                                r#type: "checkbox",
                                checked: "{current().contains(&value)}",
                                onchange: move |_| {
                                    let mut next = current();
                                    if let Some(pos) = next.iter().position(|v| v == &value) {
                                        next.remove(pos);
                                    } else {
                                        next.push(value.clone());
                                    }
                                    current.set(next.clone());
                                    if let Some(handler) = &on_change {
                                        handler.call(next);
                                    }
                                }
                            }
                            span { "{label}" }
                        }
                    }
                }
            }
        }
    }
}
