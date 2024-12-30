use dioxus::prelude::*;
use dioxus_motion::{
    animation::Tween,
    prelude::*,
    use_transform_motion::{use_transform_animation, Transform},
};
use document::eval;
use easer::functions::Easing;
use svg_attributes::points;

use crate::PROFILE_PIC;

#[component]
pub fn Profile() -> Element {
    let mut image_transform = use_transform_animation(
        Transform {
            opacity: 0.0,
            scale: 0.8,
            rotate: -10.0,
            ..Default::default()
        },
        Transform::default(),
        AnimationMode::Spring(Spring {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
        }),
    );

    let points_labels: Vec<String> = vec![
        "Spearheading battery management innovation with Rust".to_string(),
        "Revolutionizing renewable energy through Rust-powered solutions".to_string(),
        "Dedicated to driving advancements in battery technology".to_string(),
    ];

    let point_animations = (0..3).map(|i: usize| {
        let mut transform = use_transform_animation(
            Transform {
                opacity: 0.0,
                x: -20.0,
                ..Default::default()
            },
            Transform::default(),
            AnimationMode::Tween(Tween {
                duration: Duration::from_millis(500 + i as u64 * 400),
                easing: easer::functions::Cubic::ease_out,
            }),
        );

        rsx! {
            p {
                class: "text-gray-400 leading-relaxed flex items-center space-x-2",
                style: "{transform.style()}",
                onmounted: move |_| {
                    transform.start();
                },
                span { class: "text-primary", "•" }
                span { {points_labels[i].clone()} }
            }
        }
    });

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

                        div { class: "mt-6 space-y-2", {point_animations} }
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
                            style: "{image_transform.style()}",
                            onmounted: move |_| {
                                image_transform.start();
                            },
                            // Glow effect with larger blur
                            div { class: "absolute inset-0 bg-gradient-to-r from-primary/0 via-primary/40 to-accent-purple/0 opacity-0 group-hover:opacity-100 transition-all duration-500 blur-2xl" }
                            // Image with separate zoom
                            img {
                                class: "w-full aspect-square object-cover border-4 border-surface-light/20 transition-all duration-500 group-hover:scale-105 group-hover:border-primary/50",
                                src: PROFILE_PIC,
                                alt: "Sabin Regmi",
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
