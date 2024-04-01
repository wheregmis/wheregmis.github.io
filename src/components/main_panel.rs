use crate::BlogList;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::HiCode;
use dioxus_free_icons::Icon;
use wheregmis::ui_core::prelude::*;

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
        div { class: "w-full h-full flex flex-row items-center justify-start m-5",
            Icon { class: "w-20 h-20 fill-gray-200", icon: HiCode }

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
