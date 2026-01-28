//! Custom Dioxus hooks

pub mod use_theme;
pub mod use_i18n;
pub mod use_websocket;

// Re-export hooks
pub use use_theme::use_theme;
pub use use_i18n::use_i18n;
pub use use_websocket::use_websocket;
