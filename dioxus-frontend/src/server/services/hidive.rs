use crate::models::*;
use crate::server::services::ServiceActions;

/// Hidive service handler
pub struct HidiveService;

impl ServiceActions for HidiveService {
    async fn auth(&self, data: &AuthData) -> ApiResponse<String> {
        // In production: authenticate via Hidive API
        if data.username.is_empty() || data.password.is_empty() {
            return ApiResponse::err("Username and password are required");
        }
        ApiResponse::ok("hidive_session_token_placeholder".to_string())
    }

    async fn check_token(&self) -> ApiResponse<bool> {
        // In production: verify stored Hidive token
        ApiResponse::ok(true)
    }

    async fn search(&self, data: &SearchData) -> ApiResponse<Vec<SearchItem>> {
        // In production: POST https://dce-frontoffice.imggaming.com/api/v4/search
        if data.search.is_empty() {
            return ApiResponse::err("Search query cannot be empty");
        }

        let results = vec![
            SearchItem {
                id: "1001".to_string(),
                name: "My Hero Academia".to_string(),
                desc: Some(
                    "The story of a boy born without superpowers in a world where they are the norm."
                        .to_string(),
                ),
                image: String::new(),
                lang: vec!["jpn".to_string(), "eng".to_string()],
                rating: 4.7,
            },
            SearchItem {
                id: "1002".to_string(),
                name: "Made in Abyss".to_string(),
                desc: Some("An orphan girl discovers a robot boy and descends into the Abyss.".to_string()),
                image: String::new(),
                lang: vec!["jpn".to_string(), "eng".to_string()],
                rating: 4.8,
            },
        ];

        let filtered: Vec<SearchItem> = results
            .into_iter()
            .filter(|item| {
                item.name
                    .to_lowercase()
                    .contains(&data.search.to_lowercase())
            })
            .collect();

        ApiResponse::ok(filtered)
    }

    async fn list_episodes(&self, series_id: &str) -> ApiResponse<Vec<Episode>> {
        // In production: GET https://dce-frontoffice.imggaming.com/api/v4/vod/series/{id}/seasons
        let episodes = vec![
            Episode {
                e: "1".to_string(),
                lang: vec!["jpn".to_string(), "eng".to_string()],
                name: "Episode 1".to_string(),
                season: "1".to_string(),
                season_title: "Season 1".to_string(),
                episode: "E01".to_string(),
                id: format!("{series_id}-E01"),
                img: String::new(),
                description: "First episode.".to_string(),
                time: "24:00".to_string(),
            },
            Episode {
                e: "2".to_string(),
                lang: vec!["jpn".to_string(), "eng".to_string()],
                name: "Episode 2".to_string(),
                season: "1".to_string(),
                season_title: "Season 1".to_string(),
                episode: "E02".to_string(),
                id: format!("{series_id}-E02"),
                img: String::new(),
                description: "Second episode.".to_string(),
                time: "24:00".to_string(),
            },
        ];

        ApiResponse::ok(episodes)
    }

    async fn available_dub_codes(&self) -> Vec<LanguageItem> {
        vec![
            LanguageItem {
                code: "jpn".to_string(),
                name: "Japanese".to_string(),
            },
            LanguageItem {
                code: "eng".to_string(),
                name: "English".to_string(),
            },
        ]
    }

    async fn available_sub_codes(&self) -> Vec<LanguageItem> {
        vec![
            LanguageItem {
                code: "all".to_string(),
                name: "All".to_string(),
            },
            LanguageItem {
                code: "none".to_string(),
                name: "None".to_string(),
            },
            LanguageItem {
                code: "eng".to_string(),
                name: "English".to_string(),
            },
            LanguageItem {
                code: "spa".to_string(),
                name: "Spanish".to_string(),
            },
        ]
    }

    async fn get_default(&self, name: &str) -> ApiResponse<String> {
        let value = match name {
            "dubLang" => "[\"jpn\"]",
            "dlsubs" => "[\"all\"]",
            "q" => "0",
            "fileName" => "[{service}] {title} - S{season}E{episode} [{quality}]",
            "dlVideoOnce" => "false",
            _ => "",
        };
        ApiResponse::ok(value.to_string())
    }
}
