use dioxus::prelude::*;
use wheregmis::ui_core::prelude::*;
#[component]
pub fn LeftPanel() -> Element {
    rsx! {
        Expanded { class: "bg-gray-900 w-1/4 h-full md:block hidden m-5 shadow-neumorphic text-gray-300",
            Column {
                class: "w-full h-full m-5",
                main_axis_alignment: MainAxisAlignment::Start,
                cross_axis_alignment: CrossAxisAlignment::Start,
                MyIntro {}
                MyBio {}
                // create a divider
                div { class: "border border-gray-700 mt-5 -ml-5 w-full" }
                MyWorkHistory {}
            }
        }
    }
}
pub fn MyIntro() -> Element {
    rsx! {
        Column { class: "items-start justify-start",
            Row {
                class: "w-full",
                cross_axis_alignment: CrossAxisAlignment::Start,
                main_axis_alignment: MainAxisAlignment::SpaceBetween,
                Column { main_axis_alignment: MainAxisAlignment::Start, cross_axis_alignment: CrossAxisAlignment::Start,
                    img {
                        class: "w-16 h-16 rounded-full",
                        src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                        alt: "Sabin Regmi"
                    }
                }
                Column {
                    Row { main_axis_alignment: MainAxisAlignment::Start, cross_axis_alignment: CrossAxisAlignment::Start,
                        img {
                            class: "w-4 h-4 rounded-full mr-5",
                            src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                            alt: "Sabin Regmi"
                        }
                        img {
                            class: "w-4 h-4 rounded-full",
                            src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                            alt: "Sabin Regmi"
                        }
                    }
                }
            }
            Row {
                main_axis_alignment: MainAxisAlignment::Start,
                cross_axis_alignment: CrossAxisAlignment::Start,
                class: "w-full",
                Column { main_axis_alignment: MainAxisAlignment::Start, cross_axis_alignment: CrossAxisAlignment::Start,
                    h1 { class: "text-xl font-bold", "Sabin Regmi" }
                    h1 { class: "text-gray-500 text-sm ", "get2sabin@gmail.com" }
                    h1 { class: "text-gray-500 text-sm", "wheregmis@github.io" }
                    Row {
                        class: "w-full h-full mt-2",
                        main_axis_alignment: MainAxisAlignment::Start,
                        cross_axis_alignment: CrossAxisAlignment::Start,
                        div { class: "inline-flex items-center px-2 py-0.5 rounded-full text-xs font-thin bg-gray-500 text-white",
                            "Rust"
                        }
                        div { class: "inline-flex items-center px-2 py-0.5 rounded-full text-xs font-thin bg-gray-500 text-white ml-3",
                            "Python"
                        }
                        div { class: "inline-flex items-center px-2 py-0.5 rounded-full text-xs font-thin bg-gray-500 text-white ml-3",
                            "Javascript"
                        }
                        div { class: "inline-flex items-center px-2 py-0.5 rounded-full text-xs font-thin bg-gray-500 text-white ml-3",
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
        Row {
            class: "w-full mt-5",
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Start,
            Column {
                main_axis_alignment: MainAxisAlignment::Start,
                cross_axis_alignment: CrossAxisAlignment::Start,
                class: "w-full mr-7",
                h1 { "Bio" }
                p { class: "text-gray-500 text-sm",
                    "The world of digital design and
                development is constantly evolving and so
                has my role over the last 7 years."
                }
                Row {
                    class: "w-full mt-5 -ml-3",
                    main_axis_alignment: MainAxisAlignment::SpaceBetween,
                    cross_axis_alignment: CrossAxisAlignment::Start,
                    Column {
                        Row {
                            img {
                                class: "w-2 h-2 rounded-full mr-2",
                                src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                                alt: "Sabin Regmi"
                            }
                            h1 { class: "text-sm", "5 Years of Experience" }
                        }
                    }
                    Column {
                        Row {
                            img {
                                class: "w-2 h-3 rounded-full mr-2",
                                src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                                alt: "Sabin Regmi"
                            }
                            h1 { class: "text-sm", "10+ Projects" }
                        }
                    }
                }
            }
        }
    }
}

pub fn MyWorkHistory() -> Element {
    rsx! {
        div { class: "w-full mt-5 flex flex-col items-start justify-start",
            h1 { "My Work History" }
            div { class: "w-full mt-5 flex flex-row items-center justify-evenly -ml-5",
                img {
                    class: "w-2 h-2 rounded-full mr-2",
                    src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                    alt: "Sabin Regmi"
                }
                div { class: "flex flex-col items-start justify-start",
                    p { "Software Engineer" }
                    p { class: "text-xs text-gray-500", "TROES Corp." }
                }
                p { class: "text-sm text-gray-500 ml-5", "Aug 2022 - Current" }
            }
            div { class: "w-full mt-5 flex flex-row items-center justify-evenly -ml-5",
                img {
                    class: "w-2 h-2 rounded-full mr-2",
                    src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                    alt: "Sabin Regmi"
                }
                div { class: "flex flex-col items-start justify-start",
                    p { "Research Student" }
                    p { class: "text-xs text-gray-500", "Lambton College" }
                }
                p { class: "text-sm text-gray-500 ml-5", "Aug 2022 - Current" }
            }
        }
    }
}
