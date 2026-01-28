use dioxus::prelude::*;
use crate::models::ServiceType;
use crate::components::{Button, ButtonVariant, IconButton};
use crate::state::AuthState;

#[derive(Props, PartialEq, Clone)]
pub struct AuthButtonProps {
    service: ServiceType,
    #[props(default = false)]
    is_authenticated: bool,
    #[props(default = false)]
    loading: bool,
    on_click: EventHandler<()>,
    on_logout: Option<EventHandler<()>>,
}

#[component]
pub fn AuthButton(props: AuthButtonProps) -> Element {
    let label = match props.service {
        ServiceType::Crunchyroll => "Crunchyroll",
        ServiceType::Hidive => "HIDIVE",
        ServiceType::Adn => "ADN",
    };

    let color = if props.is_authenticated {
        ButtonVariant::Success
    } else {
        ButtonVariant::Primary
    };

    rsx! {
        div { class: "flex items-center space-x-2",
            Button {
                variant: color,
                disabled: props.loading,
                onclick: move |_| props.on_click.call(()),
                if props.loading { "Connecting..." } else { format!("Connect {label}") }
            }
            if props.is_authenticated {
                IconButton {
                    icon: "🚪".to_string(),
                    variant: ButtonVariant::Ghost,
                    title: "Logout".to_string(),
                    onclick: move |_| {
                        if let Some(handler) = &props.on_logout {
                            handler.call(());
                        }
                    }
                }
            }
        }
    }
}
