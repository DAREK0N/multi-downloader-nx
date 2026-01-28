use dioxus::prelude::*;

/// Button variant styles
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Success,
    Ghost,
}

/// Button size
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[component]
pub fn Button(
    #[props(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[props(default = ButtonSize::Medium)] size: ButtonSize,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] full_width: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let variant_classes = match variant {
        ButtonVariant::Primary => "bg-accent text-white hover:bg-accent-dark",
        ButtonVariant::Secondary => "bg-surface text-text-primary hover:bg-background border border-border",
        ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
        ButtonVariant::Success => "bg-green-600 text-white hover:bg-green-700",
        ButtonVariant::Ghost => "text-text-primary hover:bg-background",
    };
    
    let size_classes = match size {
        ButtonSize::Small => "px-3 py-1.5 text-sm",
        ButtonSize::Medium => "px-4 py-2 text-base",
        ButtonSize::Large => "px-6 py-3 text-lg",
    };
    
    let disabled_classes = if disabled {
        "opacity-50 cursor-not-allowed"
    } else {
        "cursor-pointer"
    };
    
    let width_class = if full_width { "w-full" } else { "" };
    
    rsx! {
        button {
            r#type: "button",
            class: "font-medium rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 {variant_classes} {size_classes} {disabled_classes} {width_class}",
            disabled,
            onclick: move |e| {
                if !disabled {
                    if let Some(handler) = &onclick {
                        handler.call(e);
                    }
                }
            },
            {children}
        }
    }
}

/// Icon button variant
#[component]
pub fn IconButton(
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default = false)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    aria_label: String,
    children: Element,
) -> Element {
    let variant_classes = match variant {
        ButtonVariant::Primary => "bg-accent text-white hover:bg-accent-dark",
        ButtonVariant::Secondary => "bg-surface text-text-primary hover:bg-background",
        ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
        ButtonVariant::Success => "bg-green-600 text-white hover:bg-green-700",
        ButtonVariant::Ghost => "text-text-primary hover:bg-background",
    };
    
    let disabled_classes = if disabled {
        "opacity-50 cursor-not-allowed"
    } else {
        "cursor-pointer"
    };
    
    rsx! {
        button {
            r#type: "button",
            class: "p-2 rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-accent {variant_classes} {disabled_classes}",
            disabled,
            "aria-label": "{aria_label}",
            onclick: move |e| {
                if !disabled {
                    if let Some(handler) = &onclick {
                        handler.call(e);
                    }
                }
            },
            {children}
        }
    }
}
