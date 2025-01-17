use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;
use std::time::Duration;

use crate::PROFILE_PIC;

#[component]
pub fn Profile() -> Element {
    // Main content animations
    let mut content_transform = use_motion(Transform::new(-20.0, 0.0, 0.8, 0.0));
    let mut content_opacity = use_motion(0.0f32);

    // Image animations
    let mut image_transform = use_motion(Transform::new(-20.0, 0.0, 0.8, 0.0));
    let mut opacity = use_motion(0.0f32);

    // Initial animation sequence on mount
    use_effect(move || {
        // Content entrance sequence
        let content_sequence = AnimationSequence::new().then(
            Transform::identity(),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 180.0,
                damping: 12.0,
                mass: 1.0,
                ..Default::default()
            })),
        );

        // Image entrance sequence
        let image_sequence = AnimationSequence::new().then(
            Transform::identity(),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 80.0,
                damping: 12.0,
                mass: 8.0,
                velocity: 10.0,
            })),
        );

        content_transform.animate_sequence(content_sequence);
        image_transform.animate_sequence(image_sequence);

        opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(100),
                easing: easer::functions::Sine::ease_in_out,
            })),
        );

        content_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(200),
                easing: easer::functions::Sine::ease_in_out,
            })),
        );
    });

    let points_labels = [
        "Spearheading battery management innovation with Rust",
        "Revolutionizing renewable energy through Rust-powered solutions",
        "Dedicated to driving advancements in battery technology",
    ];

    let point_animations = (0..3).map(|i| {
        let mut opacity = use_motion(0.0f32);

        use_effect(move || {
            let delay = Duration::from_millis(100 + i as u64 * 100);

            opacity.animate_to(
                1.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 15.0,
                    mass: 1.0,
                    ..Default::default()
                }))
                .with_delay(delay),
            );
        });

        rsx! {
            p {
                class: "text-text-muted leading-relaxed flex items-center space-x-2",
                style: "opacity: {opacity.get_value()};",
                span { class: "text-primary", "•" }
                span { "{points_labels[i]}" }
            }
        }
    });

    rsx! {
        div { class: "container mx-auto px-4 pt-20",
            div { class: "relative overflow-hidden rounded-xl bg-gradient-to-br from-surface via-surface-dark to-surface p-8 border border-surface-light",
                // Gradient overlay
                div { class: "absolute inset-0 bg-gradient-to-br from-primary/10 via-accent-purple/10 to-secondary/10" }
                // Content container with flex
                div { class: "relative z-10 flex items-start justify-between gap-8",
                    // Left content
                    div {
                        class: "flex-1",
                        style: "transform: translateX({content_transform.get_value().x}px) scale({content_transform.get_value().scale}); opacity: {content_opacity.get_value()};",
                        // Heading
                        h1 { class: "text-4xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-primary-light to-accent-purple",
                            "Software Engineer"
                        }
                        // Main description
                        p { class: "mt-4 text-xl text-text-secondary leading-relaxed",
                            "Pioneering the future of battery technology with Rust-powered solutions."
                        }

                        div { class: "mt-6 space-y-2", {point_animations} }
                        // Current work
                        p { class: "mt-4 text-text-muted",
                            "Working with "
                            span { class: "text-primary hover:text-primary-light transition-colors",
                                "Rust"
                            }
                            " and "
                            span { class: "text-accent-purple hover:text-accent-purple-hover transition-colors",
                                "Dioxus"
                            }
                            " when I'm not working on my day job."
                        }
                        // Social links
                        div { class: "mt-8 flex items-center space-x-4",
                            a {
                                class: "text-text-muted hover:text-text-primary transition-colors",
                                href: "https://github.com/yourusername",
                                i { class: "fab fa-github text-xl" }
                            }
                            a {
                                class: "text-text-muted hover:text-text-primary transition-colors",
                                href: "https://twitter.com/yourusername",
                                i { class: "fab fa-twitter text-xl" }
                            }
                            a {
                                class: "text-text-muted hover:text-text-primary transition-colors",
                                href: "https://linkedin.com/in/yourusername",
                                i { class: "fab fa-linkedin text-xl" }
                            }
                        }
                    }
                    // Right image
                    div { class: "w-[25%] flex-shrink-0 group -ml-48",
                        div {
                            id: "profile-image-container",
                            class: "relative overflow-hidden rounded-2xl transition-all duration-200 transform-gpu group-hover:scale-110",
                            style: "transform: translateX({image_transform.get_value().x}px) scale({image_transform.get_value().scale}); opacity: {opacity.get_value()};",
                            // Glow effect with larger blur
                            div { class: "absolute inset-0 bg-gradient-to-r from-primary/0 via-primary/40 to-accent-purple/0 opacity-0 group-hover:opacity-100 transition-all duration-500 blur-2xl" }
                            // Image with separate zoom
                            img {
                                class: "w-full aspect-square object-cover border-4 border-surface-light/20 transition-all duration-500 group-hover:scale-105 group-hover:border-primary/50",
                                src: PROFILE_PIC,
                                alt: "Sabin Regmi",
                            }
                            // AI Generated text overlay
                            div { class: "absolute bottom-2 right-2 px-2 py-1 rounded-lg bg-surface/50 backdrop-blur-sm text-xs text-text-muted group-hover:text-text-secondary transition-colors",
                                "Apple Playground AI Generated"
                            }
                        }
                    }
                }
            }
        }
    }
}
