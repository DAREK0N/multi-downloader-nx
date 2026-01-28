use serde::{Deserialize, Serialize};

/// Supported streaming services
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceType {
    Crunchyroll,
    Hidive,
    Adn,
}

impl ServiceType {
    pub fn as_str(&self) -> &str {
        match self {
            ServiceType::Crunchyroll => "crunchyroll",
            ServiceType::Hidive => "hidive",
            ServiceType::Adn => "adn",
        }
    }
}

impl std::fmt::Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
