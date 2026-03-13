use crate::models::*;
use crate::server::search_content;
use dioxus::prelude::*;

/// Search box component for finding anime/content
#[component]
pub fn SearchBox(service: ServiceType, on_select: EventHandler<SearchItem>) -> Element {
    let mut search_query = use_signal(String::new);
    let mut search_results = use_signal(Vec::<SearchItem>::new);
    let mut show_results = use_signal(|| false);
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    let service_for_search = service.clone();

    let trigger_search = {
        let service_for_search = service_for_search.clone();
        move || {
            let query = search_query.read().clone();
            let svc = service_for_search.clone();
            if query.len() >= 3 {
                spawn(async move {
                    loading.set(true);
                    error_msg.set(None);
                    let data = SearchData {
                        search: query,
                        page: Some(1),
                        search_type: None,
                        search_locale: None,
                    };
                    match search_content(svc, data).await {
                        Ok(ApiResponse::Ok(results)) => {
                            search_results.set(results);
                            show_results.set(true);
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
        }
    };

    rsx! {
        div { class: "search-box",
            div { class: "search-input-wrapper",
                span { class: "search-icon", "🔍" }
                input {
                    r#type: "text",
                    class: "search-input",
                    placeholder: "Search anime... (min 3 characters)",
                    value: "{search_query}",
                    oninput: move |e| {
                        search_query.set(e.value());
                        if e.value().len() < 3 {
                            show_results.set(false);
                        }
                    },
                    onkeypress: {
                        let trigger = trigger_search.clone();
                        move |e: Event<KeyboardData>| {
                            if e.key() == Key::Enter {
                                trigger();
                            }
                        }
                    },
                }
                if *loading.read() {
                    span { class: "search-spinner spinner", "" }
                }
                button {
                    class: "btn btn-primary search-btn",
                    onclick: move |_| {
                        trigger_search();
                    },
                    disabled: search_query.read().len() < 3,
                    "Search"
                }
            }

            if let Some(err) = error_msg.read().as_ref() {
                div { class: "search-error",
                    "⚠ {err}"
                }
            }

            if *show_results.read() && !search_results.read().is_empty() {
                div { class: "search-results",
                    for item in search_results.read().iter() {
                        SearchResultItem {
                            key: "{item.id}",
                            item: item.clone(),
                            on_select: move |item: SearchItem| {
                                show_results.set(false);
                                on_select.call(item);
                            },
                        }
                    }
                }
            }

            if *show_results.read() && search_results.read().is_empty() && !*loading.read() {
                div { class: "search-empty",
                    p { "No results found. Try a different search term." }
                }
            }
        }
    }
}

#[component]
fn SearchResultItem(item: SearchItem, on_select: EventHandler<SearchItem>) -> Element {
    let item_for_click = item.clone();

    rsx! {
        div {
            class: "search-result-item",
            onclick: move |_| on_select.call(item_for_click.clone()),
            if !item.image.is_empty() {
                div { class: "search-result-img",
                    img {
                        src: "{item.image}",
                        alt: "{item.name}",
                        loading: "lazy",
                    }
                }
            } else {
                div { class: "search-result-img search-result-placeholder",
                    span { "🎬" }
                }
            }
            div { class: "search-result-info",
                h4 { class: "search-result-name", "{item.name}" }
                if let Some(ref desc) = item.desc {
                    p { class: "search-result-desc",
                        {desc.chars().take(120).collect::<String>()}
                        if desc.len() > 120 { "..." } else { "" }
                    }
                }
                div { class: "search-result-meta",
                    div { class: "search-result-rating",
                        span { class: "star", "★" }
                        span { " {item.rating:.1}" }
                    }
                    div { class: "search-result-langs",
                        for lang in item.lang.iter() {
                            span { class: "lang-tag", "{lang}" }
                        }
                    }
                }
            }
        }
    }
}
