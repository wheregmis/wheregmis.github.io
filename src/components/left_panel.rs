use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::icons::hi_solid_icons::HiHome;
use dioxus_free_icons::Icon;
use wheregmis::ui_core::prelude::*;
#[component]
pub fn LeftPanel() -> Element {
    rsx! {
        Expanded { class: "bg-gray-900 w-1/4 h-full md:block hidden m-5 text-gray-300",
            Column {
                class: "w-full h-full m-5",
                main_axis_alignment: MainAxisAlignment::Start,
                cross_axis_alignment: CrossAxisAlignment::Start,
                MyIntro {}
                MyBio {}
                // create a divider
                div { class: "border border-gray-700 mt-5 -ml-5 w-full" }
                MyWorkHistory {}
                div { class: "border border-gray-700 mt-5 -ml-5 w-full" }
                MyEducationHistory {}
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
                    div { class: "flex flex-row items-end justify-between",
                        Icon { class: "w-5 h-5 fill-white mr-4", icon: HiHome }
                        Icon { class: "w-5 h-5 fill-white", icon: FaGithub }
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
            h1 { "Work History" }
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

pub fn MyEducationHistory() -> Element {
    rsx! {
        div { class: "w-full mt-5 flex flex-col items-start justify-start",
            h1 { "Education" }
            div { class: "w-full mt-5 flex flex-row items-center justify-evenly -ml-5",
                img {
                    class: "w-2 h-2 rounded-full mr-2",
                    src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                    alt: "Sabin Regmi"
                }
                div { class: "flex flex-col items-start justify-start",
                    p { "Post Graduate" }
                    p { class: "text-xs text-gray-500", "Lambton College" }
                }
                p { class: "text-sm text-gray-500 ml-5", "Jan 2022 - 2024" }
            }
            div { class: "w-full mt-5 flex flex-row items-start justify-evenly -ml-5",
                img {
                    class: "w-2 h-2 rounded-full mr-2 -ml-2",
                    src: "http://rustacean.net/assets/rustacean-flat-happy.png",
                    alt: "Sabin Regmi"
                }
                div { class: "flex flex-col items-start justify-start",
                    p { "Bachelor's   " }
                    p { class: "text-xs text-gray-500", "Leeds Beckett" }
                }
                p { class: "text-sm text-gray-500 ml-5", "Nov 2016 - 2019" }
            }
        }
    }
}
