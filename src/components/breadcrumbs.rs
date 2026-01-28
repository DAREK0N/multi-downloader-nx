use dioxus::prelude::*;
use crate::router::Route;

#[derive(Clone, PartialEq)]
struct Crumb {
    label: String,
    to: Route,
}

#[component]
pub fn Breadcrumbs() -> Element {
    let current_route = use_route::<Route>();

    let crumbs: Vec<Crumb> = match current_route {
        Route::Home {} => vec![Crumb { label: "Home".into(), to: Route::Home {} }],
        Route::Downloads {} => vec![
            Crumb { label: "Home".into(), to: Route::Home {} },
            Crumb { label: "Downloads".into(), to: Route::Downloads {} },
        ],
        Route::Scheduled {} => vec![
            Crumb { label: "Home".into(), to: Route::Home {} },
            Crumb { label: "Scheduled".into(), to: Route::Scheduled {} },
        ],
        Route::Settings {} => vec![
            Crumb { label: "Home".into(), to: Route::Home {} },
            Crumb { label: "Settings".into(), to: Route::Settings {} },
        ],
        Route::Webhooks {} => vec![
            Crumb { label: "Home".into(), to: Route::Home {} },
            Crumb { label: "Webhooks".into(), to: Route::Webhooks {} },
        ],
    };

    rsx! {
        nav { class: "flex items-center space-x-2 text-sm text-text-secondary",
            for (idx, crumb) in crumbs.iter().enumerate() {
                if idx > 0 {
                    span { class: "text-border", "/" }
                }
                Link {
                    to: crumb.to.clone(),
                    class: "hover:text-text-primary transition-colors",
                    "{crumb.label}"
                }
            }
        }
    }
}
