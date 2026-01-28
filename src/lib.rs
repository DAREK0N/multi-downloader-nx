//! Multi Downloader NX - Shared Library Code
//! 
//! This library provides shared types, utilities, and functionality
//! used across both client and server contexts.

pub mod models;
pub mod components;
pub mod views;
pub mod pages;
pub mod hooks;
pub mod state;
pub mod router;

// Re-export commonly used items
pub use models::*;
pub use router::Route;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "server")]
pub mod services;

// Initialize internationalization
rust_i18n::i18n!("locales", fallback = "en");
