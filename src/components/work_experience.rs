use dioxus::prelude::*;
use document::eval;

#[component]
pub fn WorkExperience() -> Element {
    let mut selected_company = use_signal(|| 0);

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
    ];

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
                        onmounted: move |_| {
                            eval(
                                r#"
                                                                                                                                                                                                                    requestAnimationFrame(() => {
                                                                                                                                                                                                                        const container = document.querySelector('.bg-gray-800');
                                                                                                                                                                                                                        if (container) {
                                                                                                                                                                                                                            const height = container.offsetHeight;
                                                                                                                                                                                                                            Motion.animate('#timeline-dot', {
                                                                                                                                                                                                                                opacity: [0, 1],
                                                                                                                                                                                                                                y: [0, height]
                                                                                                                                                                                                                            }, {
                                                                                                                                                                                                                                duration: 2,
                                                                                                                                                                                                                                repeat: Infinity,
                                                                                                                                                                                                                                easing: "linear",
                                                                                                                                                                                                                                delay: 0
                                                                                                                                                                                                                            });
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    });
                                                                                                                                                                                                                "#,
                            );
                        }
                    }
                }
                // Companies list
                div { class: "space-y-8 pl-4",
                    for (index , (company , duration , _title , _description , _tech_stack)) in companies.iter().enumerate() {
                        div {
                            class: "relative cursor-pointer group",
                            onclick: move |_| selected_company.set(index),
                            div {
                                class: "absolute -left-[25px] top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-2 transition-colors duration-300",
                                class: if selected_company() == index {
                                    "bg-blue-500 border-blue-400"
                                } else {
                                    "bg-gray-800 border-gray-700"
                                }
                            }
                            div {
                                class: "p-4 rounded-lg transition-colors duration-300",
                                class: if selected_company() == index { "bg-gray-800/50" } else { "bg-transparent" },
                                h3 { class: "font-medium text-white", "{company}" }
                                p { class: "text-sm text-gray-400", "{duration}" }
                            }
                        }
                    }
                }
                // Experience details section with animations
                div {
                    id: "experience-details",
                    class: "bg-gray-900/50 rounded-xl p-6 border border-gray-800",
                    onmounted: move |_| {
                        eval(
                            r#"
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            const element = document.getElementById('experience-details');
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            if (element) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                Motion.animate(element, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    opacity: [0, 1],
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    transform: ['translateX(20px)', 'translateX(0)']
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    duration: 0.5,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    easing: 'ease-out'
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                });
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        "#,
                        );
                    },
                    {
                        let (company, duration, title, description, tech_stack) = &companies[selected_company()];
                        rsx! {
                            div {
                                class: "space-y-4",
                                h3 {
                                    id: "experience-title",
                                    class: "text-xl font-semibold text-white",
                                    onmounted: move |_| {
                                        eval(r#"
                                            Motion.animate('#experience-title', {
                                                opacity: [0, 1],
                                                transform: ['translateY(10px)', 'translateY(0)']
                                            }, {
                                                duration: 0.3,
                                                delay: 0.2
                                            });
                                        "#);
                                    },
                                    "{title}"
                                }
                                p {
                                    class: "text-gray-400",
                                    "{duration}"
                                }
                                p {
                                    class: "text-gray-300 leading-relaxed",
                                    "{description}"
                                }
                                div {
                                    class: "flex flex-wrap gap-2",
                                    for (index, tech) in tech_stack.iter().enumerate() {
                                        span {
                                            id: format!("tech-{}", index),
                                            class: "px-3 py-1 text-xs rounded-full bg-gray-800 text-gray-300 opacity-0", // Set initial opacity to 0
                                            onmounted: move |_| {
                                                eval(&format!(r#"
                                                    Motion.animate('#tech-{}', {{
                                                        opacity: [0, 1],
                                                        transform: ['scale(0.8)', 'scale(1)'],
                                                        y: [10, 0]
                                                    }}, {{
                                                        duration: 0.4,
                                                        delay: {},
                                                        easing: "spring(1, 100, 10, 0)"
                                                    }});
                                                "#, index, 0.2 + (index as f32 * 0.1)));
                                            },
                                            "{tech}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
