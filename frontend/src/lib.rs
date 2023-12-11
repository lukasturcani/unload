use reqwest::{Client, Url};
use std::{collections::HashMap, str::FromStr};
use tokio::join;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use shared_models::{
    BoardName, Color, TaskEntry, TaskId, TaskSize, TaskStatus, UserData, UserEntry, UserId,
};

enum Page {
    Board,
    AddUser,
    ShowUsers,
}

const BUTTON_CLASS: &str = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800";
const TEXT_INPUT_CLASS: &str = "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 focus:outline-none";

#[component]
pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, Model::default);
    use_shared_state_provider(cx, || Page::Board);
    let model = use_shared_state::<Model>(cx).unwrap();
    let page = use_shared_state::<Page>(cx).unwrap();
    cx.render(rsx! {
        match *page.read() {
            Page::Board => rsx!(div {
                class: "bg-gray-900 h-screen w-screen",
                BoardSettings {},
                Board {}
                button {
                    class: BUTTON_CLASS,
                    onclick: |_| {
                        *page.write() = Page::AddUser;
                    },
                    "Add User",
                }
                button {
                    class: BUTTON_CLASS,
                    onclick: |_| {
                        *page.write() = Page::ShowUsers;
                    },
                    "Show Users",
                }
            }),
            Page::AddUser => rsx!{
                div {
                    class: "bg-gray-900 h-screen w-screen",
                    AddUserForm {},
                }
            },
            Page::ShowUsers => rsx!{
                div {
                    class: "bg-gray-900 h-screen w-screen",
                    for (id, user) in model.read().users.iter() {
                        div {
                            key: "{id}",
                            class: "bg-gray-700 rounded-lg p-2.5 m-2",
                            "{user.name}"
                        }
                    }
                }
            },
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
                class: BUTTON_CLASS,
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
        join!(users(url, board_name), tasks(url, board_name))
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
    let model = use_shared_state::<Model>(cx).unwrap();
    cx.render(rsx! {
        form { class:"max-w-sm mx-auto",
            div {
                class: "mb-5",
                label {
                    r#for: "name",
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Name: "
                },
                input {
                    class: TEXT_INPUT_CLASS,
                    r#type: "text",
                    id: "name",
                    placeholder: "Scarlett",
                    required: true,
                    value: "{name}",
                    oninput: |event| {
                        name.set(event.value.clone())
                    },
                },
            }
            button {
                class: BUTTON_CLASS,
                r#type: "submit",
                onclick: |_| {
                    cx.spawn(create_user(model.clone(), (**name).clone()));
                    *page.write() = Page::Board;
                },
                "Submit"
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

async fn create_user(model: UseSharedState<Model>, name: String) {
    let color = Color::Black;
    if let Ok(user_id) = {
        let model = model.read();
        let url = &model.url;
        let board_name = &model.board_name;
        send_create_user_request(
            url,
            board_name,
            &UserData {
                name: name.to_string(),
                color,
            },
        )
        .await
    } {
        model
            .write()
            .users
            .insert(user_id, UserData { name, color });
    }
}

async fn send_create_user_request(
    url: &Url,
    board_name: &BoardName,
    user_data: &UserData,
) -> Result<UserId, anyhow::Error> {
    let client = Client::new();
    Ok(client
        .post(url.join(&format!("/api/boards/{board_name}/users"))?)
        .json(user_data)
        .send()
        .await?
        .json::<UserId>()
        .await?)
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
