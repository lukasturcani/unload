use reqwest::{Client, Url};
use std::{collections::HashMap, str::FromStr};
use tokio::join;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use shared_models::{
    BoardName, Color, TaskEntry, TaskId, TaskSize, TaskStatus, UserData, UserEntry, UserId,
};

enum Page {
    JoinBoard,
    Board,
    AddUser,
    ShowUsers,
    AddTask,
}

const BUTTON_CLASS: &str = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800";
const TEXT_INPUT_CLASS: &str = "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";
const TEXT_INPUT_LABEL_CLASS: &str = "block mb-2 text-sm font-medium text-gray-900 dark:text-white";

#[component]
pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, Model::default);
    use_shared_state_provider(cx, || Page::JoinBoard);
    let model = use_shared_state::<Model>(cx).unwrap();
    let page = use_shared_state::<Page>(cx).unwrap();

    let add_user_form_name = use_state(cx, String::default);
    let add_task_form_title = use_state(cx, String::default);

    cx.render(rsx! {
        match *page.read() {
            Page::JoinBoard => rsx!{div{
                class: "bg-gray-900 h-screen w-screen",
                form { class:"max-w-sm mx-auto",
                    div {
                        class: "mb-5",
                        label {
                            r#for: "board_name",
                            class: TEXT_INPUT_LABEL_CLASS,
                            "Board Name"
                        },
                        input {
                            class: TEXT_INPUT_CLASS,
                            r#type: "text",
                            id: "board_name",
                            required: true,
                            value: "{model.read().board_name}",
                            oninput: |event| {
                                model.write().board_name = event.value.clone().into()
                            },
                        },
                    }
                    button {
                        class: BUTTON_CLASS,
                        r#type: "submit",
                        onclick: |_| {
                            *page.write() = Page::Board;
                            request_board_data(model.clone())
                        },
                        "Join"
                    }
                },
                button {
                    class: BUTTON_CLASS,
                    onclick: |_| {
                        *page.write() = Page::Board;
                        create_board(model.clone())
                    },
                    "Create New Board",
                },
            }},
            Page::Board => rsx!(div {
                class: "bg-gray-900 h-screen w-screen",
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
                button {
                    class: BUTTON_CLASS,
                    onclick: |_| {
                        *page.write() = Page::AddTask;
                    },
                    "Add Task",
                }
            }),
            Page::AddUser => rsx!{
                div {
                    class: "bg-gray-900 h-screen w-screen",
                    form { class:"max-w-sm mx-auto",
                        div {
                            class: "mb-5",
                            label {
                                r#for: "user_name",
                                class: TEXT_INPUT_LABEL_CLASS,
                                "Name"
                            },
                            input {
                                class: TEXT_INPUT_CLASS,
                                r#type: "text",
                                id: "user_name",
                                required: true,
                                value: "{add_user_form_name}",
                                oninput: |event| {
                                    add_user_form_name.set(event.value.clone())
                                },
                            },
                        }
                        button {
                            class: BUTTON_CLASS,
                            r#type: "submit",
                            onclick: |_| {
                                *page.write() = Page::Board;
                                create_user(model.clone(), (**add_user_form_name).clone())
                            },
                            "Submit"
                        }
                    }
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
                    button {
                        class: BUTTON_CLASS,
                        onclick: |_| {
                            *page.write() = Page::Board;
                        },
                        "Back",
                    }
                }
            },
            Page::AddTask => rsx!{
                div {
                    class: "bg-gray-900 h-screen w-screen",
                    form { class:"max-w-sm mx-auto",
                        div {
                            class: "mb-5",
                            label {
                                r#for: "task_title",
                                class: TEXT_INPUT_LABEL_CLASS,
                                "Title"
                            },
                            input {
                                class: TEXT_INPUT_CLASS,
                                r#type: "text",
                                id: "task_title",
                                required: true,
                                value: "{add_task_form_title}",
                                oninput: |event| {
                                    add_task_form_title.set(event.value.clone())
                                },
                            },
                        }
                        div {
                            class: "mb-5",
                            div {
                                class: "inline-flex rounded-md shadow-sm",
                                role: "group",
                                button {
                                    r#type: "button",
                                    class: "px-4 py-2 text-sm font-medium text-gray-900 bg-transparent border border-gray-900 rounded-s-lg hover:bg-gray-900 hover:text-white focus:z-10 focus:ring-2 focus:ring-gray-500 focus:bg-gray-900 focus:text-white dark:border-white dark:text-white dark:hover:text-white dark:hover:bg-gray-700 dark:focus:bg-gray-700",
                                    "Small",
                                },
                                button {
                                    r#type: "button",
                                    class: "px-4 py-2 text-sm font-medium text-gray-900 bg-transparent border-t border-b border-gray-900 hover:bg-gray-900 hover:text-white focus:z-10 focus:ring-2 focus:ring-gray-500 focus:bg-gray-900 focus:text-white dark:border-white dark:text-white dark:hover:text-white dark:hover:bg-gray-700 dark:focus:bg-gray-700",
                                    "Medium",
                                },
                                button {
                                    r#type: "button",
                                    class: "px-4 py-2 text-sm font-medium text-gray-900 bg-transparent border border-gray-900 rounded-e-lg hover:bg-gray-900 hover:text-white focus:z-10 focus:ring-2 focus:ring-gray-500 focus:bg-gray-900 focus:text-white dark:border-white dark:text-white dark:hover:text-white dark:hover:bg-gray-700 dark:focus:bg-gray-700",
                                    "Large",
                                }
                            }
                        },
                        div {
                            class: "mb-5",
                            label {
                                r#for: "task_description" ,
                                class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                                "Description"
                            },
                            textarea {
                                r#id: "task_description",
                                rows: "4",
                                class: "block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                placeholder: "Give a description...",
                            },
                        },
                        div {
                            class: "mb-5",
                            TaskSearch{
                                id: "blocked_by_search",
                            },
                        }
                        button {
                            class: BUTTON_CLASS,
                            r#type: "submit",
                            onclick: |_| {
                                *page.write() = Page::Board;
                            },
                            "Submit"
                        }
                    }
                },
            },
        }
    })
}

