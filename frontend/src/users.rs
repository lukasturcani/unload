use crate::{model::Model, requests};
use dioxus::prelude::*;
use shared_models::BoardName;
use crate::color_picker;
use shared_models::Color;

enum Column { Color, Name }

#[component]
pub fn Users(cx: Scope, board_name: BoardName) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    if &model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    use_future(cx, (), |_| requests::board(model.clone()));
    let users = &model.read().users;
    let edit_field = use_state(cx, || None::<(usize, Column)>);
    let color = use_state(cx, || Color::Black);
    let name = use_state(cx, String::new);
    cx.render(rsx! {
        div {
            class: "w-full p-2",
            div {
                class: "overflow-hidden border border-gray-900 w-full rounded-lg",
                table {
                    class: "w-full text-sm text-left text-gray-400",
                    thead {
                        class: "text-xs uppercase bg-gray-700 text-gray-400",
                        tr {
                            th {
                                scope: "col",
                                class: "px-6 py-3",
                                "Color"
                            }
                            th {
                                scope: "col",
                                class: "px-6 py-3",
                                "User"
                            }
                        }
                    }
                    tbody {
                        for (row_index, (user_id, user)) in users.iter().enumerate() {
                            tr {
                                class: if row_index == users.len() - 1 {
                                    "bg-gray-800 sm:hover:bg-gray-600 border-gray-700"
                                } else {
                                    "bg-gray-800 sm:hover:bg-gray-600 border-gray-700 border-b"
                                },
                                td {
                                    class: "px-6 py-4",
                                    onclick: {
                                        let model = model.clone();
                                        let user_id = user_id.clone();
                                        move |_| {
                                            color.set(model.read().users[&user_id].color);
                                            edit_field.set(Some((row_index, Column::Color)));
                                        }
                                    },
                                    div {
                                        class: "w-8 h-8 rounded cursor-pointer bg-blue-500 {color_picker::class(&user.color)}",
                                    },
                                }
                                td {
                                    class: "px-6 py-4",
                                    onclick: {
                                            let model = model.clone();
                                            let user_id = user_id.clone();
                                            move |_| {
                                            name.set(model.read().users[&user_id].name.clone());
                                            edit_field.set(Some((row_index, Column::Name)));
                                        }
                                    },
                                    "{user.name}"
                                }
                            }
                        }
                    }
                }
        }
        }
    })
}
