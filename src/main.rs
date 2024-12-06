#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use document::eval;

use crate::document::Script;
use views::{Blog, BlogPreview, Footer};
mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},

        #[route("/blog/:id")]
        Blog { id: i32 },
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {
        document::Title { "Sabin Regmi" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/daisyui@4.12.14/dist/full.min.css"
        }
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.4/css/all.min.css"
        }

        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "container mx-auto",
            Profile {}
            WorkExperience {}
            ProjectGrid {}
            BlogPreview {}
            Testimonials {}
            Footer {}
        }
    }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        div { class: "w-full h-full flex flex-col items-center justify-center",
            header { class: "fixed top-0 w-full z-50 h-16  backdrop-blur-md border-b border-gray-800",
                div { class: "container mx-auto h-full px-4",
                    div { class: "flex items-center justify-between h-full",
                        // Left side - Logo and name
                        // Left side - Logo and name
                        div { class: "flex items-center space-x-3",
                            // Container for name and nav
                            div { class: "flex items-center gap-8 px-6 py-2 bg-gray-900/50 backdrop-blur-sm border border-gray-800/50 rounded-full shadow-lg shadow-gray-900/20",
                                svg {
                                    class: "w-8 h-8 text-gray-300",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    "stroke-width": "1.5",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5"
                                    }
                                }
                                h1 { class: "text-lg font-semibold bg-clip-text text-transparent bg-gradient-to-r from-gray-100 to-gray-400",
                                    "Sabin Regmi"
                                }
                                // Center - Navigation
                                nav { class: "hidden md:flex items-center space-x-6",
                                    for link in ["Experience", "Projects", "Blogs"].iter() {
                                        a {
                                            class: "text-sm text-gray-300 hover:text-white transition-colors relative group",
                                            href: "#",
                                            "{link}"
                                            // Hover line effect
                                            div { class: "absolute -bottom-1 left-0 h-[2px] w-0 bg-blue-500 transition-all group-hover:w-full" }
                                        }
                                    }
                                }
                            }
                        }
                        script { src: "https://cdn.jsdelivr.net/npm/motion@11.11.13/dist/motion.js" }

                        // create a div with id box, and we will animate it using motion
                        // div {

                        //     id: "box",
                        //     class: "w-8 h-8 bg-blue-500 box",
                        //     onmounted: move |_| {
                        //         eval(
                        //             r#"
                        //                                                                                                                                         const { animate, scroll } = Motion
                        //                                                                                                                                         animate('#box', {
                        //                                                                                                                                             x: 200,
                        //                                                                                                                                             rotate: 360,
                        //                                                                                                                                             duration: 10,
                        //                                                                                                                                             loop: true,
                        //                                                                                                                                             direction: "alternate"
                        //                                                                                                                                         })
                        //                                                                                                                                                                                                                                                                                                                                  "#,
                        //         );
                        //     }
                        // }

                        // Right side - Download CV
                        button { class: "ml-6 px-4 py-1.5 text-sm font-medium bg-gray-600 hover:bg-gray-900 text-white rounded-full transition-colors",
                            "Download CV"
                        }
                    }
                }
            }

            div { class: "container mx-auto", Outlet::<Route> {} }
        }
    }
}

