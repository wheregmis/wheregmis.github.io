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
pub fn WorkExperience() -> Element {
    let mut selected_company = use_signal(|| 0);

    // Timeline dot animation
    let mut dot_transform = use_transform_animation(
        Transform {
            opacity: 0.0,
            y: 0.0,
            ..Default::default()
        },
        Transform {
            opacity: 1.0,
            y: 300.0,
            ..Default::default()
        },
        AnimationMode::Tween(Tween {
            duration: Duration::from_secs(2),
            easing: easer::functions::Linear::ease_in_out,
        }),
    );

    // Details panel animation
    let mut details_transform = use_transform_animation(
        Transform {
            opacity: 0.0,
            x: 20.0,
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

    let companies = [
        (
            "TROES Corp",
            "2021 - Present",
            "Software Engineer",
            "Led development of battery management systems and remote monitoring solutions, resulting in 40% efficiency improvement in data processing.",
            vec![
                "Rust", "Python", "AWS", "Redis", "Vue", "Tailwind", "Docker", "Grafana", "InfluxDB",
            ]
        ),
        (
            "Lambton College",
            "2021 - Present",
            "Research Assistant",
            "Led development of battery management systems and remote monitoring solutions, resulting in 40% efficiency improvement in data processing.",
            vec![
                "Rust", "Python", "AWS", "Redis", "Vue", "Tailwind", "Docker", "Grafana", "InfluxDB",
            ]
        ),
    ];

    let companies_comp = companies.iter().enumerate().map(|(index, (company, duration, _, _, _))| {
        let mut point_transform = use_transform_animation(
            Transform::default(),
            Transform { scale: 1.2, ..Default::default() },
            AnimationMode::Spring(Spring::default())
        );

        rsx! {
            div {
                class: "relative cursor-pointer group",
                onclick: move |_| {
                    selected_company.set(index);
                    point_transform.start();
                },
                // Point indicator
                div {
                    class: "absolute -left-[25px] top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-2 transition-colors duration-300",
                    class: if selected_company() == index { "bg-blue-500 border-blue-400" } else { "bg-gray-800 border-gray-700" },
                    style: "{point_transform.style()}",
                }
                // Company info
                div {
                    class: "p-4 rounded-lg transition-colors duration-300",
                    class: if selected_company() == index { "bg-gray-800/50" } else { "bg-transparent" },
                    h3 { class: "font-medium text-white", "{company}" }
                    p { class: "text-sm text-gray-400", "{duration}" }
                }
            }
        }
    });

    let mut title_transform = use_transform_animation(
        Transform {
            y: 10.0,
            opacity: 0.0,
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

    let experience_section = {
        let (_company, duration, title, description, tech_stack) = &companies[selected_company()];
        rsx! {
            div { class: "space-y-4",
                h3 {
                    class: "text-xl font-semibold text-white",
                    style: "{title_transform.style()}",
                    onmounted: move |_| title_transform.start(),
                    "{title}"
                }
                p { class: "text-gray-400", "{duration}" }
                p { class: "text-gray-300 leading-relaxed", "{description}" }
                div { class: "flex flex-wrap gap-2",
                    {
                        tech_stack
                            .iter()
                            .enumerate()
                            .map(|(index, tech)| {
                                let mut tech_transform = use_transform_animation(
                                    Transform {
                                        opacity: 0.0,
                                        scale: 0.8,
                                        y: 10.0,
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
                                rsx! {
                                    span {
                                        class: "px-3 py-1 text-xs rounded-full bg-gray-800 text-gray-300",
                                        style: "{tech_transform.style()}",
                                        onmounted: move |_| async move {
                                            let delay = Duration::from_millis((200.0 + (index as f32 * 100.0)) as u64);
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
    };

    rsx! {
        div { id: "experience", class: "container mx-auto px-4 py-12",
            h2 { class: "text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-purple-500 mb-8",
                "Work Experience"
            }
            div { class: "relative grid grid-cols-[1px_200px_1fr] gap-8",
                // Timeline line container
                div {
                    id: "timeline-line",
                    class: "relative h-[calc(100%-2rem)] bg-gray-800",
                    // Moving dot
                    div {
                        id: "timeline-dot",
                        class: "absolute w-2 h-2 bg-blue-500 rounded-full -left-[3px] opacity-0",
                        style: "{dot_transform.style()}",
                        onmounted: move |_| dot_transform.loop_animation(),
                    }
                }
                // Companies list
                div { class: "space-y-8 pl-4", {companies_comp} }
                // Experience details section with animations
                div {
                    id: "experience-details",
                    class: "bg-gray-900/50 rounded-xl p-6 border border-gray-800",
                    style: "{details_transform.style()}",
                    onmounted: move |_| details_transform.start(),
                    {experience_section}
                }
            }
        }
    }
}
