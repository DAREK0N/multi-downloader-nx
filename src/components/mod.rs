//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod hero;
pub use hero::Hero;

mod echo;
pub use echo::Echo;

mod navigation;
pub use navigation::NavBar;
mod breadcrumbs;
pub use breadcrumbs::Breadcrumbs;

mod theme_switcher;
pub use theme_switcher::ThemeSwitcher;
mod theme_preview;
pub use theme_preview::ThemePreview;

// Reusable UI components
mod button;
pub use button::{Button, ButtonSize, ButtonVariant, IconButton};

mod input;
pub use input::{Input, TextArea};

mod card;
pub use card::{Card, CardBody, CardFooter, CardHeader};

mod modal;
pub use modal::{Modal, ModalFooter};

mod progress_bar;
pub use progress_bar::{CircularProgress, ProgressBar};

mod status_indicator;
pub use status_indicator::{DownloadStatusBadge, StatusIndicator, StatusVariant};

mod multi_select;
pub use multi_select::{MultiSelect, SelectOption};

mod context_menu;
pub use context_menu::{ContextMenu, MenuItem};

mod auth_button;
pub use auth_button::AuthButton;

// Download-specific components
mod download_card;
pub use download_card::DownloadCard;

mod download_list;
pub use download_list::{DownloadFilter, DownloadList};
