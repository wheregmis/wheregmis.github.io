use dioxus::prelude::*;
#[derive(Clone, Eq, PartialEq)]
pub enum MainAxisAlignment {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}
#[derive(Clone, Eq, PartialEq)]
pub enum CrossAxisAlignment {
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

#[component]
pub fn Row(
    main_axis_alignment: Option<MainAxisAlignment>,
    cross_axis_alignment: Option<CrossAxisAlignment>,
    class: Option<String>,
    children: Element,
) -> Element {
    let main_axis_alignment =
        main_axis_alignment.map_or("justify-center", |alignment| match alignment {
            MainAxisAlignment::Start => "justify-start",
            MainAxisAlignment::End => "justify-end",
            MainAxisAlignment::Center => "justify-center",
            MainAxisAlignment::SpaceBetween => "justify-between",
            MainAxisAlignment::SpaceAround => "justify-around",
            MainAxisAlignment::SpaceEvenly => "justify-evenly",
        });

    let cross_axis_alignment =
        cross_axis_alignment.map_or("items-center", |alignment| match alignment {
            CrossAxisAlignment::Start => "items-start",
            CrossAxisAlignment::End => "items-end",
            CrossAxisAlignment::Center => "items-center",
            CrossAxisAlignment::Stretch => "items-stretch",
            CrossAxisAlignment::Baseline => "items-baseline",
        });

    let class = class.unwrap_or("".to_string());

    rsx! {
        div { class: format!("flex flex-row {} {} {}", main_axis_alignment, cross_axis_alignment, class), {children} }
    }
}

#[component]
pub fn Column(
    main_axis_alignment: Option<MainAxisAlignment>,
    cross_axis_alignment: Option<CrossAxisAlignment>,
    class: Option<String>,
    children: Element,
) -> Element {
    let main_axis_alignment =
        main_axis_alignment.map_or("justify-center", |alignment| match alignment {
            MainAxisAlignment::Start => "justify-start",
            MainAxisAlignment::End => "justify-end",
            MainAxisAlignment::Center => "justify-center",
            MainAxisAlignment::SpaceBetween => "justify-between",
            MainAxisAlignment::SpaceAround => "justify-around",
            MainAxisAlignment::SpaceEvenly => "justify-evenly",
        });

    let cross_axis_alignment =
        cross_axis_alignment.map_or("items-center", |alignment| match alignment {
            CrossAxisAlignment::Start => "items-start",
            CrossAxisAlignment::End => "items-end",
            CrossAxisAlignment::Center => "items-center",
            CrossAxisAlignment::Stretch => "items-stretch",
            CrossAxisAlignment::Baseline => "items-baseline",
        });

    let class = class.unwrap_or("".to_string());

    rsx! {
        div { class: format!("flex flex-col {} {} {}", main_axis_alignment, cross_axis_alignment, class), {children} }
    }
}

#[component]
pub fn Expanded(class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        div { class: format!("flex-grow {}", class), {children} }
    }
}
