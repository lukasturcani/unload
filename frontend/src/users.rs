use crate::color_picker::{self, ColorPicker};
use crate::{model::Model, requests};
use dioxus::prelude::*;
use itertools::Itertools;
use shared_models::BoardName;
use shared_models::{Color, UserId};

enum Column {
    Color,
    Name,
}

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
                        for (row_index, (user_id, user)) in users.iter().sorted_by_key(|x| x.0).enumerate() {
                            tr {
                                class: if row_index == users.len() - 1 {
                                    "bg-gray-800 sm:hover:bg-gray-600 border-gray-700"
                                } else {
                                    "bg-gray-800 sm:hover:bg-gray-600 border-gray-700 border-b"
                                },
                                td {
                                    class: "px-6 py-4",
                                    onclick: {
                                        let user_id = user_id.clone();
                                        move |_| {
                                            color.set(model.read().users[&user_id].color);
                                            edit_field.set(Some((row_index, Column::Color)));
                                        }
                                    },
                                    match **edit_field {
                                        Some((edit_row, Column::Color)) if edit_row == row_index => {rsx!{
                                            ColorPicker {
                                                on_pick_color: {
                                                        let user_id = *user_id;
                                                        move |color| {
                                                        edit_field.set(None);
                                                        cx.spawn(set_user_color(model.clone(), user_id, color));
                                                    }
                                                },
                                            }
                                        }}
                                        _ => {rsx!{
                                            div {
                                                class: "flex flex-row gap-1",
                                                div {
                                                    class: "w-8 h-8 rounded cursor-pointer bg-blue-500 {color_picker::class(&user.color)}",
                                                },
                                                svg {
                                                    xmlns: "http://www.w3.org/2000/svg",
                                                    fill: "none",
                                                    "viewBox": "0 0 24 24",
                                                    "stroke-width": "1.5",
                                                    stroke: "currentColor",
                                                    class: "w-4 h-4",
                                                    path {
                                                        "stroke-linecap": "round",
                                                        "stroke-linejoin": "round",
                                                        d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                                                    }
                                                }
                                            }
                                        }}
                                    }
                                }
                                td {
                                    class: "px-6 py-4",
                                    onclick: {
                                            let user_id = user_id.clone();
                                            move |_| {
                                            name.set(model.read().users[&user_id].name.clone());
                                            edit_field.set(Some((row_index, Column::Name)));
                                        }
                                    },
                                    div {
                                        class: "flex flex-row gap-1",
                                        "{user.name}"
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "none",
                                            "viewBox": "0 0 24 24",
                                            "stroke-width": "1.5",
                                            stroke: "currentColor",
                                            class: "w-4 h-4",
                                            path {
                                                "stroke-linecap": "round",
                                                "stroke-linejoin": "round",
                                                d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
        }
        }
    })
}

async fn set_user_color(model: UseSharedState<Model>, user_id: UserId, color: Color) {
    if send_set_user_color_request(model.clone(), user_id, color)
        .await
        .is_ok()
    {
        requests::board(model).await;
    }
}

async fn send_set_user_color_request(
    model: UseSharedState<Model>,
    user_id: UserId,
    color: Color,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/users/{}/color",
            model.board_name, user_id
        ))?
    };
    Ok(reqwest::Client::new()
        .put(url)
        .json(&color)
        .send()
        .await?
        .json::<()>()
        .await?)
}
