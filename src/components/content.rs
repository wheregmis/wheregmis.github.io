use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::blog_content::ContentEntry;

#[component]
pub(crate) fn ContentPost(post: &'static ContentEntry) -> Element {
    rsx!(
        section { class: "py-20",
            div { class: "container px-4 mx-auto mb-16",
                div { class: "max-w-xl lg:max-w-2xl mx-auto text-center",
                    div { class: "flex mb-2 items-center justify-center",
                        a { class: "text-xs text-indigo-500", href: "#", "Home" }
                        icons::icon_0 {}
                        a { class: "text-xs text-indigo-500", href: "#", "Blog" }
                        icons::icon_1 {}
                        a { class: "text-xs text-indigo-500", href: "#", "Article" }
                    }
                    h2 { class: "mb-6 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-5xl lg:leading-tight font-bold font-heading",
                        "Lorem ipsum dolor sit amet consectutar domor at elis"
                    }
                    div { class: "flex items-center justify-center",
                        div { class: "mr-6",
                            img {
                                class: "w-16 h-16 object-cover rounded-full",
                                alt: "",
                                src: "plain-assets/images/indigo-500-avatar.png"
                            }
                        }
                        div {
                            h3 { class: "text-2xl font-bold font-heading", "Danny Bailey" }
                            p { class: "text-lg text-indigo-500", "February 26, 2021" }
                        }
                    }
                }
            }
            div { class: "h-96 mb-12 lg:mb-16",
                img {
                    class: "w-full h-full object-cover",
                    alt: "",
                    src: "plain-assets/images/indigo-600-horizontal.png"
                }
            }
            div { class: "container px-4 mx-auto",
                div { class: "max-w-2xl mx-auto",
                    p { class: "mb-6 lg:mb-8 text-lg leading-loose lg:text-xl lg:leading-relaxed text-gray-500",
                        "Building websites from wireframes that I had received. Some of those questions were:"
                    }
                    p { class: "mb-6 lg:mb-8 text-lg leading-loose lg:text-xl lg:leading-relaxed text-gray-500",
                        "These types of questions led me to miss numerous deadlines, and I wasted
        time and energy in back-and-forth communication. Sadly, this situation
        could have been avoided if the wireframes had provided enough detail."
                    }
                    p { class: "text-lg leading-loose lg:text-xl lg:leading-relaxed text-gray-500",
                        "Now that I am a UX designer, I notice that some designers tend to forget
        that wireframes are equally creative and technical. We are responsible
        for designing great ideas, but we are also responsible for creating
        product specifications. I admit that there can be so many details to
        remember that it’s easy to lose track. To save time and energy for
        myself, I gatheindigo all of my years of wireframing knowledge into a
        single checklist that I refer to throughout the process. And now I am
        sharing this knowledge with you, so that you can get back to being
        creative."
                    }
                }
            }
        }
    )
}

#[component]
pub fn ContentList(header: Element, content: &'static [ContentEntry], readmore: bool) -> Element {
    rsx!(
        section { class: "py-20",
            div { class: "container px-4 sm:px-12 lg:px-24 x-lg:px-48 mx-auto",
                {header},
                div { class: "flex flex-wrap -mx-4",
                    for item in content.iter() {
                        ContentPreview { item, readmore }
                    }
                }
            }
        }
    )
}

#[component]
fn ContentPreview(item: &'static ContentEntry, readmore: bool) -> Element {
    let ContentEntry {
        title,
        date,
        image,
        description,
        content,
        weight,
        slug,
        link,
        ..
    } = *item;

    let link = link.unwrap_or("");

    let content =
        use_signal(|| comrak::markdown_to_html(content, &comrak::ComrakOptions::default()));

    rsx!(
        div { class: "w-full px-4 lg:px-16 mb-12 lg:mb-16",
            Link { to: "{link}",
                div { class: "flex flex-wrap -mx-4",
                    div { class: "w-full lg:w-1/3 px-4 mb-6 lg:mb-0",
                        div {
                            img {
                                class: "h-40 lg:h-80 object-contain rounded-lg",
                                alt: "{description}",
                                src: "{image}"
                            }
                        }
                    }
                    div { class: "w-full lg:w-2/3 px-4",
                        // span { class: "text-xs font-bold text-indigo-500",
                        //     "{date}"
                        // }
                        h2 { class: "mt-2 mb-2 text-2xl font-bold font-heading", "{title}" }
                        // p { class: "mb-4 text-m text-gray-500 leading-loose",
                        //     "{description}"
                        // }
                        p {
                            class: "mb-4 text-m text-gray-500 leading-loose space-y-4",
                            dangerous_inner_html: "{content}"
                        }
                        if readmore {
                            Link {
                                to: format!("/{}/{slug}", item.archetype),
                                class: "flex items-center text-m font-bold text-indigo-500 hover:text-indigo-700",
                                span { "Read more" }
                                span { icons::icon_0 {} }
                            }
                        }
                    }
                }
            }
        }
    )
}

mod icons {
    use super::*;
    pub(super) fn icon_0() -> Element {
        rsx!(
            svg {
                class: "w-3 h-3 mx-1 text-indigo-500",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                stroke: "currentColor",
                fill: "none",
                path {
                    stroke_linejoin: "round",
                    stroke_linecap: "round",
                    stroke_width: "2",
                    d: "M9 5l7 7-7 7"
                }
            }
        )
    }
    pub(super) fn icon_1() -> Element {
        rsx!(
            svg {
                class: "w-3 h-3 mx-1 text-indigo-500",
                fill: "none",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    stroke_linecap: "round",
                    stroke_width: "2",
                    d: "M9 5l7 7-7 7",
                    stroke_linejoin: "round"
                }
            }
        )
    }
}
