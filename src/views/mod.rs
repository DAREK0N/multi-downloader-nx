//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//!
//! The [`Home`] component will be rendered when the current route is [`Route::Home`].
//!
//! Note: Blog and Navbar components are commented out during migration to new routing system.

mod home;
pub use home::Home;

// Temporarily disabled during migration
// mod blog;
// pub use blog::Blog;

// mod navbar;
// pub use navbar::Navbar;
