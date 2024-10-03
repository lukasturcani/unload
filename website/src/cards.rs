use dioxus::prelude::*;

#[component]
pub fn Card(children: Element) -> Element {
    rsx! {
        div {
            class: "max-w-5xl p-6 border rounded-lg shadow bg-background-card border-gray-700",
            {children}
        }
    }
}
