use crate::{model::Model, requests, route::Route, styles};
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use shared_models::BoardName;

#[component]
pub fn Users(cx: Scope, board_name: BoardName) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let nav = use_navigator(cx);
    if &model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    use_future(cx, (), |_| requests::board(model.clone()));
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
                onclick: |_| {
                    nav.push(Route::Board {
                        board_name: board_name.clone(),
                    });
                },
                "Back",
            }
        }
    })
}
