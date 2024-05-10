use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use reqwest::Client;
use shared_models::{TagId, TaskId, TaskStatus, UserId};

use crate::{
    model::{Model, TaskData},
    requests,
};

#[component]
pub fn Task(task_id: TaskId, task: TaskData) -> Element {
    let style = "
        border border-gray-700
        rounded-lg
        shadow
        bg-gray-800 sm:hover:bg-gray-700
    ";
    let expanded = use_signal(|| false);
    rsx! {
        div {
            class: "flex flex-col gap-2 p-3 {style}",
            div {
                class: "flex flex-row justify-between",
                Title { task_id, title: task.title }
                StatusButtons { task_id }
            }
            // div {
            //     class: "",
            //     Users { task_id, users: task.assignees }
            //     TaskActions { task_id }
            // }
            // Tags { task_id, tags: task.tags }
            // if expanded() {
            //     Due { task_id, due: task.due }
            //     Description { task_id, description: task.description }
            //     SpecialActions { task_id }
            // }
            // ToggleExpanded {
            //     expanded
            // }
        }
    }
}

#[component]
fn Title(task_id: TaskId, title: String) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            TitleInput { task_id, editing, title }
        } else {
            TitleShow { editing, title }
        }
    }
}

#[component]
fn TitleInput(task_id: TaskId, editing: Signal<bool>, title: String) -> Element {
    let style = "
        text-base
        rounded-lg
        border border-gray-600
        bg-gray-700
        focus:ring-blue-500 focus:border-blue-500
    ";
    let mut title = use_signal(|| title);
    let model = use_context::<Signal<Model>>();
    rsx! {
        div {
            form {
                class: "flex gap-2 items-center",
                onsubmit: move |_| {
                    spawn_forever(set_task_title(model, task_id, title()));
                    editing.set(false);
                },
                input {
                    required: true,
                    class: "p-2.5 {style}",
                    oninput: move |event| title.set(event.value()),
                    value: title
                }
                ConfirmButton {}
                CancelButton { editing }
            }
        }
    }
}

#[component]
fn ConfirmButton() -> Element {
    let style = "
        rounded-md
        border border-green-500
        stroke-green-500
        active:bg-green-500
        sm:hover:bg-green-500 sm:hover:stroke-white
    ";
    rsx! {
        button {
            class: style,
            r#type: "submit",
            ConfirmIcon {}
        }
    }
}

#[component]
fn CancelButton(editing: Signal<bool>) -> Element {
    let style = "
        rounded-md
        border border-red-500
        stroke-red-500
        active:bg-red-500
        sm:hover:bg-red-500 sm:hover:stroke-white
    ";
    rsx! {
        button {
            class: style,
            onclick: move |_| {
                editing.set(false);
            },
            CancelIcon {}
        }
    }
}

#[component]
fn TitleShow(editing: Signal<bool>, title: String) -> Element {
    rsx! {
        div {
            class: "flex gap-2 items-center",
            h3 {
                class: "
                    text-lg sm:text-xl
                    font-bold tracking-tight
                    underline underline-offset-8
                ",
                {title}
            }
            button {
                onclick: move |_| {
                    editing.set(true);
                },
                EditIcon {
                    style: "size-4",
                }
            }
        }
    }
}

#[component]
fn Description(task_id: TaskId, description: String) -> Element {
    todo!()
}

#[component]
fn SpecialActions(task_id: TaskId) -> Element {
    todo!()
}

#[component]
fn StatusButtons(task_id: TaskId) -> Element {
    let model = use_context::<Signal<Model>>();
    rsx! {
        div {
            button {
                class: "active:stroke-red-600 sm:hover:stroke-red-600",
                onclick: move |_| {
                    spawn_forever(set_task_status(model, task_id, TaskStatus::ToDo));
                },
                ToDoIcon {}
            }
            button {
                class: "active:stroke-yellow-300 sm:hover:stroke-yellow-300",
                onclick: move |_| {
                    spawn_forever(set_task_status(model, task_id, TaskStatus::InProgress));
                },
                InProgressIcon {}
            }
            button {
                class: "active:stroke-green-500 sm:hover:stroke-green-500",
                onclick: move |_| {
                    spawn_forever(set_task_status(model, task_id, TaskStatus::Done));
                },
                DoneIcon {}
            }
        }
    }
}

#[component]
fn ToDoIcon() -> Element {
    rsx! {
        Icon {
            style: "size-8",
            d: "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
        }
    }
}

#[component]
fn InProgressIcon() -> Element {
    rsx! {
        Icon {
            style: "size-8",
            d: "M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
        }
    }
}

#[component]
fn DoneIcon() -> Element {
    rsx! {
        Icon {
            style: "size-8",
            d: "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
        }
    }
}

#[component]
fn BoltIcon() -> Element {
    todo!()
}

#[component]
fn CopyIcon() -> Element {
    todo!()
}

#[component]
fn ArchiveIcon() -> Element {
    todo!()
}

#[component]
fn Users(task_id: TaskId, users: Vec<UserId>) -> Element {
    todo!()
}

#[component]
fn Tags(task_id: TaskId, tags: Vec<TagId>) -> Element {
    todo!()
}

#[component]
fn TaskActions(task_id: TaskId) -> Element {
    todo!()
}

#[component]
fn Due(task_id: TaskId, due: Option<DateTime<Utc>>) -> Element {
    todo!()
}

#[component]
fn ToggleExpanded(expanded: Signal<bool>) -> Element {
    todo!()
}

#[component]
fn EditIcon(style: &'static str) -> Element {
    rsx! {
        Icon {
            style,
            d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10",
        }
    }
}

#[component]
fn ConfirmIcon() -> Element {
    rsx! {
        Icon {
            style: "size-6",
            d: "m4.5 12.75 6 6 9-13.5",
        }
    }
}

#[component]
fn CancelIcon() -> Element {
    rsx! {
        Icon {
            style: "size-6",
            d: "M6 18 18 6M6 6l12 12",
        }
    }
}

#[component]
fn Icon(style: &'static str, d: &'static str) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            "viewBox": "0 0 24 24",
            "stroke-width": "1.5",
            class: style,
            path {
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                d,
            }
        }
    }
}

async fn set_task_title(model: Signal<Model>, task_id: TaskId, title: String) {
    if send_set_task_title_request(model, task_id, title)
        .await
        .is_ok()
    {
        requests::board(model).await;
    }
}

async fn send_set_task_title_request(
    model: Signal<Model>,
    task_id: TaskId,
    title: String,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/title",
            model.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .put(url)
        .json(&title)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn set_task_status(model: Signal<Model>, task_id: TaskId, status: TaskStatus) {
    if send_set_task_status_request(model, task_id, status)
        .await
        .is_ok()
    {
        requests::board(model).await;
    }
}

async fn send_set_task_status_request(
    model: Signal<Model>,
    task_id: TaskId,
    status: TaskStatus,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/status",
            model.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .put(url)
        .json(&status)
        .send()
        .await?
        .json::<()>()
        .await?)
}
