use dioxus::prelude::*;

use crate::{Route, BLOG_PREVIEW, PROFILE_ELEMENT, PROJECT_GRID, WORKEXPERIENCE};

#[component]
pub fn NavBar() -> Element {
    let path: Route = use_route();

    let path_string = use_signal(|| path.to_string());

    let is_blog = path.to_string().starts_with("/blo");

    rsx! {
        div { class: "w-full h-full bg-background text-text-secondary",
            header { class: "fixed top-0 w-full z-50 h-16 backdrop-blur-md border-b border-surface-light/20",
                div { class: "container mx-auto h-full px-4",
                    div { class: "flex items-center justify-between h-full",
                        // Left side - Logo and name
                        div { class: "flex items-center space-x-3",
                            div { class: "flex items-center gap-8 px-6 py-2 bg-surface/50 backdrop-blur-sm border border-surface-light/10 rounded-full shadow-lg shadow-background/5",
                                svg {
                                    class: "w-8 h-8 text-text-secondary",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    "stroke-width": "1.5",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5",
                                    }
                                }
                                Link {
                                    to: Route::Home {},
                                    onclick: move |evt: Event<MouseData>| async move {
                                        if path_string() == *"/" {
                                            evt.prevent_default();
                                            if let Some(header) = PROFILE_ELEMENT.cloned() {
                                                let _ = header.scroll_to(ScrollBehavior::Smooth).await;
                                            }
                                        }
                                    },
                                    h1 { class: "text-lg font-semibold text-text-primary hover:text-primary transition-colors cursor-pointer",
                                        "Sabin Regmi"
                                    }
                                }
                                if !is_blog {
                                    nav { class: "hidden md:flex items-center space-x-6",
                                        for link in ["Experience", "Projects", "Blogs"].iter() {
                                            a {
                                                class: "text-sm text-text-secondary hover:text-text-primary transition-colors relative group cursor-pointer",
                                                onclick: move |_| async move {
                                                    match *link {
                                                        "Experience" => {
                                                            if let Some(header) = WORKEXPERIENCE.cloned() {
                                                                let _ = header.scroll_to(ScrollBehavior::Smooth).await;
                                                            }
                                                        }
                                                        "Projects" => {
                                                            if let Some(header) = PROJECT_GRID.cloned() {
                                                                let _ = header.scroll_to(ScrollBehavior::Smooth).await;
                                                            }
                                                        }
                                                        "Blogs" => {
                                                            if let Some(header) = BLOG_PREVIEW.cloned() {
                                                                let _ = header.scroll_to(ScrollBehavior::Smooth).await;
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                },
                                                "{link}"
                                                div { class: "absolute -bottom-1 left-0 h-[2px] w-0 bg-primary transition-all group-hover:w-full" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Outlet::<Route> {}
        }
    }
}