async fn create_board(model: UseSharedState<Model>) {
    if let Ok(board_name) = send_create_board_request(&model).await {
        model.write().board_name = board_name;
    }
}

async fn send_create_board_request(
    model: &UseSharedState<Model>,
) -> Result<BoardName, anyhow::Error> {
    let request = {
        let model = model.read();
        let client = Client::new();
        let url = model.url.join("/api/boards")?;
        client.post(url).json(&model.board_name)
    };
    Ok(request.send().await?.json::<BoardName>().await?)
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
                    class: TEXT_INPUT_LABEL_CLASS,
                    "Name"
                },
                input {
                    class: TEXT_INPUT_CLASS,
                    r#type: "text",
                    id: "name",
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
                    cx.spawn_forever(create_user(model.clone(), (**name).clone()));
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

async fn tasks(model: &UseSharedState<Model>) -> Result<Tasks, anyhow::Error> {
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

async fn users(model: &UseSharedState<Model>) -> Result<HashMap<UserId, UserData>, anyhow::Error> {
    let url = {
        let model = model.read();
        model
            .url
            .join(&format!("/api/boards/{}/users", model.board_name))?
    };
    let client = Client::new();
    Ok(client
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

async fn create_user(model: UseSharedState<Model>, name: String) {
    let color = Color::Black;
    if let Ok(user_id) = send_create_user_request(
        &model,
        &UserData {
            name: name.to_string(),
            color,
        },
    )
    .await
    {
        model
            .write()
            .users
            .insert(user_id, UserData { name, color });
    }
}

async fn send_create_user_request(
    model: &UseSharedState<Model>,
    user_data: &UserData,
) -> Result<UserId, anyhow::Error> {
    let url = {
        let model = model.read();
        model
            .url
            .join(&format!("/api/boards/{}/users", model.board_name))?
    };
    let client = Client::new();
    Ok(client
        .post(url)
        .json(user_data)
        .send()
        .await?
        .json::<UserId>()
        .await?)
}

#[component]
fn TaskSearch<'a>(cx: Scope, id: &'a str) -> Element<'a> {
    let model = use_shared_state::<Model>(cx).unwrap();
    let has_focus = use_state(cx, || false);
    let search_input = use_state(cx, String::default);
    let selected = use_ref(cx, Vec::<String>::new);
    cx.render(rsx! {
        label {
            r#for: *id,
            class: TEXT_INPUT_LABEL_CLASS,
            "Blocked by"
        },
        div {
            class: "relative",
            div {
                class: "absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none",
                svg {
                    class: "w-4 h-4 text-gray-500 dark:text-gray-400",
                    "aria-hidden": "true",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none" ,
                    "viewBox": "0 0 20 20",
                    path {
                        d: "m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z",
                        stroke: "currentColor",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        "stroke-width", "2",
                    },
                },
            },
            input {
                r#type: "search",
                id: *id,
                class: "block w-full p-4 ps-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                placeholder: "Search",
                onfocusin: |_| has_focus.set(true),
                onfocusout: |_| has_focus.set(false),
                onchange: |event| search_input.set(event.data.value.clone())
            },
        },
        if **has_focus {rsx!{
            div {
                class: "mt-2 z-10 bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-700",
                ul {
                    class: "py-2 text-sm text-gray-700 dark:text-gray-200",
                    li {
                        class: "block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white",
                        "First title",
                    },
                    li {
                        class: "block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white",
                        "Second title",
                    },
                }
            }
        }}
    })
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
