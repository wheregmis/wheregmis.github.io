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
            href: "https://cdn.jsdelivr.net/npm/daisyui@4.12.14/dist/full.min.css"
        }
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.4/css/all.min.css"
        }

        Router::<Route> {}
    }
}

// Home component - Main landing page container
#[component]
fn Home() -> Element {
    rsx! {
        // Main container div with Tailwind CSS classes for centering and max-width
        div { class: "container mx-auto",
            // Profile section - Displays personal/bio information
            Profile {}

            // Work Experience section - Shows professional history
            WorkExperience {}

            // Project Grid - Displays portfolio of projects
            ProjectGrid {}

            // Blog Preview - Shows recent blog posts
            BlogPreview {}

            // Testimonials section - Displays client/colleague feedback
            Testimonials {}

            // Footer component - Contains contact info and site navigation
            Footer {}
        }
    }
}
