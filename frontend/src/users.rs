use std::str::FromStr;

use crate::color_picker::{self, SelectingColorPicker};
use crate::components::nav::NavBar;
use dioxus::prelude::*;
use itertools::Itertools;
use reqwest::Url;
use shared_models::{BoardName, Color, UserEntry, UserId};

struct UsersUrl(reqwest::Url);
struct UserEntries(Vec<UserEntry>);

#[component]
pub fn Users(board_name: BoardName) -> Element {
    let url = use_signal(|| {
        let url = Url::from_str(&web_sys::window().unwrap().origin()).unwrap();
        UsersUrl(url.join(&format!("/api/boards/{}/", board_name)).unwrap())
    });
    let users = use_signal(|| UserEntries(Vec::new()));
    use_future(move || async move {
        let url = &url.read().0;
        get_users(users, url).await;
    });
    rsx! {
        div {
            class: "
                w-screen h-dvh
                bg-gray-900
                flex flex-col
                text-white stroke-white
            ",
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
                            class: "divide-y divide-gray-700",
                            for user in users
                                .read()
                                .0
                                .iter()
                                .sorted_by_key(|user| user.name.to_lowercase())
                            {
                                UserRow {
                                    key: "{user.id}",
                                    user: user.clone(),
                                    url,
                                    users,
                                }
                            }
                        }
                    }
                }
            }
            NavBar { board_name }
        }
    }
}

#[component]
fn UserRow(user: UserEntry, url: Signal<UsersUrl>, users: Signal<UserEntries>) -> Element {
    let mut editing_color_signal = use_signal(|| false);
    let editing_color = editing_color_signal();

    let mut name = use_signal(|| None::<String>);
    rsx! {
        tr {
            class: "bg-gray-800 sm:hover:bg-gray-600",
            td {
                class: "p-3",
                if editing_color {
                    SelectingColorPicker {
                        default_color: user.color,
                        on_pick_color: move |picked_color| {
                            editing_color_signal.set(false);
                            spawn(
                                set_user_color(users, url, user.id, picked_color)
                            );
                        },
                    }
                } else {
                    div {
                        class: "flex flex-row gap-1",
                        div {
                            class: "w-8 h-8 rounded cursor-pointer {color_picker::bg_class(&user.color)}",
                            onclick: move |_| editing_color_signal.set(true),
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "w-4 h-4",
                            onclick: move |_| editing_color_signal.set(true),
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                            }
                        }
                    }

                }
            }
            td {
                class: "p-3",
                if let Some(name_value) = name() {
                    input {
                        r#type: "text",
                        value: "{name_value}",
                        class: "bg-inherit rounded text-sm",
                        oninput: move |event| name.set(Some(event.data.value())),
                        onfocusout: move |_| {
                            name.set(None);
                            set_user_name(users, url, user.id, name_value.clone())
                        },
                    }
                } else {
                    div {
                        class: "flex flex-row gap-1",
                        p {
                            onclick: {
                                let user_name = user.name.clone();
                                move |_| name.set(Some(user_name.clone()))
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
                            onclick: move |_| name.set(Some(user.name.clone())),
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                            }
                        }
                    }
                },
            }
            td {
                class: "p-3",
                div {
                    class: "grid grid-rows-1 place-items-end",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "w-6 h-6 cursor-pointer text-red-600",
                        onclick: move |_| delete_user(users, url, user.id),
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

async fn set_user_color(
    users: Signal<UserEntries>,
    url: Signal<UsersUrl>,
    user_id: UserId,
    color: Color,
) {
    let url = &url.read().0;
    let _ = send_set_user_color_request(url, user_id, color).await;
    get_users(users, url).await;
}

async fn send_set_user_color_request(
    url: &Url,
    user_id: UserId,
    color: Color,
) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("users/{}/color", user_id))?;
    Ok(reqwest::Client::new()
        .put(url)
        .json(&color)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn set_user_name(
    users: Signal<UserEntries>,
    url: Signal<UsersUrl>,
    user_id: UserId,
    name: String,
) {
    let url = &url.read().0;
    let _ = send_set_user_name_request(url, user_id, name).await;
    get_users(users, url).await;
}

async fn send_set_user_name_request(
    url: &Url,
    user_id: UserId,
    name: String,
) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("users/{}/name", user_id))?;
    Ok(reqwest::Client::new()
        .put(url)
        .json(&name)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn delete_user(users: Signal<UserEntries>, url: Signal<UsersUrl>, user_id: UserId) {
    let url = &url.read().0;
    let _ = send_delete_user_request(url, user_id).await;
    get_users(users, url).await;
}

async fn send_delete_user_request(url: &Url, user_id: UserId) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("users/{}", user_id))?;
    Ok(reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn get_users(mut users: Signal<UserEntries>, url: &Url) {
    if let Ok(result) = send_get_users_request(url).await {
        users.write().0 = result;
    }
}

async fn send_get_users_request(url: &Url) -> Result<Vec<UserEntry>, anyhow::Error> {
    let url = url.join("users")?;
    Ok(reqwest::Client::new()
        .get(url.clone())
        .send()
        .await?
        .json::<Vec<UserEntry>>()
        .await?)
}
