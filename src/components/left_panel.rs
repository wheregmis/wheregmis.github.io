use dioxus::prelude::*;
use wheregmis::ui_core::prelude::*;
#[component]
pub fn LeftPanel() -> Element {
    rsx! {
        Expanded { class: "bg-gray-900 w-1/4 h-full md:block hidden m-5 shadow-neumorphic text-gray-300",
            Column { class: "w-full h-full items-start justify-start ", MyIntro {} }
        }
    }
}
pub fn MyIntro() -> Element {
    rsx! {
        Column { class: "w-full items-start justify-start",
            Row { class: "w-full items-start justify-between",
                Column { class: "items-end justify-start",
                    img {
                        class: "w-16 h-16 rounded-full",
                        src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                        alt: "Sabin Regmi"
                    }
                }
                Column {
                    Row { class: "items-end justify-start -mt-4",
                        img {
                            class: "w-4 h-4 rounded-full mr-4",
                            src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                            alt: "Sabin Regmi"
                        }
                        img {
                            class: "w-4 h-4 rounded-full mr-4",
                            src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                            alt: "Sabin Regmi"
                        }
                    }
                }
            }
            Row {
                Column { class: "items-start justify-start",
                    h1 { class: "text-xl font-bold", "Sabin Regmi" }
                    h1 { class: "text-md", "get2sabin@gmail.com" }
                    h1 { class: "text-md", "wheregmis@github.io" }
                    Row { class: "w-full h-full items-center justify-items-start",
                        div { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-500 text-white",
                            "Rust"
                        }
                        div { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-500 text-white",
                            "Python"
                        }
                        div { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-500 text-white",
                            "Javascript"
                        }
                        div { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-500 text-white",
                            "Dart"
                        }
                    }
                }
            }
        }
    }
}

pub fn MyBio() -> Element {
    rsx! {
        div {
            h1 { "My Biography" }
            p { "This is where you can write your biography." }
        }
    }
}

pub fn MyWorkHistory() -> Element {
    rsx! {
        div {
            h1 { "My Work History" }
            p { "This is where you can list your previous work experiences." }
        }
    }
}
