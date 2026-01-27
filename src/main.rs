mod app;
mod components;
mod models;
mod pages;
mod router;
mod server;
mod services;
mod state;
mod hooks;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Launch the Dioxus application
    dioxus::launch(app::App);
}
