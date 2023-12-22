use reqwest::{Client, Url};
use std::{collections::HashMap, str::FromStr};
use tokio::join;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use shared_models::{
    BoardName, Color, TaskEntry, TaskId, TaskSize, TaskStatus, UserData, UserEntry, UserId,
};
use tasks::Tasks;

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
    let add_task_blocked_by = use_ref(cx, Vec::new);
    let add_task_blocks = use_ref(cx, Vec::new);

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
                                on_select_task: |task_id| add_task_blocked_by.write().push(task_id),
                                on_remove_task: |task_id| add_task_blocked_by.write().retain(|&value| value != task_id),
                            },
                        }
                        div {
                            class: "mb-5",
                            TaskSearch{
                                id: "blocks_search",
                                on_select_task: |task_id| add_task_blocks.write().push(task_id),
                                on_remove_task: |task_id| add_task_blocks.write().retain(|&value| value != task_id),
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
    let data = model.tasks.get(task_id);
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
fn TaskSearch<'a>(
    cx: Scope<'a>,
    id: &'a str,
    on_select_task: EventHandler<'a, TaskId>,
    on_remove_task: EventHandler<'a, TaskId>,
) -> Element<'a> {
    let model = use_shared_state::<Model>(cx).unwrap();
    let has_input_focus = use_state(cx, || false);
    let search_input = use_state(cx, String::default);
    let selected = use_ref(cx, Vec::<(TaskId, String)>::new);
    let dropdown_data = has_input_focus.then(|| {
        if search_input.is_empty() {
            model.read().most_recent_titles()
        } else {
            model.read().find_titles(search_input)
        }
    });
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
                onfocusin: |_| has_input_focus.set(true),
                onfocusout: |_| has_input_focus.set(false),
                oninput: |event| search_input.set(event.data.value.clone())
            },
        },
        if let Some(suggestions) = dropdown_data {
            if suggestions.is_empty() {rsx!{
                div {
                    class: "mt-2 z-10 bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-700 focus:border-blue-500",
                    ul {
                        class: "py-2 text-sm text-gray-700 dark:text-gray-200 focus:border-blue-500",
                        li {
                            class: "italic text-gray-500 dark:text-gray-400 block text-left w-full px-4 py-2",
                            prevent_default: "onmousedown",
                            onmousedown: |_| {},
                            "No matches"
                        },
                    }
                }
            }} else {rsx!{
                div {
                    class: "mt-2 z-10 bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-700 focus:border-blue-500",
                    ul {
                        class: "py-2 text-sm text-gray-700 dark:text-gray-200 focus:border-blue-500",
                        rsx!{
                            for task in suggestions {rsx!{
                                li {
                                    class: "focus:border-blue-500",
                                    // TODO: check key have correct value
                                    key: "{task.1}",
                                    button {
                                        r#type: "button",
                                        class: "block text-left w-full px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white focus:border-blue-500",
                                        prevent_default: "onmousedown",
                                        onmousedown: |_| {},
                                        onclick: move |_| {
                                            selected.write().push(task.clone());
                                            on_select_task.call(task.0);
                                        },
                                        task.1.clone(),
                                    }
                                },
                            }}
                        }
                    }
                }}
            }
        }
        div {
            class: "mt-2",
            for task in selected.read().iter().map(|x| x.clone()) {rsx!{
                span {
                    class: "inline-flex items-center px-2 py-1 me-2 mt-2 text-sm font-medium text-gray-800 bg-gray-100 rounded dark:bg-gray-700 dark:text-gray-300",
                    task.1.clone(),
                    button {
                        r#type: "button",
                        class: "inline-flex items-center p-1 ms-2 text-sm text-gray-400 bg-transparent rounded-sm hover:bg-gray-200 hover:text-gray-900 dark:hover:bg-gray-600 dark:hover:text-gray-300",
                        "aria-label": "Remove",
                        onclick: move |_| {
                            selected.write().retain(|this| this.0 != task.0);
                            on_remove_task.call(task.0);
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
                        span {
                            class: "sr-only",
                            "Remove badge",
                        },
                    },
                },
            }},
        },
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
    tasks: Tasks,
    users: HashMap<UserId, UserData>,
    to_do: Vec<TaskId>,
    in_progress: Vec<TaskId>,
    done: Vec<TaskId>,
}

impl Model {
    fn most_recent_titles(&self) -> Vec<(TaskId, String)> {
        let tasks = self.tasks.tasks();
        let mut titles = Vec::with_capacity(self.tasks.most_recently_updated().len());
        for (_, task_id) in self.tasks.most_recently_updated() {
            titles.push((*task_id, tasks[task_id].title.clone()))
        }
        titles
    }

    fn find_titles(&self, search_input: &str) -> Vec<(TaskId, String)> {
        self.tasks
            .tasks()
            .iter()
            .filter(|(task_id, task)| {
                task.title.find(search_input).is_some()
                    || task.description.find(search_input).is_some()
            })
            .map(|(task_id, task)| (*task_id, task.title.clone()))
            .collect()
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            url: Url::from_str("http://localhost:8080").unwrap(),
            board_name: BoardName::from(""),
            tasks: Tasks::default(),
            users: HashMap::default(),
            to_do: Vec::default(),
            in_progress: Vec::default(),
            done: Vec::default(),
        }
    }
}

mod tasks {
    use super::TaskData;
    use shared_models::TaskId;
    use std::collections::{BinaryHeap, HashMap};

    const NUM_MOST_RECENTLY_UPDATED: usize = 5;

    #[derive(Default, Debug)]
    pub struct Tasks {
        tasks: HashMap<TaskId, TaskData>,
        most_recently_updated: BinaryHeap<(i64, TaskId)>,
    }

    impl Tasks {
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                tasks: HashMap::with_capacity(capacity),
                most_recently_updated: BinaryHeap::with_capacity(NUM_MOST_RECENTLY_UPDATED),
            }
        }

        pub fn get(&self, task_id: &TaskId) -> &TaskData {
            &self.tasks[task_id]
        }

        pub fn insert(&mut self, task_id: TaskId, task_data: TaskData) -> Option<TaskData> {
            self.most_recently_updated
                .retain(|(task_id, _)| task_id != task_id);
            let timestamp = task_data.updated.timestamp();
            if self.most_recently_updated.len() < NUM_MOST_RECENTLY_UPDATED
                || self
                    .most_recently_updated
                    .peek()
                    .map_or(false, |value| timestamp < value.0)
            {
                self.most_recently_updated.pop();
                self.most_recently_updated.push((timestamp, task_id));
            }
            self.tasks.insert(task_id, task_data)
        }

        pub fn tasks(&self) -> &HashMap<TaskId, TaskData> {
            &self.tasks
        }

        pub fn most_recently_updated(&self) -> &BinaryHeap<(i64, TaskId)> {
            &self.most_recently_updated
        }
    }
}
