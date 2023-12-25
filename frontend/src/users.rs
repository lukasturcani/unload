use crate::{model::Model, requests};
use dioxus::prelude::*;
use shared_models::BoardName;

#[component]
pub fn Users(cx: Scope, board_name: BoardName) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    if &model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    use_future(cx, (), |_| requests::board(model.clone()));
    cx.render(rsx! {
        div {
            class: "bg-gray-900 h-screen w-screen grid justify-items-center",
            ul {
                class: "max-w-md divide-y divide-gray-200 dark:divide-gray-700",
                for (id, user) in model.read().users.iter() {
                    li {
                        key: "{id}",
                        class: "pb-3 sm:pb-4",
                        div {
                            class: "flex-1 min-w-0",
                            p {
                                class: "text-sm font-medium text-gray-900 truncate dark:text-white",
                                "{user.name}"
                            }
                        }
                    }
                }
            }
        }
    })
}
