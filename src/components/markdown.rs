#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::{debug, info};

#[derive(Props, Clone, PartialEq)]
pub struct MarkdownProps {
    #[props(default)]
    id: String,
    #[props(default)]
    class: String,
    content: String,
}

/// Render some text as markdown.
pub fn Markdown(props: MarkdownProps) -> Element {
    let html_buf = String::from(props.content);
    let html = markdown::to_html(&html_buf);
    info!("Markdown: {}", html);

    rsx! {
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css"
        }
        div {
            id: "{props.id}",
            class: "{props.class}",
            dangerous_inner_html: "{html}"
        }
    }
}
