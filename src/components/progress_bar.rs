use dioxus::prelude::*;

#[component]
pub fn ProgressBar(
    value: f32,
    #[props(default = 100.0)] max: f32,
    #[props(default = true)] show_label: bool,
    #[props(default = "".to_string())] label: String,
    #[props(default = "bg-accent".to_string())] color: String,
) -> Element {
    let percentage = (value / max * 100.0).min(100.0).max(0.0);
    
    rsx! {
        div { class: "w-full",
            if show_label {
                div { class: "flex items-center justify-between mb-1",
                    if !label.is_empty() {
                        span { class: "text-sm font-medium text-text-primary", {label} }
                    }
                    span { class: "text-sm font-medium text-text-secondary", "{percentage:.1}%" }
                }
            }

            div { class: "w-full bg-background rounded-full h-2 overflow-hidden",
                div {
                    class: "h-full rounded-full transition-all duration-300 {color}",
                    style: "width: {percentage}%",
                }
            }
        }
    }
}

#[component]
pub fn CircularProgress(
    #[props(default = 40.0)] size: f32,
    #[props(default = 4.0)] stroke_width: f32,
) -> Element {
    let radius = (size - stroke_width) / 2.0;
    let circumference = 2.0 * std::f32::consts::PI * radius;
    
    rsx! {
        div { class: "inline-flex items-center justify-center",
            svg {
                class: "animate-spin",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 {size} {size}",

                circle {
                    class: "text-background",
                    stroke: "currentColor",
                    stroke_width: "{stroke_width}",
                    fill: "none",
                    cx: "{size / 2.0}",
                    cy: "{size / 2.0}",
                    r: "{radius}",
                }

                circle {
                    class: "text-accent",
                    stroke: "currentColor",
                    stroke_width: "{stroke_width}",
                    stroke_linecap: "round",
                    fill: "none",
                    cx: "{size / 2.0}",
                    cy: "{size / 2.0}",
                    r: "{radius}",
                    stroke_dasharray: "{circumference}",
                    stroke_dashoffset: "{circumference * 0.25}",
                }
            }
        }
    }
}
