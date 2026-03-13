use serde::{Deserialize, Serialize};

/// Service provider type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ServiceType {
    #[default]
    Crunchyroll,
    Hidive,
    Adn,
}

impl ServiceType {
    pub fn label(&self) -> &'static str {
        match self {
            ServiceType::Crunchyroll => "Crunchyroll",
            ServiceType::Hidive => "Hidive",
            ServiceType::Adn => "ADN",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ServiceType::Crunchyroll => "🍥",
            ServiceType::Hidive => "📺",
            ServiceType::Adn => "🎬",
        }
    }
}

impl std::fmt::Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// Authentication request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

/// Generic API response wrapper
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ApiResponse<T> {
    Ok(T),
    Err(String),
}

impl<T> ApiResponse<T> {
    pub fn ok(value: T) -> Self {
        ApiResponse::Ok(value)
    }

    pub fn err(reason: impl Into<String>) -> Self {
        ApiResponse::Err(reason.into())
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, ApiResponse::Ok(_))
    }
}

/// Search request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchData {
    pub search: String,
    pub page: Option<u32>,
    pub search_type: Option<String>,
    pub search_locale: Option<String>,
}

/// Search result item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchItem {
    pub image: String,
    pub name: String,
    pub desc: Option<String>,
    pub id: String,
    pub lang: Vec<String>,
    pub rating: f32,
}

/// Episode information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Episode {
    pub e: String,
    pub lang: Vec<String>,
    pub name: String,
    pub season: String,
    pub season_title: String,
    pub episode: String,
    pub id: String,
    pub img: String,
    pub description: String,
    pub time: String,
}

/// Queue item for download
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueueItem {
    pub title: String,
    pub episode: String,
    pub file_name: String,
    pub dl_subs: Vec<String>,
    pub parent: ParentInfo,
    pub quality: u32,
    pub dl_video_once: bool,
    pub dub_lang: Vec<String>,
    pub image: String,
    pub id: String,
    pub all: bool,
    pub but: bool,
    pub no_vids: bool,
    pub no_audio: bool,
    pub e: String,
}

/// Parent series info
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParentInfo {
    pub title: String,
    pub season: String,
}

/// Download progress data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressData {
    pub total: u64,
    pub cur: u64,
    pub percent: f64,
    pub time: f64,
    pub download_speed: f64,
    pub bytes: u64,
}

/// Language item for display
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageItem {
    pub code: String,
    pub name: String,
}

/// Download info for current download display
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadInfo {
    pub image: String,
    pub parent_title: String,
    pub title: String,
    pub language: LanguageItem,
    pub file_name: String,
}

/// Extended progress (progress + download info)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedProgress {
    pub progress: ProgressData,
    pub download_info: DownloadInfo,
}

/// Download options for configuring downloads
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadOptions {
    pub quality: u32,
    pub id: String,
    pub e: String,
    pub dub_lang: Vec<String>,
    pub dl_subs: Vec<String>,
    pub file_name: String,
    pub dl_video_once: bool,
    pub all: bool,
    pub but: bool,
    pub no_vids: bool,
    pub no_audio: bool,
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self {
            quality: 0,
            id: String::new(),
            e: String::new(),
            dub_lang: vec!["jpn".to_string()],
            dl_subs: vec!["eng".to_string()],
            file_name: "[{service}] {title} - S{season}E{episode} [{quality}]".to_string(),
            dl_video_once: false,
            all: false,
            but: false,
            no_vids: false,
            no_audio: false,
        }
    }
}

/// GUI state for persistence
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GuiState {
    pub setup: bool,
    pub services: std::collections::HashMap<String, ServiceState>,
}

/// Per-service state
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceState {
    pub queue: Vec<QueueItem>,
}

/// Folder types for open commands
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FolderType {
    Content,
    Config,
}

/// App configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub password: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            password: None,
        }
    }
}

/// Available quality options
pub fn quality_options() -> Vec<(u32, &'static str)> {
    vec![
        (0, "Best"),
        (240, "240p"),
        (360, "360p"),
        (480, "480p"),
        (720, "720p"),
        (1080, "1080p"),
    ]
}

/// Available languages for dubs/subs
pub fn available_languages() -> Vec<LanguageItem> {
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
            code: "spa".to_string(),
            name: "Spanish".to_string(),
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
        LanguageItem {
            code: "kor".to_string(),
            name: "Korean".to_string(),
        },
        LanguageItem {
            code: "zho".to_string(),
            name: "Chinese".to_string(),
        },
    ]
}
