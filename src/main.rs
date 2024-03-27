#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
use wheregmis::ui_core::prelude::*;
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog")]
    BlogList,
    #[route("/blog/:id")]
    BlogPost { id: String },
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "w-full h-full items-start justify-start",
            Row {
                LeftPanel {}
                MainPanel {}
                RightPanel {}
            }
        }
    }
}

#[component]
fn MainPanel() -> Element {
    rsx! {
        Expanded { class: "bg-slate-900 w-2/4 h-full", BlogList {} }
    }
}

#[component]
fn RightPanel() -> Element {
    rsx! {
        Expanded { class: "bg-yellow-500 w-1/4 lg:block hidden items-start justify-start", h1 { class: "title", "Right" } }
    }
}

// All Exported Components
pub use components::*;
mod components {
    pub mod blog;
    pub use blog::*;

    pub mod content;
    pub use content::*;

    pub mod markdown;
    pub use markdown::*;

    pub mod left_panel;
    pub use left_panel::*;
}
pub mod blog_content;
