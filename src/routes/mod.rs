//! Application routing definitions

use dioxus::prelude::*;
use crate::ui::views::{WelcomeScreen, MainApp};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    WelcomeScreen {},
    #[route("/app")]
    MainApp {},
}