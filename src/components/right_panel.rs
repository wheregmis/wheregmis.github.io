use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{FaGithub, FaLinkedin, FaReddit, FaTwitter};
use dioxus_free_icons::icons::hi_solid_icons::HiCode;
use dioxus_free_icons::Icon;
use wheregmis::ui_core::prelude::Expanded;

#[component]
pub fn RightPanel() -> Element {
    rsx! {
        Expanded { class: " bg-gray-900 w-1/4 lg:block hidden items-start justify-start m-5",
            h1 { class: "title text-gray-500",
                div { class: "w-full h-full flex flex-col items-start justify-start",
                    h1 { class: "text-2xl font-bold text-white m-5", "Resume" }
                    p { class: "text-gray-500 text-sm mx-5",
                        "I am a software engineer with a passion for building web applications. I have experience in building web applications using Rust, JavaScript, and TypeScript. I am also a Rust enthusiast and have experience in building web applications using Rust and WebAssembly."
                    }

                    // create a button to download resume
                    div { class: "w-full h-full flex flex-row items-start justify-start m-5",
                        a {
                            href: "https://drive.google.com/file/d/1Q6Z",
                            class: "flex items-center text-sm text-indigo-500 hover:text-indigo-700",
                            span { "Download Resume" }
                        }
                    }

                    // create a divider
                    div { class: "border border-gray-700 w-full" }

                    // Create a section for opensource contributions
                    h1 { class: "text-2xl font-bold text-white m-5", "Contributions" }
                    div { class: "w-full h-full flex flex-row items-center justify-evenly",
                        Icon { class: "w-5 h-5 fill-white", icon: HiCode }
                        p { class: "text-gray-500 text-sm", "RINF (Rust in Flutter)" }
                        a {
                            href: "/",
                            class: "flex items-center text-sm text-indigo-500 hover:text-indigo-700",
                            span { "View" }
                        }
                    }

                    // create a divider
                    div { class: "border border-gray-700 mt-5 w-full" }

                    div { class: "w-full h-full flex flex-row items-center justify-evenly my-5",
                        Icon { class: "w-5 h-5 fill-white", icon: FaGithub }
                        Icon { class: "w-5 h-5 fill-white", icon: FaLinkedin }
                        Icon { class: "w-5 h-5 fill-white", icon: FaReddit }
                        Icon { class: "w-5 h-5 fill-white", icon: FaTwitter }
                    }
                }
            }
        }
    }
}
