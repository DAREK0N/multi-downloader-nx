use crate::models::*;
use crate::server::services::ServiceActions;

/// Crunchyroll service handler
pub struct CrunchyrollService;

impl ServiceActions for CrunchyrollService {
    async fn auth(&self, data: &AuthData) -> ApiResponse<String> {
        // In production: authenticate via Crunchyroll API
        // POST https://beta-api.crunchyroll.com/auth/v1/token
        if data.username.is_empty() || data.password.is_empty() {
            return ApiResponse::err("Username and password are required");
        }
        // Simulate successful auth
        ApiResponse::ok("cr_session_token_placeholder".to_string())
    }

    async fn check_token(&self) -> ApiResponse<bool> {
        // In production: verify stored token against Crunchyroll API
        ApiResponse::ok(true)
    }

    async fn search(&self, data: &SearchData) -> ApiResponse<Vec<SearchItem>> {
        // In production: GET https://beta-api.crunchyroll.com/content/v2/discover/search
        if data.search.is_empty() {
            return ApiResponse::err("Search query cannot be empty");
        }

        // Return demo data for demonstration
        let results = vec![
            SearchItem {
                id: "GRDQPM1ZY".to_string(),
                name: "Demon Slayer: Kimetsu no Yaiba".to_string(),
                desc: Some(
                    "Tanjiro sets out on the path of the Demon Slayer to save his sister."
                        .to_string(),
                ),
                image: "https://www.crunchyroll.com/imgsrv/display/thumbnail/480x720/catalog/crunchyroll/a249096c7812deb8c3c2c54a405e3aab.jpe".to_string(),
                lang: vec!["jpn".to_string(), "eng".to_string(), "deu".to_string()],
                rating: 4.8,
            },
            SearchItem {
                id: "GY8VEQ95Y".to_string(),
                name: "One Piece".to_string(),
                desc: Some("Monkey D. Luffy sets off on an adventure to find the One Piece.".to_string()),
                image: "https://www.crunchyroll.com/imgsrv/display/thumbnail/480x720/catalog/crunchyroll/757bae5a21039bac4ebace5c084e0209.jpe".to_string(),
                lang: vec!["jpn".to_string(), "eng".to_string()],
                rating: 4.9,
            },
            SearchItem {
                id: "GRMG8ZQZR".to_string(),
                name: "Attack on Titan".to_string(),
                desc: Some("Humanity fights for survival against giant humanoid Titans.".to_string()),
                image: "https://www.crunchyroll.com/imgsrv/display/thumbnail/480x720/catalog/crunchyroll/efa0dbe3a390d36dacd5caa22ad4a048.jpe".to_string(),
                lang: vec!["jpn".to_string(), "eng".to_string(), "fra".to_string()],
                rating: 4.9,
            },
        ];

        // Filter by search query (case-insensitive)
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
        // In production: GET https://beta-api.crunchyroll.com/content/v2/cms/series/{id}/seasons
        // Then for each season: GET .../seasons/{id}/episodes
        let episodes = vec![
            Episode {
                e: "1".to_string(),
                lang: vec!["jpn".to_string(), "eng".to_string()],
                name: "Cruelty".to_string(),
                season: "1".to_string(),
                season_title: "Season 1".to_string(),
                episode: "E01".to_string(),
                id: format!("{series_id}-E01"),
                img: String::new(),
                description: "First episode of the series.".to_string(),
                time: "24:00".to_string(),
            },
            Episode {
                e: "2".to_string(),
                lang: vec!["jpn".to_string(), "eng".to_string()],
                name: "Trainer Sakonji Urokodaki".to_string(),
                season: "1".to_string(),
                season_title: "Season 1".to_string(),
                episode: "E02".to_string(),
                id: format!("{series_id}-E02"),
                img: String::new(),
                description: "Second episode of the series.".to_string(),
                time: "24:00".to_string(),
            },
            Episode {
                e: "3".to_string(),
                lang: vec!["jpn".to_string(), "eng".to_string()],
                name: "Sabito and Makomo".to_string(),
                season: "1".to_string(),
                season_title: "Season 1".to_string(),
                episode: "E03".to_string(),
                id: format!("{series_id}-E03"),
                img: String::new(),
                description: "Third episode of the series.".to_string(),
                time: "24:00".to_string(),
            },
        ];

        ApiResponse::ok(episodes)
    }

    async fn available_dub_codes(&self) -> Vec<LanguageItem> {
        // Crunchyroll available audio languages
        vec![
            LanguageItem {
                code: "jpn".to_string(),
                name: "Japanese".to_string(),
            },
            LanguageItem {
                code: "eng".to_string(),
                name: "English".to_string(),
            },
            LanguageItem {
                code: "spa-419".to_string(),
                name: "Spanish (Latin America)".to_string(),
            },
            LanguageItem {
                code: "fra".to_string(),
                name: "French".to_string(),
            },
            LanguageItem {
                code: "deu".to_string(),
                name: "German".to_string(),
            },
            LanguageItem {
                code: "por".to_string(),
                name: "Portuguese (Brazil)".to_string(),
            },
            LanguageItem {
                code: "ita".to_string(),
                name: "Italian".to_string(),
            },
            LanguageItem {
                code: "rus".to_string(),
                name: "Russian".to_string(),
            },
            LanguageItem {
                code: "ara".to_string(),
                name: "Arabic".to_string(),
            },
            LanguageItem {
                code: "hin".to_string(),
                name: "Hindi".to_string(),
            },
        ]
    }

    async fn available_sub_codes(&self) -> Vec<LanguageItem> {
        // Crunchyroll available subtitle languages
        vec![
            LanguageItem {
                code: "eng".to_string(),
                name: "English".to_string(),
            },
            LanguageItem {
                code: "spa-419".to_string(),
                name: "Spanish (Latin America)".to_string(),
            },
            LanguageItem {
                code: "spa".to_string(),
                name: "Spanish".to_string(),
            },
            LanguageItem {
                code: "fra".to_string(),
                name: "French".to_string(),
            },
            LanguageItem {
                code: "deu".to_string(),
                name: "German".to_string(),
            },
            LanguageItem {
                code: "por".to_string(),
                name: "Portuguese (Brazil)".to_string(),
            },
            LanguageItem {
                code: "ita".to_string(),
                name: "Italian".to_string(),
            },
            LanguageItem {
                code: "rus".to_string(),
                name: "Russian".to_string(),
            },
            LanguageItem {
                code: "ara".to_string(),
                name: "Arabic".to_string(),
            },
        ]
    }

    async fn get_default(&self, name: &str) -> ApiResponse<String> {
        // Return default values matching cli-defaults.yml
        let value = match name {
            "dubLang" => "[\"jpn\"]",
            "dlsubs" => "[\"all\"]",
            "q" => "0",
            "fileName" => "[{service}] {title} - S{season}E{episode} [{quality}]",
            "dlVideoOnce" => "false",
            "simul" => "false",
            _ => "",
        };
        ApiResponse::ok(value.to_string())
    }
}
