use reqwest::{Client, Url};
use std::{collections::HashMap, str::FromStr};
use tokio::join;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use shared_models::{
    BoardName, TaskEntry, TaskId, TaskSize, TaskStatus, UserData, UserEntry, UserId,
};

enum Page {
    Board,
    AddUser,
}

#[component]
pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, Model::default);
    use_shared_state_provider(cx, || Page::Board);
    let page = use_shared_state::<Page>(cx).unwrap();
    cx.render(rsx! {
        match *page.read() {
            Page::Board => rsx!(div {
                class: "grid grid-flow-row",
                BoardSettings {},
                Board {}
                button {
                    onclick: |_| {
                        *page.write() = Page::AddUser;
                    },
                    "Add User",
                }
            }),
            Page::AddUser => rsx!(
                AddUserForm {},
            )
        }
    })
}

#[component]
fn BoardSettings(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    cx.render(rsx! {
        div {
            label {
                "Board Name: "
                input {
                    value: "{model.read().board_name}",
                    oninput: |event| {
                        model.write().board_name = event.data.value.clone().into();
                    },
                },
            }
            button {
                onclick: move |_| {
                    cx.spawn(request_board_data(model.clone()));
                },
                "Load",
            },
        }
    })
}

async fn request_board_data(model: UseSharedState<Model>) {
    if let (Ok(users), Ok(tasks)) = {
        let url = &model.read().url;
        let board_name = &model.read().board_name;
        join!(users(&url, &board_name), tasks(&url, &board_name))
    } {
        let mut model = model.write();
        model.users = users;
        model.tasks = tasks.tasks;
        model.to_do = tasks.to_do;
        model.in_progress = tasks.in_progress;
        model.done = tasks.done;
    }
}

#[component]
fn Board(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "grid grid-cols-3",
            ToDoColumn {},
            InProgressColumn {},
            DoneColumn {},
        },
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
    let data = &model.tasks[task_id];
    cx.render(rsx! {
        div {
            "{data.title}"
        }
    })
}

#[component]
fn AddUserForm(cx: Scope) -> Element {
    let name = use_state(cx, String::default);
    let page = use_shared_state::<Page>(cx).unwrap();
    cx.render(rsx! {
        div {
            class: "grid grid-flow-row",
            label {
                "Name: ",
                input {
                    value: "{name}",
                    oninput: |event| name.set(event.data.value.clone())
                }
            }
            button {
                onclick: |_| {
                    *page.write() = Page::Board;
                    // cx.spawn(create_user())
                },
                "Add User"
            }

        }
    })
}

#[derive(Default, Debug)]
struct Tasks {
    tasks: HashMap<TaskId, TaskData>,
    to_do: Vec<TaskId>,
    in_progress: Vec<TaskId>,
    done: Vec<TaskId>,
}

async fn tasks(url: &Url, board_name: &BoardName) -> Result<Tasks, anyhow::Error> {
    let client = Client::new();
    Ok(client
        .get(url.join(&format!("/api/boards/{board_name}/tasks"))?)
        .send()
        .await?
        .json::<Vec<TaskEntry>>()
        .await?
        .into_iter()
        .fold(Tasks::default(), |mut tasks, task| {
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

async fn users(
    url: &Url,
    board_name: &BoardName,
) -> Result<HashMap<UserId, UserData>, anyhow::Error> {
    let client = Client::new();
    Ok(client
        .get(url.join(&format!("/api/boards/{board_name}/users"))?)
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

async fn create_user(board_name: &BoardName, name: &str) -> Result<UserId, anyhow::Error> {
    todo!()
}

#[derive(Clone, Debug)]
struct TaskData {
    title: String,
    description: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    due: Option<DateTime<Utc>>,
    size: TaskSize,
    assignees: Vec<UserId>,
    blocks: Vec<TaskId>,
    blocked_by: Vec<TaskId>,
}

struct Model {
    url: Url,
    board_name: BoardName,
    tasks: HashMap<TaskId, TaskData>,
    users: HashMap<UserId, UserData>,
    to_do: Vec<TaskId>,
    in_progress: Vec<TaskId>,
    done: Vec<TaskId>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            url: Url::from_str("http://localhost:8080").unwrap(),
            board_name: BoardName::from(""),
            tasks: HashMap::default(),
            users: HashMap::default(),
            to_do: Vec::default(),
            in_progress: Vec::default(),
            done: Vec::default(),
        }
    }
}
