use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn WorkExperience() -> Element {
    let mut selected_company = use_signal(|| 0);

    // Timeline dot animation using new API
    let mut dot_transform = use_motion(Transform::identity());

    // Details panel animation using new API
    let mut details_transform = use_motion(Transform::identity());

    let mut animate_details = move |_| {
        details_transform.animate_to(
            Transform::new(0.0, 0.0, 1.0, 0.0),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 15.0,
                mass: 1.0,
                ..Default::default()
            })),
        );
    };

    let companies = [
        (
            "TROES Corp",
            "May, 2023 - Present",
            "Software Engineer",
            "Developed high-performance battery management systems and remote monitoring solutions, achieving 40% improvement in data processing efficiency. Built robust battery controllers using Rust, with real-time monitoring through InfluxDB and Grafana. Designed scalable web platforms using Django and Vue.js for seamless IoT energy management.",
            vec![
                "Rust", "Python", "AWS", "Redis", "Vue", "Tailwind", "Docker", "Grafana", "InfluxDB",
            ]
        ),
        (
            "Lambton College",
            "Aug, 2022 - April, 2023",
            "Research Assistant",
            "Focused on developing remote monitoring systems for battery energy storage using Flutter, Vue.js, and Django. Designed and implemented web and mobile applications to track system performance, enabling real-time data access and improving overall monitoring efficiency.",
            vec![
                "Python", "AWS", "Vue", "Tailwind", "Docker", "InfluxDB",
            ]
        ),
        (
            "Seva Development",
            "Oct, 2021 - Jan, 2022",
            "Contract Based Software Engineer",
            "Developed a data migration engine as a Software Engineer, enabling seamless data transfer between diverse sources and destinations, including MySQL, Oracle, PostgreSQL, and Salesforce. Optimized the migration process for accuracy and efficiency, ensuring reliable data handling across multiple platforms.",
            vec![
                "Python", "AWS Lambda","PostgreSQL",
            ]
        ),
    ];

    let companies_comp = companies.iter().enumerate().map(|(index, (company, duration, _, _, _))| {
        let mut point_transform = use_motion(Transform::identity());

        let animate_hover = move |_| {
            point_transform.animate_to(
                Transform::new(0.0, 0.0, 1.2, 0.0),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 180.0,
                    damping: 12.0,
                    mass: 1.0,
                    ..Default::default()
                })),
            );
        };

        let animate_reset = move |_| {
            point_transform.animate_to(
                Transform::identity(),
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );
        };

        rsx! {
            div {
                class: "relative cursor-pointer group",
                onmouseenter: animate_hover,
                onmouseleave: animate_reset,
                onclick: move |_| {
                    selected_company.set(index);
                    animate_details(());
                },
                // Point indicator with transform
                div {
                    class: "absolute -left-[25px] top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-2",
                    class: if selected_company() == index { "bg-blue-500 border-blue-400" } else { "bg-gray-800 border-gray-700" },
                    style: "transform: scale({point_transform.get_value().scale});",
                }
                // Company info
                div {
                    class: "p-4 rounded-lg transition-colors duration-300",
                    class: if selected_company() == index { "bg-surface/50" } else { "bg-transparent" },
                    h3 { class: "font-medium text-text-primary", "{company}" }
                    p { class: "text-sm text-text-muted", "{duration}" }
                }
            }
        }
    });

    let experience_section = {
        let (_company, duration, title, description, tech_stack) = &companies[selected_company()];
        rsx! {
            div { class: "space-y-4",
                h3 {
                    class: "text-xl font-semibold text-white",
                    style: "transform: none",
                    onmounted: move |_| {},
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
                                let mut tech_transform = use_motion(Transform {
                                    scale: 0.8,
                                    y: 10.0,
                                    x: (index as f32 * 10.0),
                                    rotation: 0.0,
                                });
                                rsx! {
                                    span {
                                        class: "px-3 py-1 text-xs rounded-full bg-gray-800 text-gray-300",
                                        style: "transform: translate({tech_transform.get_value().x}px, {tech_transform.get_value().y}px) scale({tech_transform.get_value().scale}) rotate({tech_transform.get_value().rotation}deg);",
                                        onmounted: move |_| async move {
                                            let delay = Duration::from_millis((200.0 + (index as f32 * 100.0)) as u64);
                                            Time::delay(delay).await;
                                            tech_transform
                                                .animate_to(
                                                    Transform::identity(),
                                                    AnimationConfig::new(
                                                        AnimationMode::Spring(Spring {
                                                            stiffness: 100.0,
                                                            damping: 15.0,
                                                            mass: 1.0,
                                                            velocity: 0.0,
                                                        }),
                                                    ),
                                                );
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
                        class: "absolute w-2 h-2 bg-blue-500 rounded-full -left-[3px]",
                        style: "transform: translateY({dot_transform.get_value().y}px);",
                        onmounted: move |_| {
                            dot_transform
                                .animate_to(
                                    Transform::new(0.0, 200.0, 1.0, 0.0),
                                    AnimationConfig::new(
                                            AnimationMode::Spring(Spring {
                                                stiffness: 100.0,
                                                damping: 15.0,
                                                mass: 1.0,
                                                velocity: 10.0,
                                            }),
                                        )
                                        .with_loop(LoopMode::Infinite),
                                );
                        },
                    }
                }
                // Companies list
                div { class: "space-y-8 pl-4", {companies_comp} }
                // Experience details section with animations
                div {
                    id: "experience-details",
                    class: "bg-gray-900/50 rounded-xl p-6 border border-gray-800",
                    style: "transform: scale({details_transform.get_value().scale});",
                    onmounted: move |_| {
                        details_transform
                            .animate_to(
                                Transform::new(0.0, 0.0, 1.0, 0.0),
                                AnimationConfig::new(
                                    AnimationMode::Spring(Spring {
                                        stiffness: 100.0,
                                        damping: 15.0,
                                        mass: 1.0,
                                        velocity: 0.0,
                                    }),
                                ),
                            );
                    },
                    {experience_section}
                }
            }
        }
    }
}
