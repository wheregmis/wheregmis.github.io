#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
use wheregmis::ui_core::prelude::*;
#[rustfmt::skip]
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    // #[layout(LeftPanel)]
    #[layout(Nav)]
        #[route("/")]
        Home {},
        #[route("/blog")]
        BlogList {},
        #[route("/blog/:id")]
        BlogPost { id: String },
}

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    dioxus::launch(App);
}

#[component]
fn Nav() -> Element {
    rsx! {
        div { class: "w-full h-full items-start justify-start",
            Row {
                class: "w-full",
                main_axis_alignment: MainAxisAlignment::Start,
                cross_axis_alignment: CrossAxisAlignment::Start,
                LeftPanel {}
                Expanded { class: "w-full",
                    Header {}
                    div { class: "w-full h-full flex flex-row items-center justify-center",
                        div { class: "w-full h-full", Outlet::<Route> {} }
                    }
                }
                RightPanel {}
            }
        }
    }
}

fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[component]
fn Home() -> Element {
    rsx! { MainPanel {} }
}

// All Exported Components
pub use components::*;
mod components {
    pub mod blog;
    pub use blog::*;

    pub mod markdown;
    pub use markdown::*;

    pub mod left_panel;
    pub use left_panel::*;

    pub mod main_panel;
    pub use main_panel::*;

    pub mod right_panel;
    pub use right_panel::*;
}
pub mod blog_content;
