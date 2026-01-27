//! Multi-Downloader NX Library
//! 
//! A Dioxus fullstack application for downloading anime from various streaming services.

// Initialize rust-i18n
rust_i18n::i18n!("locales", fallback = "en");

pub mod components;
pub mod models;
pub mod pages;
pub mod router;
pub mod server;
pub mod services;
pub mod state;
pub mod hooks;

// Re-export commonly used items
pub use components::*;
pub use models::*;
pub use state::*;
