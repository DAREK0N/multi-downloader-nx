use crate::models::*;
use crate::server::services::ServiceActions;

/// Animation Digital Network (ADN) service handler
pub struct AdnService;

impl ServiceActions for AdnService {
    async fn auth(&self, data: &AuthData) -> ApiResponse<String> {
        // In production: authenticate via ADN API
        // POST https://gw.api.animedigitalnetwork.fr/authentication/login
        if data.username.is_empty() || data.password.is_empty() {
            return ApiResponse::err("Username and password are required");
        }
        ApiResponse::ok("adn_session_token_placeholder".to_string())
    }

    async fn check_token(&self) -> ApiResponse<bool> {
        // In production: verify stored ADN token
        ApiResponse::ok(true)
    }

    async fn search(&self, data: &SearchData) -> ApiResponse<Vec<SearchItem>> {
        // In production: GET https://gw.api.animedigitalnetwork.fr/show/catalog
        if data.search.is_empty() {
            return ApiResponse::err("Search query cannot be empty");
        }

        let results = vec![
            SearchItem {
                id: "2001".to_string(),
                name: "Naruto Shippuden".to_string(),
                desc: Some(
                    "Naruto returns after two years of training to face new threats.".to_string(),
                ),
                image: String::new(),
                lang: vec!["jpn".to_string(), "fra".to_string()],
                rating: 4.6,
            },
            SearchItem {
                id: "2002".to_string(),
                name: "Dragon Ball Super".to_string(),
                desc: Some("Goku faces powerful new foes from across the multiverse.".to_string()),
                image: String::new(),
                lang: vec!["jpn".to_string(), "fra".to_string()],
                rating: 4.5,
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
        // In production: GET https://gw.api.animedigitalnetwork.fr/video/show/{id}
        let episodes = vec![
            Episode {
                e: "1".to_string(),
                lang: vec!["jpn".to_string(), "fra".to_string()],
                name: "Épisode 1".to_string(),
                season: "1".to_string(),
                season_title: "Saison 1".to_string(),
                episode: "E01".to_string(),
                id: format!("{series_id}-E01"),
                img: String::new(),
                description: "Premier épisode de la série.".to_string(),
                time: "24:00".to_string(),
            },
            Episode {
                e: "2".to_string(),
                lang: vec!["jpn".to_string(), "fra".to_string()],
                name: "Épisode 2".to_string(),
                season: "1".to_string(),
                season_title: "Saison 1".to_string(),
                episode: "E02".to_string(),
                id: format!("{series_id}-E02"),
                img: String::new(),
                description: "Deuxième épisode de la série.".to_string(),
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
                code: "fra".to_string(),
                name: "French".to_string(),
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
                code: "fra".to_string(),
                name: "French".to_string(),
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
