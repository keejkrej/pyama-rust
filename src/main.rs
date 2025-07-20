use dioxus::prelude::*;
// Views are used by the Router component
use routes::Route;

mod ui;
mod services;
mod io;
mod utils;
mod routes;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}

