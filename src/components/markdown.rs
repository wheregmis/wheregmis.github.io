use dioxus::prelude::*;
use document::eval;
use pulldown_cmark::{html, Parser};

#[derive(Props, PartialEq, Clone)]
pub struct MarkdownProps {
    #[props(default)]
    id: String,
    #[props(default)]
    class: String,
    content: String,
}

#[component]
pub fn Markdown(props: MarkdownProps) -> Element {
    let parser = Parser::new(&props.content);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.8.0/styles/github-dark.min.css",
        }

        div {
            id: "{props.id}",
            class: "prose prose-invert max-w-none {props.class}",
            dangerous_inner_html: "{html_buf}",
            onmounted: move |_| {
                eval(
                    r#"
                                                                                                                            // Initialize highlight.js
                                                                                                                            hljs.highlightAll();
                                                                                                                        "#,
                );
            },
        }
    }
}
