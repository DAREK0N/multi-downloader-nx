use crate::models::*;
use crate::server::{authenticate, check_token};
use dioxus::prelude::*;

/// Authentication button and dialog
#[component]
pub fn AuthButton(service: ServiceType) -> Element {
    let mut show_dialog = use_signal(|| false);
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut authenticated = use_signal(|| false);

    // Check token on mount
    let service_clone = service.clone();
    use_effect(move || {
        let svc = service_clone.clone();
        spawn(async move {
            if let Ok(result) = check_token(svc).await {
                if result.is_ok() {
                    authenticated.set(true);
                }
            }
        });
    });

    let service_for_submit = service.clone();

    rsx! {
        if *authenticated.read() {
            div { class: "auth-status auth-status-success",
                span { class: "auth-icon", "✓" }
                span { "Authenticated" }
            }
        } else {
            button {
                class: "btn btn-primary auth-login-btn",
                onclick: move |_| show_dialog.set(true),
                span { class: "btn-icon", "🔑" }
                span { "Login" }
            }
        }

        if *show_dialog.read() {
            div { class: "modal-overlay",
                onclick: move |_| show_dialog.set(false),
                div {
                    class: "modal-content auth-modal",
                    onclick: move |e| e.stop_propagation(),
                    div { class: "modal-header",
                        h2 { "Login to {service.label()}" }
                        button {
                            class: "modal-close",
                            onclick: move |_| show_dialog.set(false),
                            "✕"
                        }
                    }
                    form {
                        class: "auth-form",
                        onsubmit: {
                            let service = service_for_submit.clone();
                            move |e: FormEvent| {
                                e.stop_propagation();
                                let svc = service.clone();
                                let user = username.read().clone();
                                let pass = password.read().clone();
                                spawn(async move {
                                    loading.set(true);
                                    error_msg.set(None);
                                    let auth_data = AuthData {
                                        username: user,
                                        password: pass,
                                    };
                                    match authenticate(svc, auth_data).await {
                                        Ok(ApiResponse::Ok(_)) => {
                                            authenticated.set(true);
                                            show_dialog.set(false);
                                        }
                                        Ok(ApiResponse::Err(reason)) => {
                                            error_msg.set(Some(reason));
                                        }
                                        Err(err) => {
                                            error_msg.set(Some(err.to_string()));
                                        }
                                    }
                                    loading.set(false);
                                });
                            }
                        },
                        div { class: "form-group",
                            label { r#for: "username", "Username / Email" }
                            input {
                                r#type: "text",
                                id: "username",
                                class: "form-input",
                                placeholder: "Enter your username",
                                value: "{username}",
                                oninput: move |e| username.set(e.value()),
                            }
                        }
                        div { class: "form-group",
                            label { r#for: "password", "Password" }
                            input {
                                r#type: "password",
                                id: "password",
                                class: "form-input",
                                placeholder: "Enter your password",
                                value: "{password}",
                                oninput: move |e| password.set(e.value()),
                            }
                        }
                        if let Some(err) = error_msg.read().as_ref() {
                            div { class: "auth-error",
                                span { class: "error-icon", "⚠" }
                                span { "{err}" }
                            }
                        }
                        div { class: "form-actions",
                            button {
                                r#type: "submit",
                                class: "btn btn-primary btn-full",
                                disabled: *loading.read(),
                                if *loading.read() {
                                    span { class: "spinner", "" }
                                    " Logging in..."
                                } else {
                                    "Login"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
