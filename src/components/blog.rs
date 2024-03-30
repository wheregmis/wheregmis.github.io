use crate::blog_content::BLOG_POSTS;
use crate::Markdown;
use dioxus::prelude::*;

#[component]
pub fn BlogPost(id: String) -> Element {
    let post = BLOG_POSTS.iter().find(|p| p.slug == id).unwrap();

    rsx! {
        div { class: "flex flex-col items-center justify-center min-w-full mt-5 bg-gray-900",
            Link { to: "/", h2 { class: "text-gray-500", "Go Back" } }
            Markdown {
                class: "content text-gray-500 m-5 items-center justify-center bg-gray-950",
                content: post.content
            }
        }
    }
}

#[component]
pub fn BlogList() -> Element {
    let posts = BLOG_POSTS.iter().map(|post| {
        rsx! {
            div { class: "w-full h-full flex flex-row items-center mt-5 justify-evenly bg-gray-900",
                img { src: post.image, alt: post.title, class: "w-20 h-20 object-cover" }
                h2 { class: "text-gray-500", "{post.title}" }
                Link { to: format!("/blog/{}", post.slug), h2 { class: "text-gray-500", "Read More" } }
            }
        }
    });

    if BLOG_POSTS.is_empty() {
        rsx!( h1 { class: "text-2xl font-bold text-white mt-5", "No posts found" } )
    } else {
        rsx!({ posts })
    }
}
