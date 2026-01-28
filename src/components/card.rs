use dioxus::prelude::*;

#[component]
pub fn Card(
    #[props(default = "".to_string())] title: String,
    #[props(default = false)] elevated: bool,
    children: Element,
) -> Element {
    let shadow_class = if elevated { "shadow-lg" } else { "shadow" };
    
    rsx! {
        div { class: "bg-surface border border-border rounded-lg {shadow_class}",
            if !title.is_empty() {
                div { class: "px-6 py-4 border-b border-border",
                    h3 { class: "text-lg font-semibold text-text-primary", {title} }
                }
            }

            div { class: "p-6", {children} }
        }
    }
}

#[component]
pub fn CardHeader(children: Element) -> Element {
    rsx! {
        div { class: "px-6 py-4 border-b border-border", {children} }
    }
}

#[component]
pub fn CardBody(children: Element) -> Element {
    rsx! {
        div { class: "p-6", {children} }
    }
}

#[component]
pub fn CardFooter(children: Element) -> Element {
    rsx! {
        div { class: "px-6 py-4 border-t border-border bg-background rounded-b-lg",
            {children}
        }
    }
}
