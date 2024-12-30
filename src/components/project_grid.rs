use dioxus::prelude::*;
use document::eval;

use dioxus_motion::{
    animation::Tween,
    platform::TimeProvider,
    prelude::*,
    use_transform_motion::{use_transform_animation, Transform},
    Time,
};
use easer::functions::Easing;

#[component]
pub fn ProjectGrid() -> Element {
    rsx! {
        div { id: "projects", class: "container mx-auto px-4 py-12",
            // Section header
            h2 { class: "text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-purple-500 mb-2",
                "Featured Projects"
            }
            p { class: "text-gray-400 mb-8",
                "Some things I've built to make the world a better place"
            }
            // Project grid
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                ProjectCard {
                    title: "Dioxus Motion 🚀",
                    description: "A lightweight, cross-platform animation library for Dioxus, designed to bring smooth, flexible animations to your Rust web, desktop, and mobile applications.",
                    image: "https://devpro-aceternity.vercel.app/_next/image?url=%2Fimages%2Fprojects%2Falgochurn.png&w=3840&q=75",
                    tech_stack: vec!["Rust", "Dioxus", "Tailwind"],
                    link: "https://crates.io/crates/dioxus-motion",
                }
                ProjectCard {
                    title: "HTML to RSX Converter",
                    description: "Convert HTML to Dioxus RSX with a single click.",
                    image: "https://devpro-aceternity.vercel.app/_next/image?url=%2Fimages%2Fprojects%2Falgochurn.png&w=3840&q=75",
                    tech_stack: vec!["Rust", "Dioxus", "Tailwind"],
                    link: "https://wheregmis.github.io/dioxus_html_rsx/",
                }

            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct ProjectCardProps {
    title: String,
    description: String,
    image: String,
    tech_stack: Vec<&'static str>,
    link: String,
}
#[component]
fn ProjectCard(props: ProjectCardProps) -> Element {
    let is_visible = use_signal(|| false);

    let mut card_transform = use_transform_animation(
        Transform {
            opacity: 0.0,
            y: 20.0,
            ..Default::default()
        },
        Transform::default(),
        AnimationMode::Spring(Spring {
            stiffness: 100.0,
            damping: 15.0,
            mass: 1.0,
            velocity: 0.0,
        }),
    );

    let mut image_transform = use_transform_animation(
        Transform::default(),
        Transform {
            scale: 1.1,
            ..Default::default()
        },
        AnimationMode::Spring(Spring {
            stiffness: 200.0,
            damping: 20.0,
            mass: 1.0,
            velocity: 0.0,
        }),
    );

    rsx! {
        a { href: "{props.link}", target: "_blank",
            div {
                class: "group h-[420px] flex flex-col relative overflow-hidden rounded-xl bg-gray-900/50 border border-gray-800 transition-all duration-300 hover:bg-gray-900/70 hover:border-gray-700 hover:shadow-xl hover:shadow-blue-500/20",
                style: "{card_transform.style()}",
                onmounted: move |_| card_transform.start(),

                div {
                    class: "relative h-48 overflow-hidden",
                    onmouseenter: move |_| image_transform.start(),
                    onmouseleave: move |_| image_transform.reset(),

                    img {
                        class: "w-full h-full object-cover",
                        style: "{image_transform.style()}",
                        src: "{props.image}",
                        alt: "{props.title}",
                    }
                }

                div { class: "flex flex-col flex-1 p-6",
                    h3 { class: "text-xl font-semibold text-white mb-2 group-hover:text-blue-400 transition-colors",
                        "{props.title}"
                    }
                    p { class: "text-gray-400 mb-4 line-clamp-3", "{props.description}" }
                    div { class: "mt-auto flex flex-wrap gap-2",
                        {
                            props
                                .tech_stack
                                .iter()
                                .enumerate()
                                .map(|(index, tech)| {
                                    let mut tech_transform = use_transform_animation(
                                        Transform {
                                            opacity: 0.0,
                                            y: 10.0,
                                            ..Default::default()
                                        },
                                        Transform::default(),
                                        AnimationMode::Tween(Tween {
                                            duration: Duration::from_millis(300),
                                            easing: easer::functions::Cubic::ease_out,
                                        }),
                                    );
                                    rsx! {
                                        span {
                                            class: "px-3 py-1 text-xs rounded-full bg-gray-800 text-gray-300",
                                            style: "{tech_transform.style()}",
                                            onmounted: move |_| async move {
                                                let delay = Duration::from_millis((500.0 + (index as f32 * 100.0)) as u64);
                                                Time::delay(delay).await;
                                                tech_transform.start();
                                            },
                                            "{tech}"
                                        }
                                    }
                                })
                        }
                    }
                }
            }
        }
    }
}
