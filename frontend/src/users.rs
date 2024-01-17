use crate::color_picker::{self, ColorPicker};
use crate::route::Route;
use crate::styles;
use crate::{model::Model, requests};
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use itertools::Itertools;
use shared_models::BoardName;
use shared_models::{Color, UserId};

enum Column {
    Color,
    Name,
}

#[component]
pub fn Users(cx: Scope, board_name: BoardName) -> Element {
    let nav = use_navigator(cx);
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
            class: "
                w-screen h-screen
                bg-gray-900
                flex flex-col
            ",
            onclick: |_|  edit_field.set(None),
            div {
                class: "grow w-full p-4 overflow-auto",
                div {
                    class: "overflow-scroll border border-gray-900 w-full rounded-lg",
                    table {
                        class: "w-full text-sm text-left text-gray-400",
                        thead {
                            class: "text-xs uppercase bg-gray-700 text-gray-400",
                            tr {
                                th {
                                    scope: "col",
                                    class: "p-3",
                                    "Color"
                                }
                                th {
                                    scope: "col",
                                    class: "p-3",
                                    "User"
                                }
                                th {
                                    scope: "col",
                                    class: "p-3",
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
                                        class: "p-3",
                                        match **edit_field {
                                            Some((edit_row, Column::Color)) if edit_row == row_index => rsx!{
                                                ColorPicker {
                                                    on_pick_color: {
                                                        let user_id = *user_id;
                                                        move |color| {
                                                            edit_field.set(None);
                                                            cx.spawn(set_user_color(model.clone(), user_id, color));
                                                        }
                                                    },
                                                }
                                            },
                                            _ => rsx!{
                                                div {
                                                    class: "flex flex-row gap-1",
                                                    div {
                                                        class: "w-8 h-8 rounded cursor-pointer {color_picker::bg_class(&user.color)}",
                                                        onclick: {
                                                            let user_id = *user_id;
                                                            move |_| {
                                                                color.set(model.read().users[&user_id].color);
                                                                edit_field.set(Some((row_index, Column::Color)));
                                                            }
                                                        },
                                                    },
                                                    svg {
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        fill: "none",
                                                        "viewBox": "0 0 24 24",
                                                        "stroke-width": "1.5",
                                                        stroke: "currentColor",
                                                        class: "w-4 h-4",
                                                        onclick: {
                                                            let user_id = *user_id;
                                                            move |_| {
                                                                color.set(model.read().users[&user_id].color);
                                                                edit_field.set(Some((row_index, Column::Color)));
                                                            }
                                                        },
                                                        path {
                                                            "stroke-linecap": "round",
                                                            "stroke-linejoin": "round",
                                                            d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                                                        }
                                                    }
                                                }
                                            },
                                        }
                                    }
                                    td {
                                        class: "p-3",
                                        match **edit_field {
                                            Some((edit_row, Column::Name)) if edit_row == row_index => rsx! {
                                                input {
                                                    r#type: "text",
                                                    value: "{name}",
                                                    class: "bg-inherit rounded text-sm",
                                                    oninput: |event| name.set(event.data.value.clone()),
                                                    onfocusout: {
                                                        let user_id = *user_id;
                                                        move |_| {
                                                            edit_field.set(None);
                                                            set_user_name(model.clone(), user_id, name.to_string())
                                                        }
                                                    },
                                                }
                                            },
                                            _ => rsx!{
                                                div {
                                                    class: "flex flex-row gap-1",
                                                    p {
                                                        onclick: {
                                                            let user_id = *user_id;
                                                            move |_| {
                                                                name.set(model.read().users[&user_id].name.clone());
                                                                edit_field.set(Some((row_index, Column::Name)));
                                                            }
                                                        },
                                                        "{user.name}"
                                                    }
                                                    svg {
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        fill: "none",
                                                        "viewBox": "0 0 24 24",
                                                        "stroke-width": "1.5",
                                                        stroke: "currentColor",
                                                        class: "w-4 h-4",
                                                        onclick: {
                                                            let user_id = *user_id;
                                                            move |_| {
                                                                name.set(model.read().users[&user_id].name.clone());
                                                                edit_field.set(Some((row_index, Column::Name)));
                                                            }
                                                        },
                                                        path {
                                                            "stroke-linecap": "round",
                                                            "stroke-linejoin": "round",
                                                            d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                                                        }
                                                    }
                                                }
                                            },
                                        }
                                    }
                                    td {
                                        class: "p-3",
                                        div {
                                            class: "grid grid-rows-1 place-items-end text-red-600",
                                            svg {
                                                xmlns: "http://www.w3.org/2000/svg",
                                                fill: "none",
                                                "viewBox": "0 0 24 24",
                                                "stroke-width": "1.5",
                                                stroke: "currentColor",
                                                class: "w-6 h-6 cursor-pointer",
                                                onclick: {
                                                    let user_id = *user_id;
                                                    move |_| {
                                                        delete_user(model.clone(), user_id)
                                                    }
                                                },
                                                path {
                                                    "stroke-linecap": "round",
                                                    "stroke-linejoin": "round",
                                                    d: "m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0",
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
            div {
                class: styles::BOTTOM_BAR,
                button {
                    r#type: "button",
                    class: styles::BOTTOM_BAR_BUTTON,
                    onclick: |_| {
                        nav.push(Route::Board {
                            board_name: board_name.clone(),
                        });
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "
                            w-6 h-6 text-gray-400
                            group-active:text-blue-500
                            sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M9 4.5v15m6-15v15m-10.875 0h15.75c.621 0 1.125-.504 1.125-1.125V5.625c0-.621-.504-1.125-1.125-1.125H4.125C3.504 4.5 3 5.004 3 5.625v12.75c0 .621.504 1.125 1.125 1.125Z"
                        }
                    }
                }
                button {
                    r#type: "button" ,
                    class: styles::BOTTOM_BAR_BUTTON,
                    onclick: |_| {
                        nav.push(Route::AddUser {
                            board_name: board_name.clone(),
                        });
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "
                            w-6 h-6 text-gray-400
                            group-active:text-blue-500
                            sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M18 7.5v3m0 0v3m0-3h3m-3 0h-3m-2.25-4.125a3.375 3.375 0 1 1-6.75 0 3.375 3.375 0 0 1 6.75 0ZM3 19.235v-.11a6.375 6.375 0 0 1 12.75 0v.109A12.318 12.318 0 0 1 9.374 21c-2.331 0-4.512-.645-6.374-1.766Z",
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

async fn set_user_name(model: UseSharedState<Model>, user_id: UserId, name: String) {
    if send_set_user_name_request(model.clone(), user_id, name)
        .await
        .is_ok()
    {
        requests::board(model).await;
    }
}

async fn send_set_user_name_request(
    model: UseSharedState<Model>,
    user_id: UserId,
    name: String,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/users/{}/name",
            model.board_name, user_id
        ))?
    };
    Ok(reqwest::Client::new()
        .put(url)
        .json(&name)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn delete_user(model: UseSharedState<Model>, user_id: UserId) {
    if send_delete_user_request(model.clone(), user_id)
        .await
        .is_ok()
    {
        requests::board(model).await;
    }
}

async fn send_delete_user_request(
    model: UseSharedState<Model>,
    user_id: UserId,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/users/{}",
            model.board_name, user_id
        ))?
    };
    Ok(reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await?)
}
