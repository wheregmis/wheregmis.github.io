use dioxus::prelude::*;

#[component]
pub fn PrimaryButton(
    on_click: EventHandler<MouseEvent>,
    text: String,
    class: Option<String>,
) -> Element {
    rsx!(
        button {
            class: "transition ease-in-out delay-150 bg-blue-500 hover:-translate-y-1 hover:scale-110 hover:bg-indigo-500 duration-300",
            class: class.unwrap_or("".to_string()),
            onclick: move |evt| on_click.call(evt),
            "{text}"
        }
    )
}

#[component]
pub fn SecondaryButton(
    on_click: EventHandler<MouseEvent>,
    text: String,
    class: Option<String>,
) -> Element {
    rsx!(
        button {
            class: "bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded-md",
            class: class.unwrap_or("".to_string()),
            onclick: move |evt| on_click.call(evt),
            "{text}"
        }
    )
}

#[component]
pub fn ErrorButton(
    on_click: EventHandler<MouseEvent>,
    text: String,
    class: Option<String>,
) -> Element {
    rsx!(
        button {
            class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded-md",
            class: class.unwrap_or("".to_string()),
            onclick: move |evt| on_click.call(evt),
            "{text}"
        }
    )
}

#[component]
pub fn OutlineButton(
    on_click: EventHandler<MouseEvent>,
    text: String,
    class: Option<String>,
) -> Element {
    rsx!(
        button {
            class: "bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded-md",
            class: class.unwrap_or("".to_string()),
            onclick: move |evt| on_click.call(evt),
            "{text}"
        }
    )
}

#[component]
pub fn Row(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-row w-full h-full items-center", {children} }
    }
}

#[component]
pub fn Column(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col items-center", {children} }
    }
}
