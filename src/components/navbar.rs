use dioxus::prelude::*;
use document::eval;

use crate::Route;

#[component]
pub fn NavBar() -> Element {
    let path: Route = use_route();
    let mut show_modal = use_signal(|| false);

    let is_blog = path.to_string().starts_with("/blo");

    rsx! {
        div {
            class: "w-full h-full bg-background text-text-secondary",
            header {
                class: "fixed top-0 w-full z-50 h-16 backdrop-blur-md border-b border-surface-light/20",
                div {
                    class: "container mx-auto h-full px-4",
                    div {
                        class: "flex items-center justify-between h-full",
                        // Left side - Logo and name
                        div {
                            class: "flex items-center space-x-3",
                            div {
                                class: "flex items-center gap-8 px-6 py-2 bg-surface/50 backdrop-blur-sm border border-surface-light/10 rounded-full shadow-lg shadow-background/5",
                                svg {
                                    class: "w-8 h-8 text-text-secondary",
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
                                Link {
                                    to: Route::Home {},
                                    onclick: move |evt: Event<MouseData>| {
                                        if path.to_string() == "/" {
                                            evt.prevent_default();
                                            eval(r#"
                                                window.scrollTo({
                                                    top: 0,
                                                    behavior: 'smooth'
                                                });
                                            "#);
                                        }
                                    },
                                    h1 {
                                        class: "text-lg font-semibold text-text-primary hover:text-primary transition-colors cursor-pointer",
                                        "Sabin Regmi"
                                    }
                                }
                                if !is_blog {
                                    nav {
                                        class: "hidden md:flex items-center space-x-6",
                                        for link in ["Experience", "Projects", "Blogs"].iter() {
                                            a {
                                                class: "text-sm text-text-secondary hover:text-text-primary transition-colors relative group cursor-pointer",
                                                onclick: move |evt| {
                                                    evt.prevent_default();
                                                    eval(
                                                        &format!(
                                                            r#"
const element = document.getElementById('{}');
if (element) {{
    const offset = 60;  // Adjust this value for desired offset
    const elementPosition = element.getBoundingClientRect().top + window.pageYOffset;
    window.scrollTo({{
        top: elementPosition - offset,
        behavior: 'smooth'
    }});
}}
                                                            "#,
                                                            link.to_lowercase(),
                                                        ),
                                                    );
                                                },
                                                "{link}"
                                                div {
                                                    class: "absolute -bottom-1 left-0 h-[2px] w-0 bg-primary transition-all group-hover:w-full"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if !is_blog {
                            button {
                                class: "ml-6 px-4 py-1.5 text-sm font-medium bg-surface hover:bg-surface-hover text-text-primary rounded-full transition-colors border border-surface-light/20",
                                onclick: move |_| show_modal.set(true),
                                "Download CV"
                            }
                        }
                    }
                }
            }
            // Modal
            if show_modal() {
                div {
                    class: "fixed inset-0 flex items-center justify-center bg-background/80 backdrop-blur-sm z-50",
                    onclick: move |_| show_modal.set(false),
                    div {
                        class: "relative w-full mx-4 md:max-w-2xl bg-surface p-8 rounded-xl border border-surface-light/20 shadow-xl",
                        onclick: move |e| e.stop_propagation(),
                        onmounted: move |_| {
                            eval(r#"
                                Motion.animate('.modal-content', {
                                    opacity: [0, 1],
                                    scale: [0.9, 1]
                                }, {
                                    duration: 0.3,
                                    easing: 'ease-out'
                                });
                            "#);
                        },
                        p {
                            class: "text-xl text-center text-text-primary font-medium mb-6",
                            "You still need my CV? Unbelievable! 😂"
                        }
                        div {
                            class: "flex justify-center",
                            button {
                                class: "px-6 py-2 text-sm bg-surface-light hover:bg-primary text-text-primary rounded-lg transition-colors",
                                onclick: move |_| show_modal.set(false),
                                "Sorry!"
                            }
                        }
                    }
                }
            }
            Outlet::<Route> {}
        }
    }
}
