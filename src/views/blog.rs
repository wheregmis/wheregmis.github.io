use crate::{components::Markdown, Route};
use dioxus::prelude::*;
use serde::Deserialize;

use include_dir::{include_dir, Dir};

static BLOG_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/content/blog");

#[derive(Clone, Deserialize)]
struct Frontmatter {
    title: String,
    date: String,
    description: String,
    image: String,
    tags: Vec<String>,
}

#[derive(Clone, Debug)]
struct BlogPost {
    id: i32,
    title: String,
    description: String,
    date: String,
    content: String,
    image: String,
    tags: Vec<String>,
}

impl BlogPost {
    fn all() -> Vec<Self> {
        BLOG_DIR
            .files()
            .filter(|f| f.path().extension().unwrap_or_default() == "md")
            .enumerate()
            .map(|(id, file)| {
                let content = file.contents_utf8().unwrap();
                let parts: Vec<&str> = content.split("+++").collect();
                let frontmatter: Frontmatter = toml::from_str(parts[1]).unwrap();
                let markdown_content = parts[2];

                Self {
                    id: id as i32,
                    title: frontmatter.title,
                    description: frontmatter.description,
                    date: frontmatter.date,
                    image: frontmatter.image,
                    content: markdown_content.to_string(),
                    tags: frontmatter.tags,
                }
            })
            .collect::<Vec<_>>()
    }

    fn new(id: i32) -> Self {
        Self::all()
            .into_iter()
            .find(|post| post.id == id)
            .unwrap_or_else(|| panic!("Blog post with id {} not found", id))
    }
}
#[component]
pub fn BlogPreview() -> Element {
    let posts = BlogPost::all();

    println!("{:?}", posts);

    rsx! {
        div { id: "blogs", class: "container mx-auto px-4 py-12",
            h2 { class: "text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-purple-500 mb-2",
                "Latest Blog Posts"
            }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6 mt-8",
                for post in posts {
                    Link { to: Route::Blog { id: post.id },
                        div { class: "group relative overflow-hidden rounded-xl bg-gray-900/50 border border-gray-800 p-6 transition-all duration-300 hover:bg-gray-900/70",
                            span { class: "text-sm text-gray-400", "{post.date}" }
                            h3 { class: "text-xl font-semibold text-white mt-2 group-hover:text-blue-400 transition-colors",
                                "{post.title}"
                            }
                            p { class: "text-gray-400 mt-2", "{post.description}" }
                            div { class: "flex flex-wrap gap-2 mt-4",
                                for tag in post.tags {
                                    span { class: "px-2 py-1 text-xs rounded-full bg-gray-800 text-gray-300",
                                        "{tag}"
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
pub fn Blog(id: i32) -> Element {
    let all_posts = BlogPost::all();
    let post = BlogPost::new(id);
    let current_index = all_posts.iter().position(|p| p.id == id).unwrap();
    let has_previous = current_index > 0;
    let has_next = current_index < all_posts.len() - 1;

    rsx! {
        div { class: "container mx-auto px-4 py-12 max-w-4xl",
            // Header
            div { class: "mb-8 mt-8",
                span { class: "text-sm text-gray-400", "{post.date}" }
                h1 { class: "text-4xl font-bold text-white mt-2", "{post.title}" }
                div { class: "flex flex-wrap gap-2 mt-4",
                    for tag in post.tags {
                        span { class: "px-2 py-1 text-xs rounded-full bg-gray-800 text-gray-300",
                            "{tag}"
                        }
                    }
                }
            }
            // Content
            div { class: "prose prose-invert max-w-none container is-fluid",
                Markdown { content: post.content }
            }
            // Navigation
            div { class: "flex items-center justify-between mt-12 pt-8 border-t border-gray-800",
                {
                    if has_previous {
                        rsx! {
                            Link {
                                to: Route::Blog { id: all_posts[current_index - 1].id },
                                class: "flex items-center text-gray-400 hover:text-white transition-colors",
                                i { class: "fas fa-arrow-left mr-2" }
                                "Previous Post"
                            }
                        }
                    } else {
                        rsx! { div {} }
                    }
                },
                {
                    if has_next {
                        rsx! {
                            Link {
                                to: Route::Blog { id: all_posts[current_index + 1].id },
                                class: "flex items-center text-gray-400 hover:text-white transition-colors",
                                "Next Post"
                                i { class: "fas fa-arrow-right ml-2" }
                            }
                        }
                    } else {
                        rsx! { div {} }
                    }
                }
            }
        }
    }
}
