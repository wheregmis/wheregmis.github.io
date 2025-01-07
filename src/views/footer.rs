use dioxus::prelude::*;

use crate::{BLOG_PREVIEW, PROFILE_ELEMENT, PROJECT_GRID, WORKEXPERIENCE};

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "w-full mt-auto bg-gradient-to-t from-black to-bg-background border-t border-surface-light/20",
            div { class: "container mx-auto px-4 py-12",
                // create a divider for the footer
                div { class: "border-b border-surface-light/50 mb-12" }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                    // Logo and description

                    div { class: "space-y-4",
                        h3 { class: "text-xl font-semibold bg-clip-text text-transparent bg-gradient-to-r from-gray-100 to-gray-400",
                            "Sabin Regmi"
                        }
                        p { class: "text-gray-400", "I will update this section soon !!!" }
                    }
                    // Quick links
                    div { class: "space-y-4",
                        h4 { class: "text-white font-medium", "Quick Links" }
                        div { class: "flex flex-col space-y-2",
                            for link in ["About", "Projects", "Blogs"].iter() {
                                a {
                                    class: "text-sm text-text-secondary hover:text-text-primary transition-colors relative group cursor-pointer",
                                    onclick: move |_| async move {
                                        match *link {
                                            "About" => {
                                                if let Some(header) = PROFILE_ELEMENT.cloned() {
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

                                }
                            }
                        }
                    }
                    // Social links
                    div { class: "space-y-4",
                        h4 { class: "text-white font-medium", "Connect" }
                        div { class: "flex space-x-4",
                            a {
                                class: "text-gray-400 hover:text-white transition-colors",
                                href: "https://github.com/wheregmis",
                                i { class: "fab fa-github text-xl" }
                            }
                            a {
                                class: "text-gray-400 hover:text-white transition-colors",
                                href: "https://twitter.com/wheregmis",
                                i { class: "fab fa-twitter text-xl" }
                            }
                            a {
                                class: "text-gray-400 hover:text-white transition-colors",
                                href: "https://linkedin.com/in/wheregmis",
                                i { class: "fab fa-linkedin text-xl" }
                            }
                        }
                    }
                }
                // Copyright
                div { class: "mt-12 pt-8 border-t border-gray-800 text-center text-gray-400",
                    "© 2024 Sabin Regmi"
                    div { class: "text-center text-gray-400",
                        "Built with ❤️ using Dioxus"
                    }
                }

            }
        }
    }
}
