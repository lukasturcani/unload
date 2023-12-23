use crate::{model::Model, styles};
use dioxus::prelude::*;

#[component]
fn Users(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    cx.render(rsx! {
        div {
            class: "bg-gray-900 h-screen w-screen",
            for (id, user) in model.read().users.iter() {
                div {
                    key: "{id}",
                    class: "bg-gray-700 rounded-lg p-2.5 m-2",
                    "{user.name}"
                }
            }
            button {
                class: styles::BUTTON,
                onclick: |_| {},
                "Back",
            }
        }
    })
}
