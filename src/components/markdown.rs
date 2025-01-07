use dioxus::prelude::*;

use document::eval;
use pulldown_cmark::{html, Parser};

use crate::MARKDOWN_CSS;

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
        document::Link { rel: "stylesheet", href: MARKDOWN_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.8.0/styles/github-dark.min.css",
        }
        // document::Link {
        //     rel: "stylesheet",
        //     href: "https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.8.1/github-markdown-dark.css",
        // }
        div {
            id: "{props.id}",
            class: "markdown-content prose prose-invert max-w-none {props.class}",
            onmounted: preserve_style_and_copy,
            div { class: "prose-content", dangerous_inner_html: html_buf }
        }
    }
}
