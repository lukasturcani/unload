use std::collections::HashMap;

use crate::model::{Model, TaskData, Tasks};
use crate::styles;
use dioxus::prelude::*;
use reqwest::Client;
use shared_models::{BoardName, TaskEntry, TaskId, TaskStatus, UserData, UserEntry, UserId};
use tokio::join;

#[component]
pub fn Board(cx: Scope, board_name: BoardName) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    use_future(cx, (), |_| request_board_data(model.clone()));
    cx.render(rsx! {
        div {
            class: "bg-gray-900 h-screen w-screen",
            div {
                class: "grid grid-cols-3",
                ToDoColumn {},
                InProgressColumn {},
                DoneColumn {},
            },
            button {
                class: styles::BUTTON,
                onclick: |_| {},
                "Add User",
            }
            button {
                class: styles::BUTTON,
                onclick: |_| {},
                "Show Users",
            }
            button {
                class: styles::BUTTON,
                onclick: |_| {},
                "Add Task",
            }
        }
    })
}

#[component]
fn ToDoColumn(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap().read();
    cx.render(rsx! {
        div {
            class: "grid grid-cols-1",
            div { "To Do" },
            div {
                for task_id in model.to_do.iter() {
                    Task {
                        key: "{task_id}",
                        task_id: *task_id,
                    }
                }
            },
        }
    })
}

#[component]
fn InProgressColumn(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap().read();
    cx.render(rsx! {
        div {
            class: "grid grid-cols-1",
            div { "In Progress" },
            div {
                for task_id in model.in_progress.iter() {
                    Task {
                        key: "{task_id}",
                        task_id: *task_id,
                    }
                }
            },
        }
    })
}

#[component]
fn DoneColumn(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap().read();
    cx.render(rsx! {
        div {
            class: "grid grid-cols-1",
            div { "Done" },
            div {
                for task_id in model.done.iter() {
                    Task {
                        key: "{task_id}",
                        task_id: *task_id,
                    }
                }
            },
        }
    })
}

#[component]
fn Task(cx: Scope, task_id: TaskId) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap().read();
    let expanded = use_state(cx, || false);
    let data = model.tasks.get(task_id);
    cx.render(rsx! {
        div {
            "{data.title}"
        }
    })
}

async fn request_board_data(model: UseSharedState<Model>) {
    if let (Ok(users), Ok(tasks)) = join!(users(&model), tasks(&model)) {
        let mut model = model.write();
        model.users = users;
        model.tasks = tasks.tasks;
        model.to_do = tasks.to_do;
        model.in_progress = tasks.in_progress;
        model.done = tasks.done;
    }
}

async fn users(model: &UseSharedState<Model>) -> Result<HashMap<UserId, UserData>, anyhow::Error> {
    let url = {
        let model = model.read();
        model
            .url
            .join(&format!("/api/boards/{}/users", model.board_name))?
    };
    Ok(Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<UserEntry>>()
        .await?
        .into_iter()
        .fold(HashMap::new(), |mut users, user| {
            users.insert(
                user.id,
                UserData {
                    name: user.name,
                    color: user.color,
                },
            );
            users
        }))
}

async fn tasks(model: &UseSharedState<Model>) -> Result<TasksResponse, anyhow::Error> {
    let url = {
        let model = model.read();
        model
            .url
            .join(&format!("/api/boards/{}/tasks", model.board_name))?
    };
    let client = Client::new();
    Ok(client
        .get(url)
        .send()
        .await?
        .json::<Vec<TaskEntry>>()
        .await?
        .into_iter()
        .fold(TasksResponse::default(), |mut tasks, task| {
            tasks.tasks.insert(
                task.id,
                TaskData {
                    title: task.title,
                    description: task.description,
                    created: task.created,
                    updated: task.updated,
                    due: task.due,
                    size: task.size,
                    assignees: task.assignees,
                    blocks: task.blocks,
                    blocked_by: task.blocked_by,
                },
            );
            match task.status {
                TaskStatus::ToDo => tasks.to_do.push(task.id),
                TaskStatus::InProgress => tasks.in_progress.push(task.id),
                TaskStatus::Done => tasks.done.push(task.id),
            }
            tasks
        }))
}

#[derive(Default, Debug)]
struct TasksResponse {
    tasks: Tasks,
    to_do: Vec<TaskId>,
    in_progress: Vec<TaskId>,
    done: Vec<TaskId>,
}

impl From<Vec<TaskEntry>> for TasksResponse {
    fn from(value: Vec<TaskEntry>) -> Self {
        let mut to_do = Vec::new();
        let mut in_progress = Vec::new();
        let mut done = Vec::new();
        let mut tasks = Tasks::with_capacity(value.len());
        for task in value {
            tasks.insert(
                task.id,
                TaskData {
                    title: task.title,
                    description: task.description,
                    created: task.created,
                    updated: task.updated,
                    due: task.due,
                    size: task.size,
                    assignees: task.assignees,
                    blocks: task.blocks,
                    blocked_by: task.blocked_by,
                },
            );
            match task.status {
                TaskStatus::ToDo => to_do.push(task.id),
                TaskStatus::InProgress => in_progress.push(task.id),
                TaskStatus::Done => done.push(task.id),
            }
        }
        Self {
            tasks,
            to_do,
            in_progress,
            done,
        }
    }
}
