//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//! The [`WelcomeScreen`] component will be rendered when the current route is [`Route::WelcomeScreen`].
//! The [`MainApp`] component will be rendered when the current route is [`Route::MainApp`].

mod welcome_screen;
pub use welcome_screen::WelcomeScreen;

mod main_app;
pub use main_app::MainApp;
