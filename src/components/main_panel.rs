use dioxus::prelude::*;
use wheregmis::ui_core::prelude::*;

use crate::BlogList;

#[component]
pub fn MainPanelClone() -> Element {
    rsx! {
        Expanded { class: "bg-gray-900 w-2/4 h-full", BlogList {} }
    }
}

#[component]
pub fn MainPanel() -> Element {
    rsx! { BlogList {} }
}

#[component]
pub fn Header() -> Element {
    rsx! {
        div { class: "w-full h-full flex flex-row items-start justify-start m-5",
            img {
                class: "w-16 h-16 rounded-full",
                src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                alt: "Sabin Regmi"
            }

            div { class: "w-full h-full flex-grow ml-10",
                div { class: "w-full flex flex-row items-center justify-start",
                    h1 { class: "text-3xl font-bold text-white", "Sabin Regmi," }
                    p { class: "text-gray-500 text-xl ml-2", "Rust Enthusiast" }
                }
                div {
                    p { class: "text-gray-500 text-sm ",
                        "I design and code beautifully simple things, and
            occasionally i write about them"
                    }
                }
            }
        }
        // create a divider
        div { class: "border border-gray-700 mt-5 w-full" }
    }
}