#[component]
fn Profile() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 pt-20",
            div { class: "relative overflow-hidden rounded-xl bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900 p-8 border border-gray-800",
                // Gradient overlay
                div { class: "absolute inset-0 bg-gradient-to-br from-blue-500/10 via-purple-500/10 to-pink-500/10" }
                // Content container
                div { class: "relative z-10",
                    // Heading
                    h1 { class: "text-4xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-purple-500",
                        "Software Engineer"
                    }
                    // Main description
                    p { class: "mt-4 text-xl text-gray-300 leading-relaxed",
                        "I'm a software engineer that rarely writes code."
                    }
                    // Detailed description
                    p { class: "mt-6 text-gray-400 leading-relaxed",
                        "Meet Sabin Regmi, the self-proclaimed code wizard who can turn caffeine into beautiful websites. His passion for building clean and functional designs is only rivaled by his excitement for finding the perfect GIF to express his enthusiasm."
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
            }
        }
    }
}
#[component]
fn WorkExperience() -> Element {
    let mut selected_company = use_signal(|| 0);

    let companies = [
        ("Company A", "2021 - Present", "Senior Software Engineer"),
        ("Company B", "2019 - 2021", "Software Engineer"),
        ("Company C", "2018 - 2019", "Junior Developer"),
    ];

    rsx! {
        div { class: "container mx-auto px-4 py-12",
            h2 { class: "text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-purple-500 mb-8",
                "Work Experience"
            }
            div { class: "relative grid grid-cols-[1px_200px_1fr] gap-8",
                // Timeline line container
                div { class: "relative h-[calc(100%-2rem)] bg-gray-800",
                    // Moving dot
                    div { class: "absolute w-2 h-2 bg-blue-500 rounded-full -left-[3px] animate-move-down" }
                }
                // Companies list
                div { class: "space-y-8 pl-4",
                    for (index , (company , duration , _)) in companies.iter().enumerate() {
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
                // Experience details
                div { class: "bg-gray-900/50 rounded-xl p-6 border border-gray-800",
                    {
                        let (company, duration, title) = companies[selected_company()];
                        rsx! {
                            div {
                                class: "space-y-4",
                                h3 { class: "text-xl font-semibold text-white", "{title} at {company}" }
                                p { class: "text-gray-400", "{duration}" }
                                p {
                                    class: "text-gray-300 leading-relaxed",
                                    "Led development of key features resulting in 40% user growth."
                                }
                                div {
                                    class: "flex flex-wrap gap-2",
                                    for tech in ["Rust", "TypeScript", "React", "AWS"].iter() {
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
        }
    }
}

#[component]
fn ProjectGrid() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 py-12",
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
                    title: "Algochurn",
                    description: "A web app that allows users to practice for front-end and UI interviews.",
                    image: "https://devpro-aceternity.vercel.app/_next/image?url=%2Fimages%2Fprojects%2Falgochurn.png&w=3840&q=75",
                    tech_stack: vec!["React", "TypeScript", "Tailwind"],
                    link: "https://algochurn.com"
                }
                ProjectCard {
                    title: "Acernity",
                    description: "A web design and development agency that gets the job done. Somehow.",
                    image: "https://devpro-aceternity.vercel.app/_next/image?url=%2Fimages%2Fprojects%2Falgochurn.png&w=3840&q=75",
                    tech_stack: vec!["Next.js", "GraphQL", "Prisma"],
                    link: "https://acernity.com"
                }
                ProjectCard {
                    title: "Tailwind Master Kit",
                    description: "Buy premium tailwind components and templates for your next project.",
                    image: "https://devpro-aceternity.vercel.app/_next/image?url=%2Fimages%2Fprojects%2Falgochurn.png&w=3840&q=75",
                    tech_stack: vec!["Vue", "Node.js", "MongoDB"],
                    link: "https://tailwindmasterkit.com"
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
    rsx! {
        a { href: "{props.link}", target: "_blank",
            div { class: "group h-[420px] flex flex-col relative overflow-hidden rounded-xl bg-gray-900/50 border border-gray-800 transition-all duration-300 hover:bg-gray-900/70 hover:border-gray-700 hover:shadow-xl hover:shadow-blue-500/20",
                // Image container with fixed height
                div { class: "relative h-48 overflow-hidden",
                    img {
                        class: "w-full h-full object-cover transition-transform duration-300 group-hover:scale-110",
                        src: "{props.image}",
                        alt: "{props.title}"
                    }
                }
                // Content container with fixed layout
                div { class: "flex flex-col flex-1 p-6",
                    h3 { class: "text-xl font-semibold text-white mb-2 group-hover:text-blue-400 transition-colors",
                        "{props.title}"
                    }
                    p { class: "text-gray-400 mb-4 line-clamp-3", "{props.description}" }
                    // Tech stack pushed to bottom
                    div { class: "mt-auto flex flex-wrap gap-2",
                        for tech in props.tech_stack.iter() {
                            span { class: "px-3 py-1 text-xs rounded-full bg-gray-800 text-gray-300",
                                "{tech}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct TestimonialProps {
    name: String,
    role: String,
    company: String,
    avatar: String,
    quote: String,
}

#[component]
fn Testimonials() -> Element {
    let testimonials = vec![
        TestimonialProps {
            name: "John Doe".into(),
            role: "CEO".into(),
            company: "TechCorp".into(),
            avatar: "https://api.uifaces.co/our-content/donated/xZ4wg2Xj.jpg".into(),
            quote: "Working with Sabin was an absolute pleasure. His attention to detail and technical expertise helped us achieve our goals.".into(),
        },
        TestimonialProps {
            name: "Jane Smith".into(),
            role: "CTO".into(),
            company: "DevInc".into(),
            avatar: "https://randomuser.me/api/portraits/women/79.jpg".into(),
            quote: "One of the most talented developers I've worked with. Delivers high-quality code consistently.".into(),
        },
    ];

    rsx! {
        div { class: "container mx-auto px-4 py-12",
            // Section header
            h2 { class: "text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-purple-500 mb-2 text-center",
                "What People Say"
            }
            p { class: "text-gray-400 mb-12 text-center", "Testimonials from people I've worked with" }
            // Testimonials grid
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                for testimonial in testimonials {
                    div { class: "group relative overflow-hidden rounded-xl bg-gray-900/50 border border-gray-800 p-6 transition-all duration-300 hover:bg-gray-900/70 hover:border-gray-700",
                        // Quote
                        p { class: "text-gray-300 mb-6 italic",
                            ""
                            {testimonial.quote},
                            ""
                        }
                        // Author info
                        div { class: "flex items-center space-x-4",
                            img {
                                class: "w-12 h-12 rounded-full object-cover",
                                src: "{testimonial.avatar}",
                                alt: "{testimonial.name}"
                            }
                            div {
                                h4 { class: "text-white font-medium", "{testimonial.name}" }
                                p { class: "text-sm text-gray-400",
                                    "{testimonial.role} at {testimonial.company}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
