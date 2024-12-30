#![allow(non_snake_case)]

use components::{NavBar, Profile, ProjectGrid, Testimonials, WorkExperience};
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use document::eval;

use views::{Blog, BlogPreview, Footer};
mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},

        #[route("/blog/:id")]
        Blog { id: i32 },
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const PROFILE_PIC: Asset = asset!("/assets/pf.png");
fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {
        document::Title { "Sabin Regmi" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.4/css/all.min.css",
        }

        Router::<Route> {}
    }
}

// Home component - Main landing page container
#[component]
fn Home() -> Element {
    rsx! {
        div { class: "min-h-screen bg-background text-text-primary",
            // Main container
            // Hero/Profile section
            Profile {}

            // Work Experience section
            div { class: "mt-20", WorkExperience {} }

            // Project Grid
            div { class: "mt-20", ProjectGrid {} }

            // Blog Preview
            div { class: "mt-20", BlogPreview {} }

            // Testimonials
            div { class: "mt-20", Testimonials {} }

            // Footer
            div { class: "mt-20", Footer {} }
        }
    }
}
