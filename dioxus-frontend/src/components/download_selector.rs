use crate::models::*;
use crate::server::{available_dub_codes, available_sub_codes};
use dioxus::prelude::*;

/// Download configuration selector
#[component]
pub fn DownloadSelector(
    service: ServiceType,
    selected_item: Option<SearchItem>,
    on_add_to_queue: EventHandler<DownloadOptions>,
    on_list_episodes: EventHandler<String>,
) -> Element {
    let mut options = use_signal(DownloadOptions::default);
    let mut available_dubs = use_signal(Vec::<LanguageItem>::new);
    let mut available_subs = use_signal(Vec::<LanguageItem>::new);
    let mut dubs_loaded = use_signal(|| false);

    // Load available languages on mount
    let service_dub = service.clone();
    let service_sub = service.clone();
    use_effect(move || {
        let svc_dub = service_dub.clone();
        let svc_sub = service_sub.clone();
        spawn(async move {
            if let Ok(dubs) = available_dub_codes(svc_dub).await {
                available_dubs.set(dubs);
            }
            if let Ok(subs) = available_sub_codes(svc_sub).await {
                available_subs.set(subs);
            }
            dubs_loaded.set(true);
        });
    });

    // Update ID when item is selected
    if let Some(ref item) = selected_item {
        let mut opts = options.write();
        if opts.id != item.id {
            opts.id = item.id.clone();
        }
    }

    let current_opts = options.read().clone();

    rsx! {
        div { class: "download-selector",
            h3 { class: "section-title",
                "📋 Download Options"
            }

            if selected_item.is_some() {
                div { class: "selector-form",
                    // Series ID
                    div { class: "form-row",
                        div { class: "form-group",
                            label { "Series ID" }
                            input {
                                r#type: "text",
                                class: "form-input",
                                value: "{current_opts.id}",
                                oninput: move |e| {
                                    options.write().id = e.value();
                                },
                            }
                        }
                        div { class: "form-group",
                            label { "Episode(s)" }
                            input {
                                r#type: "text",
                                class: "form-input",
                                placeholder: "e.g. 1-5, 1, all",
                                value: "{current_opts.e}",
                                oninput: move |e| {
                                    options.write().e = e.value();
                                },
                            }
                        }
                    }

                    // Quality
                    div { class: "form-group",
                        label { "Quality" }
                        div { class: "quality-options",
                            for (val, label) in quality_options() {
                                button {
                                    class: if current_opts.quality == val { "quality-btn active" } else { "quality-btn" },
                                    onclick: move |_| {
                                        options.write().quality = val;
                                    },
                                    "{label}"
                                }
                            }
                        }
                    }

                    // Audio Languages
                    if *dubs_loaded.read() {
                        div { class: "form-group",
                            label { "Audio Languages" }
                            div { class: "multi-select",
                                for lang in available_dubs.read().iter() {
                                    {
                                        let code = lang.code.clone();
                                        let is_selected = current_opts.dub_lang.contains(&code);
                                        let code_click = code.clone();
                                        rsx! {
                                            button {
                                                class: if is_selected { "tag-btn active" } else { "tag-btn" },
                                                onclick: move |_| {
                                                    let mut opts = options.write();
                                                    if opts.dub_lang.contains(&code_click) {
                                                        opts.dub_lang.retain(|c| c != &code_click);
                                                    } else {
                                                        opts.dub_lang.push(code_click.clone());
                                                    }
                                                },
                                                "{lang.name}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Subtitle Languages
                    if *dubs_loaded.read() {
                        div { class: "form-group",
                            label { "Subtitle Languages" }
                            div { class: "multi-select",
                                for lang in available_subs.read().iter() {
                                    {
                                        let code = lang.code.clone();
                                        let is_selected = current_opts.dl_subs.contains(&code);
                                        let code_click = code.clone();
                                        rsx! {
                                            button {
                                                class: if is_selected { "tag-btn active" } else { "tag-btn" },
                                                onclick: move |_| {
                                                    let mut opts = options.write();
                                                    if opts.dl_subs.contains(&code_click) {
                                                        opts.dl_subs.retain(|c| c != &code_click);
                                                    } else {
                                                        opts.dl_subs.push(code_click.clone());
                                                    }
                                                },
                                                "{lang.name}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // File Name Template
                    div { class: "form-group",
                        label { "File Name Template" }
                        input {
                            r#type: "text",
                            class: "form-input",
                            value: "{current_opts.file_name}",
                            oninput: move |e| {
                                options.write().file_name = e.value();
                            },
                        }
                    }

                    // Toggle Options
                    div { class: "form-row options-toggles",
                        ToggleOption {
                            label: "All Episodes",
                            checked: current_opts.all,
                            on_change: move |v| { options.write().all = v; },
                        }
                        ToggleOption {
                            label: "Skip Video",
                            checked: current_opts.no_vids,
                            on_change: move |v| { options.write().no_vids = v; },
                        }
                        ToggleOption {
                            label: "Skip Audio",
                            checked: current_opts.no_audio,
                            on_change: move |v| { options.write().no_audio = v; },
                        }
                        ToggleOption {
                            label: "DL Video Once",
                            checked: current_opts.dl_video_once,
                            on_change: move |v| { options.write().dl_video_once = v; },
                        }
                    }

                    // Action Buttons
                    div { class: "form-actions",
                        button {
                            class: "btn btn-secondary",
                            onclick: {
                                let id = current_opts.id.clone();
                                move |_| {
                                    on_list_episodes.call(id.clone());
                                }
                            },
                            "📋 List Episodes"
                        }
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                on_add_to_queue.call(options.read().clone());
                            },
                            "➕ Add to Queue"
                        }
                    }
                }
            } else {
                div { class: "selector-empty",
                    p { "Search and select an anime above to configure download options." }
                }
            }
        }
    }
}

#[component]
fn ToggleOption(label: &'static str, checked: bool, on_change: EventHandler<bool>) -> Element {
    rsx! {
        label { class: "toggle-option",
            input {
                r#type: "checkbox",
                checked,
                onchange: move |e| on_change.call(e.checked()),
            }
            span { class: "toggle-slider", "" }
            span { class: "toggle-label", "{label}" }
        }
    }
}
