use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "mt-auto bg-gradient-to-b from-gray-900 to-black border-t border-gray-800",
            div { class: "container mx-auto px-4 py-12",
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
                            for link in ["About", "Projects", "Blog", "Contact"].iter() {
                                a {
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    href: "#",
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
                    "© 2024 Sabin Regmi. All rights reserved. Thanks to ChatGPT for sucking in RUST"
                }
            }
        }
    }
}
