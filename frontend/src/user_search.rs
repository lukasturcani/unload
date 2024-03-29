use std::collections::HashSet;

use dioxus::prelude::*;
use itertools::Itertools;
use reqwest::Client;
use shared_models::{TaskId, UserData, UserId};

use crate::{color_picker::ColorPicker, model::Model, requests, styles};

#[component]
pub fn UserSearch(
    cx: Scope,
    task_id: TaskId,
    badge_style: &'static str,
    ul_style: &'static str,
    hover_style: &'static str,
    text_input_style: &'static str,
) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    if model.read().user_search_created_user.is_some() {
        if let Some(user) = model.write().user_search_created_user.take() {
            spawn(add_task_assignee(model.clone(), *task_id, user.0));
        }
    }
    let read_model = model.read();
    let assignees: HashSet<_> = read_model.tasks[task_id]
        .assignees
        .iter()
        .copied()
        .collect();
    let show_add_user_button = use_state(cx, || true);
    let new_user = use_state(cx, String::new);
    cx.render(rsx! {
        div {
            class: "
                flex flex-col gap-2
            ",
            div {
                class: "flex flex-row gap-2 flex-wrap",
                for (user_id, user_name) in read_model
                    .tasks[task_id]
                    .assignees
                    .iter()
                    .map(|id| (id, &read_model.users[id].name))
                {rsx!{
                    span {
                        class: "
                            flex flex-row gap-1 items-center px-2 py-1 text-sm
                            font-medium rounded bg-gray-700 text-gray-200
                            {badge_style}
                        ",
                        user_name.clone()
                        button {
                            r#type: "button",
                            class: "
                                p-1 text-sm text-gray-400
                                bg-transparent rounded-sm
                                active:bg-gray-600 active:text-gray-200
                                sm:hover:bg-gray-600 sm:hover:text-gray-200
                            ",
                            "aria-label": "Remove",
                            onclick: {
                            let task_id = *task_id;
                            let user_id = *user_id;
                                move |event| {
                                    event.stop_propagation();
                                    delete_task_assignee(model.clone(), task_id, user_id)
                                }
                            },
                            svg {
                                class: "w-2 h-2",
                                "aria-hidden": "true",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                "viewBox": "0 0 14 14",
                                path {
                                    stroke: "currentColor",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    d: "m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6",
                                },
                            },
                        },
                    },
                }},
            }
            ul {
                class: "text-sm text-gray-200 z-10 rounded-lg shadow {ul_style}",
                rsx!{
                    for (user_id, user) in read_model
                        .users
                        .iter()
                        .filter(|(id, _)| !assignees.contains(id))
                        .sorted_by_key(|(_, user)| user.name.to_lowercase())
                    {rsx!{
                        li {
                            key: "{user_id}",
                            button {
                                r#type: "button",
                                class: "
                                    text-left w-full px-4 py-2
                                    {hover_style}
                                    active:text-white
                                    sm:hover:text-white
                                ",
                                prevent_default: "onmousedown",
                                onmousedown: |_| {},
                                onclick: {
                                    let task_id = *task_id;
                                    let user_id = *user_id;
                                    move |event| {
                                        event.stop_propagation();
                                        add_task_assignee(model.clone(), task_id, user_id)
                                    }
                                },
                                user.name.clone(),
                            }
                        },
                    }}
                }
                li {
                    key: "add user",
                    if **show_add_user_button {rsx! {
                        button {
                            r#type: "button",
                            class: "
                                text-left w-full px-4 py-2
                                {hover_style}
                                font-medium text-blue-500
                                active:underline
                                sm:hover:underline
                            ",
                            prevent_default: "onmousedown",
                            onmousedown: |_| {},
                            onclick: |_| {
                                show_add_user_button.set(false);
                            },
                            "Add User"
                        }
                    }} else {rsx! {
                        div {
                            class: "p-2",
                            div {
                                class: "flex flex-col gap-2 p-2",
                                input {
                                    class: "{styles::TEXT_INPUT} {text_input_style}",
                                    r#type: "text",
                                    placeholder: "Name",
                                    value: "{new_user}",
                                    oninput: |event| {
                                        new_user.set(event.value.clone())
                                    },
                                }
                                ColorPicker {
                                    on_pick_color: |color| {
                                        show_add_user_button.set(true);
                                        if new_user.trim().is_empty() {
                                            return;
                                        }
                                        cx.spawn(create_user(
                                            model.clone(),
                                            UserData {
                                                name: new_user.make_mut().drain(..).collect(),
                                                color
                                            },
                                        ));
                                    },
                                }
                            }
                        }
                    }}
                }
            }
        }
    })
}

async fn create_user(model: UseSharedState<Model>, user_data: UserData) {
    if let Ok(user_data) = requests::create_user(model.clone(), user_data).await {
        requests::board(model.clone()).await;
        model.write().user_search_created_user = Some(user_data);
    }
}

async fn delete_task_assignee(model: UseSharedState<Model>, task_id: TaskId, assignee: UserId) {
    if send_delete_task_assignee_request(model.clone(), task_id, assignee)
        .await
        .is_ok()
    {
        requests::board(model).await;
    }
}

async fn send_delete_task_assignee_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
    assignee: UserId,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/assignees/{}",
            model.board_name, task_id, assignee
        ))?
    };
    Ok(Client::new().delete(url).send().await?.json::<()>().await?)
}

async fn add_task_assignee(model: UseSharedState<Model>, task_id: TaskId, assignee: UserId) {
    if send_add_task_assignee_request(model.clone(), task_id, assignee)
        .await
        .is_ok()
    {
        requests::board(model.clone()).await;
    }
}

async fn send_add_task_assignee_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
    assignee: UserId,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/assignees",
            model.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .post(url)
        .json(&assignee)
        .send()
        .await?
        .json::<()>()
        .await?)
}
