use dioxus::prelude::*;

// Import app and router modules
mod app;
mod router;
mod components;
mod views;
mod pages;
mod models;
mod state;
mod hooks;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
mod services;

fn main() {
    dioxus::launch(app::App);
}
