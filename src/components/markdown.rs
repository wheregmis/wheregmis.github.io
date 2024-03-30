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
    let html = markdown::to_html(&props.content);

    rsx! {
        iframe {
            srcdoc: format!(
                r#"
                                                                                <!DOCTYPE html>
                                                                                <html class="dark">
                                                                                <head>
                                                                                    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@1.0.0/css/bulma.min.css">
                                                                                    
                                                                                </head>
                                                                                <body>
                                                                                    <div id="{}" class="{}" style="width: 100%;">
                                                                                        {}
                                                                                    </div>
                                                                                </body>
                                                                                </html>
                                                                                "#,
                props.id,
                props.class,
                html,
            ),
            style: "width: 100%; height: 80vh; border: none;",
            class: "w-full"
        }
    }
}

// rsx! {iframe { srcdoc: format!(
//     r#"
//                     <!DOCTYPE html>
//                     <html class="dark">
//                     <head>
//                         <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@1./css/bulma.min.css">

//                     </head>
//                     <body>
//                         <div id="{}" class="{}" style="width: 100%;">
//                             {}
//                         </div>
//                     </body>
//                     </html>
//                     "#,
//     props.id,
//     props.class,
//     html,
// ), class: "w-full h-full bg-gray-900" }
// }
