use dioxus::prelude::*;
use document::eval;

#[component]
pub fn ProjectGrid() -> Element {
    rsx! {
        div {
            id: "projects",
            class: "container mx-auto px-4 py-12",
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
                    title: "HTML to Rsx",
                    description: "Convert HTML to Dioxus RSX with a single click.",
                    image: "https://devpro-aceternity.vercel.app/_next/image?url=%2Fimages%2Fprojects%2Falgochurn.png&w=3840&q=75",
                    tech_stack: vec!["Rust", "Dioxus", "Tailwind"],
                    link: "https://wheregmis.github.io/dioxus_html_rsx/"
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
    let title = props.title.to_lowercase().replace(" ", "-");

    rsx! {
        a {
            href: "{props.link}",
            target: "_blank",
            div {
                id: format!("project-card-{}", props.title.to_lowercase().replace(" ", "-")),
                class: "opacity-0 group h-[420px] flex flex-col relative overflow-hidden rounded-xl bg-gray-900/50 border border-gray-800 transition-all duration-300 hover:bg-gray-900/70 hover:border-gray-700 hover:shadow-xl hover:shadow-blue-500/20",
                onmounted: move |_| {
                    eval(&format!(r#"
                        const observer = new IntersectionObserver((entries) => {{
                            entries.forEach(entry => {{
                                if (entry.isIntersecting) {{
                                    Motion.animate('#project-card-{}', {{
                                        opacity: [0, 1],
                                        transform: ['translateY(20px)', 'translateY(0)']
                                    }}, {{
                                        duration: 0.6,
                                        easing: "spring(1, 100, 10, 0)"
                                    }});
                                    observer.unobserve(entry.target);
                                }}
                            }});
                        }}, {{ threshold: 0.1 }});

                        observer.observe(document.getElementById("project-card-{}"));
                    "#, 
                    title,
                    title
                    ));
                },
                // Rest of the card content remains the same
                div {
                    class: "relative h-48 overflow-hidden",
                    img {
                        class: "w-full h-full object-cover transition-transform duration-300 group-hover:scale-110",
                        src: "{props.image}",
                        alt: "{props.title}"
                    }
                }
                div {
                    class: "flex flex-col flex-1 p-6",
                    h3 {
                        class: "text-xl font-semibold text-white mb-2 group-hover:text-blue-400 transition-colors",
                        "{props.title}"
                    }
                    p {
                        class: "text-gray-400 mb-4 line-clamp-3",
                        "{props.description}"
                    }
                    div {
                        class: "mt-auto flex flex-wrap gap-2",
                        for tech in props.tech_stack.iter() {
                            span {
                                class: "px-3 py-1 text-xs rounded-full bg-gray-800 text-gray-300",
                                "{tech}"
                            }
                        }
                    }
                }
            }
        }
    }
}
