use crate::{components::Markdown, Route};
use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use document::eval;
// Add this import
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

    rsx! {
        div { id: "blogs", class: "container mx-auto px-4 py-12",
            h2 { class: "text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-purple-500 mb-2",
                "Latest Blog Posts"
            }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6 mt-8",
                for (index , post) in posts.into_iter().enumerate() {
                    {
                        let mut card_transform = use_motion(Transform::new(0.0, 20.0, 0.8, 0.0));
                        let mut card_opacity = use_motion(0.0f32);
                        let on_card_visible = move |_| {
                            let card_sequence = AnimationSequence::new()
                                .then(
                                    Transform::identity(),
                                    AnimationConfig::new(
                                            AnimationMode::Spring(Spring {
                                                stiffness: 100.0,
                                                damping: 15.0,
                                                mass: 1.0,
                                                velocity: 0.0,
                                            }),
                                        )
                                        .with_delay(Duration::from_millis(1300)),
                                );
                            card_transform.animate_sequence(card_sequence);
                        };
                        rsx! {
                            Link { to: Route::Blog { id: post.id },
                                div {
                                    // Added fixed height and hover glow effect
                                    onvisible: on_card_visible,
                                    class: "group relative h-[250px] overflow-hidden rounded-xl bg-gray-900/50 border border-gray-800 p-6 transition-all duration-300 hover:bg-gray-900/70 hover:border-gray-700 hover:shadow-xl hover:shadow-primary/20",
                                    style: "transform: translateY({card_transform.get_value().y}px) scale({card_transform.get_value().scale}); opacity: {card_transform.get_value().scale};",
                                    // Glow effect
                                    div { class: "absolute inset-0 bg-gradient-to-r from-blue-500/0 via-blue-500/10 to-purple-500/0 opacity-0 group-hover:opacity-100 transition-all duration-500 blur-xl" }
                                    // Content wrapper with relative positioning
                                    div { class: "relative h-full flex flex-col",
                                        // Date
                                        span { class: "text-sm text-gray-400", "{post.date}" }
                                        // Title with line clamp
                                        h3 { class: "text-xl font-semibold text-white mt-2 group-hover:text-blue-400 transition-colors line-clamp-2",
                                            "{post.title}"
                                        }
                                        // Description with line clamp
                                        p { class: "text-gray-400 mt-2 line-clamp-3", "{post.description}" }
                                        // Tags at the bottom
                                        div { class: "flex flex-wrap gap-2 mt-auto pt-4",
                                            for (tag_index , tag) in post.tags.iter().enumerate() {
                                                {
                                                    let mut tag_transform = use_motion(Transform::new(0.0, 10.0, 0.0, 0.0));
                                                    use_effect(move || {
                                                        let tag_delay = Duration::from_millis(
                                                            (100 * index as u64) + (50 * tag_index as u64),
                                                        );
                                                        tag_transform
                                                            .animate_to(
                                                                Transform::identity(),
                                                                AnimationConfig::new(
                                                                        AnimationMode::Spring(Spring {
                                                                            stiffness: 100.0,
                                                                            damping: 15.0,
                                                                            mass: 1.0,
                                                                            ..Default::default()
                                                                        }),
                                                                    )
                                                                    .with_delay(tag_delay),
                                                            );
                                                    });
                                                    rsx! {
                                                        span {
                                                            class: "px-2 py-1 text-xs rounded-full bg-gray-800 text-gray-300 group-hover:bg-gray-700 transition-colors",
                                                            style: "transform: translateY({tag_transform.get_value().y}px) scale({tag_transform.get_value().scale}); opacity: {tag_transform.get_value().scale};",
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

    let preserve_style_and_copy = move |_| {
        eval(
            r#"
        hljs.highlightAll();
        
        // Add copy function
        window.copyCode = function(button) {
            const pre = button.parentElement.querySelector('pre');
            const code = pre.textContent;
            
            navigator.clipboard.writeText(code).then(() => {
                const originalText = button.textContent;
                button.textContent = 'Copied!';
                button.style.backgroundColor = 'rgba(46, 160, 67, 0.4)';
                
                setTimeout(() => {
                    button.textContent = originalText;
                    button.style.backgroundColor = 'rgba(255, 255, 255, 0.1)';
                }, 2000);
            }).catch(err => {
                console.error('Failed to copy:', err);
                button.textContent = 'Error!';
                button.style.backgroundColor = 'rgba(248, 81, 73, 0.4)';
                
                setTimeout(() => {
                    button.textContent = 'Copy';
                    button.style.backgroundColor = 'rgba(255, 255, 255, 0.1)';
                }, 2000);
            });
        };

        // Initialize copy buttons after syntax highlighting
        const preElements = document.querySelectorAll('.markdown-content pre');
        preElements.forEach(pre => {
            const wrapper = document.createElement('div');
            wrapper.className = 'code-block';
            const copyButton = document.createElement('button');
            copyButton.className = 'copy-button';
            copyButton.textContent = 'Copy';
            copyButton.onclick = function() { copyCode(this); };
            
            pre.parentNode.insertBefore(wrapper, pre);
            wrapper.appendChild(pre);
            wrapper.appendChild(copyButton);
        });
    "#,
        );
    };

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
            div { class: "prose prose-invert w-full is-fluid",
                Markdown { content: post.content }
            }
            // Navigation
            div { class: "flex items-center justify-between mt-12 pt-8 border-t border-gray-800",
                {
                    if has_previous {
                        let prev_post = &all_posts[current_index - 1];
                        rsx! {
                            Link {
                                to: Route::Blog { id: prev_post.id },
                                onmounted: preserve_style_and_copy,
                                class: "flex flex-col items-start text-gray-400 hover:text-white transition-colors group",
                                span { class: "flex items-center text-sm mb-1",
                                    i { class: "fas fa-arrow-left mr-2 group-hover:-translate-x-1 transition-transform" }
                                    "Previous"
                                }
                                span { class: "text-base font-medium", "{prev_post.title}" }
                            }
                        }
                    } else {
                        rsx! {
                            div {}
                        }
                    }
                }
                {
                    if has_next {
                        let next_post = &all_posts[current_index + 1];
                        rsx! {
                            Link {
                                to: Route::Blog { id: next_post.id },
                                onmounted: preserve_style_and_copy,
                                class: "flex flex-col items-end text-gray-400 hover:text-white transition-colors group",
                                span { class: "flex items-center text-sm mb-1",
                                    "Next"
                                    i { class: "fas fa-arrow-right ml-2 group-hover:translate-x-1 transition-transform" }
                                }
                                span { class: "text-base font-medium", "{next_post.title}" }
                            }
                        }
                    } else {
                        rsx! {
                            div {}
                        }
                    }
                }
            }
        }
    }
}
