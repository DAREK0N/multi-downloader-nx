use dioxus::prelude::*;

#[component]
pub fn Modal(
    show: Signal<bool>,
    #[props(default = "".to_string())] title: String,
    #[props(default = true)] show_close: bool,
    children: Element,
) -> Element {
    if !show() {
        return rsx! {};
    }
    
    rsx! {
        div {
            class: "fixed inset-0 z-50 overflow-y-auto",
            "aria-labelledby": "modal-title",
            role: "dialog",
            "aria-modal": "true",

            // Backdrop
            div {
                class: "fixed inset-0 bg-black bg-opacity-50 transition-opacity",
                onclick: move |_| {
                    if show_close {
                        show.set(false)
                    }
                },
            }

            // Modal content
            div { class: "flex items-center justify-center min-h-screen p-4",
                div {
                    class: "relative bg-surface rounded-lg shadow-xl max-w-lg w-full border border-border",
                    onclick: move |e| e.stop_propagation(),

                    // Header
                    div { class: "flex items-center justify-between px-6 py-4 border-b border-border",
                        if !title.is_empty() {
                            h3 {
                                class: "text-lg font-semibold text-text-primary",
                                id: "modal-title",
                                {title}
                            }
                        }

                        if show_close {
                            button {
                                r#type: "button",
                                class: "text-text-secondary hover:text-text-primary transition-colors",
                                onclick: move |_| show.set(false),
                                "aria-label": "Close modal",

                                svg {
                                    class: "h-6 w-6",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M6 18L18 6M6 6l12 12",
                                    }
                                }
                            }
                        }
                    }

                    // Body
                    div { class: "px-6 py-4", {children} }
                }
            }
        }
    }
}

#[component]
pub fn ModalFooter(children: Element) -> Element {
    rsx! {
        div { class: "flex items-center justify-end space-x-3 px-6 py-4 border-t border-border",
            {children}
        }
    }
}
