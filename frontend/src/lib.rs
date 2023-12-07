use std::collections::HashMap;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use shared_models::{TaskId, TaskSize, TaskStatus, UserData, UserId};

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
    let model = use_shared_state_provider(cx, Model::default);
    cx.render(rsx! {
        Board {}
    })
}

#[allow(non_snake_case)]
fn Board(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "grid-cols-3",
            display: "grid",
            ToDoColumn {},
            InProgressColumn {},
            DoneColumn {},
        }
    })
}

#[allow(non_snake_case)]
fn ToDoColumn(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap().read();
    cx.render(rsx! {
        div {
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

#[allow(non_snake_case)]
fn InProgressColumn(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap().read();
    cx.render(rsx! {
        div {
            div { "In Progress" },
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

#[allow(non_snake_case)]
fn DoneColumn(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap().read();
    cx.render(rsx! {
        div {
            div { "Done" },
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

#[allow(non_snake_case)]
#[inline_props]
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

async fn tasks() {
    todo!()
}

async fn users() -> HashMap<UserId, UserData> {
    todo!()
}

struct TaskData {
    title: String,
    description: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    due: Option<DateTime<Utc>>,
    size: TaskSize,
    status: TaskStatus,
    assignees: Vec<UserId>,
    blocks: Vec<TaskId>,
    blocked_by: Vec<TaskId>,
}

struct Model {
    tasks: HashMap<TaskId, TaskData>,
    users: HashMap<UserId, UserData>,
    to_do: Vec<TaskId>,
    in_progress: Vec<TaskId>,
    done: Vec<TaskId>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            tasks: HashMap::default(),
            users: HashMap::default(),
            to_do: Vec::default(),
            in_progress: Vec::default(),
            done: Vec::default(),
        }
    }
}
