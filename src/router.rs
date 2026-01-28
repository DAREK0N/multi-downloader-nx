use dioxus::prelude::*;
use crate::pages::{
    Home,
    downloads::Downloads,
    scheduled::Scheduled,
    settings::Settings,
    webhooks::Webhooks,
    queue::QueuePage,
};

/// Application routes
#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(MainLayout)]
        #[route("/")]
        Home {},
        
        #[route("/downloads")]
        Downloads {},
        
        #[route("/scheduled")]
        Scheduled {},
        
        #[route("/settings")]
        Settings {},
        
        #[route("/webhooks")]
        Webhooks {},
        
        #[route("/queue")]
        Queue {},
}

/// Main layout wrapper for all pages
#[component]
fn MainLayout() -> Element {
    rsx! {
        div { class: "min-h-screen bg-background text-text-primary",
            crate::components::NavBar {}

            main { class: "container mx-auto px-4 py-8 space-y-4",
                crate::components::Breadcrumbs {}
                Outlet::<Route> {}
            }
        }
    }
}
