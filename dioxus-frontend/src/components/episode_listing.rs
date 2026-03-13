use crate::models::*;
use dioxus::prelude::*;

/// Episode listing component - shows available episodes for a series
#[component]
pub fn EpisodeListing(episodes: Vec<Episode>, on_select_episode: EventHandler<String>) -> Element {
    if episodes.is_empty() {
        return rsx! {};
    }

    // Group episodes by season
    let mut seasons: std::collections::BTreeMap<String, Vec<&Episode>> =
        std::collections::BTreeMap::new();
    for ep in episodes.iter() {
        seasons.entry(ep.season_title.clone()).or_default().push(ep);
    }

    rsx! {
        div { class: "episode-listing",
            h3 { class: "section-title", "📺 Episodes" }

            div { class: "episode-stats",
                span { class: "stat-badge",
                    "{episodes.len()} episodes"
                }
                span { class: "stat-badge",
                    "{seasons.len()} season(s)"
                }
            }

            for (season_title, eps) in seasons.iter() {
                div { class: "season-group",
                    h4 { class: "season-title", "{season_title}" }
                    div { class: "episode-grid",
                        for ep in eps.iter() {
                            {
                                let ep_id = ep.id.clone();
                                rsx! {
                                    div {
                                        class: "episode-card",
                                        onclick: move |_| on_select_episode.call(ep_id.clone()),
                                        div { class: "episode-card-header",
                                            span { class: "episode-number", "{ep.episode}" }
                                            span { class: "episode-time", "⏱ {ep.time}" }
                                        }
                                        h5 { class: "episode-name", "{ep.name}" }
                                        p { class: "episode-description",
                                            {ep.description.chars().take(80).collect::<String>()}
                                            if ep.description.len() > 80 { "..." } else { "" }
                                        }
                                        div { class: "episode-langs",
                                            for lang in ep.lang.iter() {
                                                span { class: "lang-tag lang-tag-sm", "{lang}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
