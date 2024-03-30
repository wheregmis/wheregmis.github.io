use dioxus::prelude::*;
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
                        img {
                            class: "w-4 h-4 rounded-full mr-5",
                            src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                            alt: "Sabin Regmi"
                        }
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
                        img {
                            class: "w-4 h-4 rounded-full mr-5",
                            src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                            alt: "Sabin Regmi"
                        }
                        img {
                            class: "w-4 h-4 rounded-full mr-5",
                            src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                            alt: "Sabin Regmi"
                        }
                        img {
                            class: "w-4 h-4 rounded-full mr-5",
                            src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                            alt: "Sabin Regmi"
                        }
                        img {
                            class: "w-4 h-4 rounded-full mr-5",
                            src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                            alt: "Sabin Regmi"
                        }
                    }
                }
            }
        }
    }
}
