use dioxus::prelude::*;
use document::eval;

const PROFILE_PIC: Asset = asset!("/assets/pf.png");

#[component]
pub fn Profile() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 pt-20",
            div { class: "relative overflow-hidden rounded-xl bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900 p-8 border border-gray-800",
                // Gradient overlay
                div { class: "absolute inset-0 bg-gradient-to-br from-blue-500/10 via-purple-500/10 to-pink-500/10" }
                // Content container with flex
                div { class: "relative z-10 flex items-start justify-between gap-8",
                    // Left content
                    div { class: "flex-1",
                        // Heading
                        h1 { class: "text-4xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-purple-500",
                            "Software Engineer"
                        }
                        // Main description
                        p { class: "mt-4 text-xl text-gray-300 leading-relaxed",
                            "Pioneering the future of battery technology with Rust-powered solutions."
                        }
                        // Detailed description
                        div { class: "mt-6 space-y-2",
                            p {
                                id: "point-1",
                                class: "text-gray-400 leading-relaxed flex items-center space-x-2",
                                onmounted: move |_| {
                                    eval(
                                        r#"
    Motion.animate('#point-1', {
        opacity: [0, 1],
        x: [-20, 0]
    }, {
        duration: 0.5,
        delay: 0.7
    });
"#,
                                    );
                                },
                                span { class: "text-primary", "•" }
                                span { "Spearheading battery management innovation with Rust" }
                            }
                            p {
                                id: "point-2",
                                class: "text-gray-400 leading-relaxed flex items-center space-x-2",
                                onmounted: move |_| {
                                    eval(
                                        r#"
    Motion.animate('#point-2', {
        opacity: [0, 1],
        x: [-20, 0]
    }, {
        duration: 0.5,
        delay: 0.8
    });
"#,
                                    );
                                },
                                span { class: "text-primary", "•" }
                                span { "Revolutionizing renewable energy through Rust-powered solutions" }
                            }
                            p {
                                id: "point-3",
                                class: "text-gray-400 leading-relaxed flex items-center space-x-2",
                                onmounted: move |_| {
                                    eval(
                                        r#"
    Motion.animate('#point-3', {
        opacity: [0, 1],
        x: [-20, 0]
    }, {
        duration: 0.5,
        delay: 0.9
    });
"#,
                                    );
                                },
                                span { class: "text-primary", "•" }
                                span { "Dedicated to driving advancements in battery technology" }
                            }
                        }
                        // Current work
                        p { class: "mt-4 text-gray-400",
                            "Working with "
                            span { class: "text-blue-400 hover:text-blue-300 transition-colors",
                                "Rust"
                            }
                            " and "
                            span { class: "text-purple-400 hover:text-purple-300 transition-colors",
                                "Dioxus"
                            }
                            " when I'm not working on my day job."
                        }
                        // Social links
                        div { class: "mt-8 flex items-center space-x-4",
                            a {
                                class: "text-gray-400 hover:text-white transition-colors",
                                href: "https://github.com/yourusername",
                                i { class: "fab fa-github text-xl" }
                            }
                            a {
                                class: "text-gray-400 hover:text-white transition-colors",
                                href: "https://twitter.com/yourusername",
                                i { class: "fab fa-twitter text-xl" }
                            }
                            a {
                                class: "text-gray-400 hover:text-white transition-colors",
                                href: "https://linkedin.com/in/yourusername",
                                i { class: "fab fa-linkedin text-xl" }
                            }
                        }
                    }
                    // Right image
                    div { class: "w-[25%] flex-shrink-0 group -ml-48",
                        div {
                            id: "profile-image-container",
                            class: "relative overflow-hidden rounded-2xl transition-all duration-500 transform-gpu group-hover:scale-110",
                            onmounted: move |_| {
                                eval(
                                    r#"
                                                                Motion.animate('#profile-image-container', {
                                                                    opacity: [0, 1],
                                                                    scale: [0.8, 1],
                                                                    rotate: [-10, 0]
                                                                }, {
                                                                    duration: 0.8,
                                                                    delay: 2,
                                                                    easing: "spring(1, 100, 10, 0)"
                                                                });
                                                            "#,
                                );
                            },
                            // Glow effect with larger blur
                            div { class: "absolute inset-0 bg-gradient-to-r from-primary/0 via-primary/40 to-accent-purple/0 opacity-0 group-hover:opacity-100 transition-all duration-500 blur-2xl" }
                            // Image with separate zoom
                            img {
                                class: "w-full aspect-square object-cover border-4 border-surface-light/20 transition-all duration-500 group-hover:scale-105 group-hover:border-primary/50",
                                src: PROFILE_PIC,
                                alt: "Sabin Regmi"
                            }
                            // AI Generated text overlay
                            div { class: "absolute bottom-2 right-2 px-2 py-1 rounded-lg bg-black/50 backdrop-blur-sm text-xs text-white/70 group-hover:text-white/90 transition-colors",
                                "Apple Playground AI Generated"
                            }
                        }
                    }
                }
            }
        }
    }
}
