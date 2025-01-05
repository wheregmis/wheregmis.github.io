use dioxus::prelude::*;
use dioxus_elements::div;
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

const MARKDOWN_CSS: Asset = asset!("/assets/markdown.css");

#[component]
pub fn Markdown(props: MarkdownProps) -> Element {
    let parser = Parser::new(&props.content);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    rsx! {
        style { {r#"
            /* Base styles */
            .markdown-content {
                font-size: 1rem;
                color: rgba(255, 255, 255, 0.85);
                line-height: 1.625;
            }

            /* Headings */
            .markdown-content h1, .markdown-content h2, .markdown-content h3,
            .markdown-content h4, .markdown-content h5, .markdown-content h6 {
                color: rgba(255, 255, 255, 0.95);
                font-weight: 600;
                margin-top: 1.5em;
                margin-bottom: 0.5em;
            }

            .markdown-content h1 { font-size: 2.25rem; }
            .markdown-content h2 { font-size: 1.875rem; }
            .markdown-content h3 { font-size: 1.5rem; }
            .markdown-content h4 { font-size: 1.25rem; }
            .markdown-content h5 { font-size: 1.125rem; }
            .markdown-content h6 { font-size: 1rem; }

            /* Paragraphs and spacing */
            .markdown-content p {
                margin-bottom: 1rem;
            }

            /* Links */
            .markdown-content a {
                color: #61afef;
                text-decoration: none;
                border-bottom: 1px solid transparent;
                transition: border-color 0.2s, color 0.2s;
            }

            .markdown-content a:hover {
                color: #8cc1f5;
                border-bottom-color: #8cc1f5;
            }

            /* Code blocks */
            .markdown-content pre {
                background-color: #1e2127;
                padding: 1rem;
                border-radius: 0.75rem;
                overflow: visible; /* Changed from hidden to visible */
                margin: 1rem 0;
                position: relative;
                border: 1px solid rgba(255, 255, 255, 0.1);
            }

            .code-block {
                position: relative;
                z-index: 1;
            }

            .copy-button {
                position: absolute;
                top: 0.5rem;
                right: 0.5rem;
                padding: 0.25rem 0.5rem;
                background-color: rgba(255, 255, 255, 0.1);
                border: none;
                border-radius: 0.375rem;
                color: var(--text-secondary);
                cursor: pointer;
                font-size: 0.75rem;
                transition: all 0.2s;
                z-index: 20;
            }

            .copy-button:hover {
                background-color: rgba(255, 255, 255, 0.2);
            }

            /* Inline code */
            .markdown-content code {
                padding: 0.125rem 0.375rem;
                border-radius: 0.25rem;
                font-size: 0.875rem;
                font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace;
            }

            /* Lists */
            .markdown-content ul,
            .markdown-content ol {
                padding-left: 1.5rem;
                margin: 1rem 0;
            }

            .markdown-content li {
                margin-bottom: 0.5rem;
            }

            .markdown-content ul { list-style-type: disc; }
            .markdown-content ul ul { list-style-type: circle; }
            .markdown-content ul ul ul { list-style-type: square; }
            .markdown-content ol { list-style-type: decimal; }
            .markdown-content ol ol { list-style-type: lower-alpha; }
            .markdown-content ol ol ol { list-style-type: lower-roman; }

            /* Task Lists */
            .markdown-content input[type="checkbox"] {
                margin-right: 0.5rem;
                position: relative;
                top: 1px;
            }

            .markdown-content .task-list-item {
                list-style-type: none;
                padding-left: 0;
                margin-left: -1.5rem;
            }

            /* Blockquotes */
            .markdown-content blockquote {
                border-left: 4px solid #61afef;
                padding: 0.5rem 0 0.5rem 1rem;
                margin: 1rem 0;
                background-color: rgba(97, 175, 239, 0.1);
                border-radius: 0 0.375rem 0.375rem 0;
            }

            .markdown-content blockquote p {
                margin: 0;
            }

            /* Tables */
            .markdown-content table {
                width: 100%;
                border-collapse: collapse;
                margin: 1rem 0;
                overflow-x: auto;
                display: block;
            }

            .markdown-content table th,
            .markdown-content table td {
                padding: 0.5rem 1rem;
                border: 1px solid rgba(255, 255, 255, 0.2);
                text-align: left;
            }

            .markdown-content table th {
                background-color: rgba(255, 255, 255, 0.1);
                font-weight: 600;
            }

            .markdown-content table tr:nth-child(even) {
                background-color: rgba(255, 255, 255, 0.025);
            }

            /* Horizontal Rule */
            .markdown-content hr {
                border: none;
                border-top: 1px solid rgba(255, 255, 255, 0.1);
                margin: 2rem 0;
            }

            /* Text Formatting */
            .markdown-content strong {
                color: rgba(255, 255, 255, 0.95);
                font-weight: 600;
            }

            .markdown-content em {
                font-style: italic;
            }

            .markdown-content del {
                text-decoration: line-through;
                color: rgba(255, 255, 255, 0.5);
            }

            .markdown-content mark {
                background-color: rgba(97, 175, 239, 0.2);
                padding: 0.125rem 0.25rem;
                border-radius: 0.25rem;
                color: rgba(255, 255, 255, 0.95);
            }

            /* Definition Lists */
            .markdown-content dl {
                margin: 1rem 0;
            }

            .markdown-content dt {
                font-weight: 600;
                color: rgba(255, 255, 255, 0.95);
                margin-top: 1rem;
            }

            .markdown-content dd {
                margin-left: 1rem;
                margin-top: 0.25rem;
            }

            /* Abbreviations */
            .markdown-content abbr {
                border-bottom: 1px dotted var(--text-muted);
                cursor: help;
            }

            /* Keyboard */
            .markdown-content kbd {
                background-color: var(--surface);
                border: 1px solid rgba(255, 255, 255, 0.1);
                border-radius: 0.25rem;
                padding: 0.125rem 0.375rem;
                font-size: 0.875rem;
                box-shadow: 0 1px 0 rgba(255, 255, 255, 0.1);
            }

            /* Subscript and Superscript */
            .markdown-content sub,
            .markdown-content sup {
                font-size: 0.75em;
                line-height: 0;
                position: relative;
                vertical-align: baseline;
            }

            .markdown-content sup { top: -0.5em; }
            .markdown-content sub { bottom: -0.25em; }

            /* Details and Summary */
            .markdown-content details {
                margin: 1rem 0;
                padding: 0.5rem 1rem;
                background-color: rgba(255, 255, 255, 0.05);
                border-radius: 0.375rem;
                border: 1px solid rgba(255, 255, 255, 0.1);
            }

            .markdown-content summary {
                cursor: pointer;
                font-weight: 600;
                color: var(--text-primary);
            }

            .markdown-content details[open] summary {
                margin-bottom: 0.5rem;
            }

            /* Footnotes */
            .markdown-content .footnotes {
                margin-top: 2rem;
                padding-top: 1rem;
                border-top: 1px solid rgba(255, 255, 255, 0.1);
            }

            .markdown-content .footnotes ol {
                font-size: 0.875rem;
            }

            .markdown-content .footnote-ref {
                font-size: 0.75em;
                vertical-align: super;
                line-height: 0;
            }
            "#.to_string()} }
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.8.0/styles/github-dark.min.css",
        }

        // document::Link { rel: "stylesheet", href: MARKDOWN_CSS }
        div {
            id: "{props.id}",
            class: "markdown-content prose prose-invert max-w-none {props.class}",
            onmounted: move |_| {
                eval(r#"
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
                "#);
            },
            div {
                class: "prose-content",
                dangerous_inner_html: html_buf,
            }
        }
    }
}
