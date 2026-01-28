use dioxus::prelude::*;

#[component]
pub fn Input(
    #[props(default = "text".to_string())] r#type: String,
    #[props(default = "".to_string())] placeholder: String,
    #[props(default = "".to_string())] label: String,
    #[props(default = None)] error: Option<String>,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] required: bool,
    value: Signal<String>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let has_error = error.is_some();
    let border_color = if has_error {
        "border-red-500 focus:ring-red-500"
    } else {
        "border-border focus:ring-accent"
    };
    
    rsx! {
        div { class: "w-full",
            if !label.is_empty() {
                label { class: "block text-sm font-medium text-text-primary mb-1",
                    {label.clone()}
                    if required {
                        span { class: "text-red-500 ml-1", "*" }
                    }
                }
            }

            input {
                r#type: "{r#type}",
                class: "w-full px-3 py-2 bg-surface text-text-primary border rounded-md transition-colors focus:outline-none focus:ring-2 {border_color}",
                class: if disabled { "opacity-50 cursor-not-allowed" } else { "" },
                placeholder: "{placeholder}",
                disabled,
                required,
                value: "{value}",
                oninput: move |e| {
                    *value.write() = e.value();
                    if let Some(handler) = &oninput {
                        handler.call(e);
                    }
                },
            }

            if let Some(error_msg) = error {
                p { class: "mt-1 text-sm text-red-500", {error_msg} }
            }
        }
    }
}

#[component]
pub fn TextArea(
    #[props(default = "".to_string())] placeholder: String,
    #[props(default = "".to_string())] label: String,
    #[props(default = None)] error: Option<String>,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] required: bool,
    #[props(default = 4)] rows: i32,
    value: Signal<String>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let has_error = error.is_some();
    let border_color = if has_error {
        "border-red-500 focus:ring-red-500"
    } else {
        "border-border focus:ring-accent"
    };
    
    rsx! {
        div { class: "w-full",
            if !label.is_empty() {
                label { class: "block text-sm font-medium text-text-primary mb-1",
                    {label.clone()}
                    if required {
                        span { class: "text-red-500 ml-1", "*" }
                    }
                }
            }

            textarea {
                class: "w-full px-3 py-2 bg-surface text-text-primary border rounded-md transition-colors focus:outline-none focus:ring-2 resize-y {border_color}",
                class: if disabled { "opacity-50 cursor-not-allowed" } else { "" },
                placeholder: "{placeholder}",
                disabled,
                required,
                rows: "{rows}",
                value: "{value}",
                oninput: move |e| {
                    *value.write() = e.value();
                    if let Some(handler) = &oninput {
                        handler.call(e);
                    }
                },
            }

            if let Some(error_msg) = error {
                p { class: "mt-1 text-sm text-red-500", {error_msg} }
            }
        }
    }
}
