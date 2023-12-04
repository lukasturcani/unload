use std::collections::HashMap;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use shared_models::{TaskId, TaskSize, UserData, UserId};

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Board {}
    })
}

#[allow(non_snake_case)]
fn Board(cx: Scope) -> Element {
    let to_do_tasks = HashMap::from([
        (),
        (),
    ])
    cx.render(rsx! {
        div {
            class: "grid-cols-3",
            display: "grid",
            TaskColumn {
                title: "To Do".to_string(),
                tasks: to_do_tasks,
            },
            TaskColumn {
                title: "In Progress".to_string(),
                tasks: in_progress_tasks,
            },
            TaskColumn {
                title: "Done".to_string(),
                tasks: done_tasks,
            },
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn TaskColumn(cx: Scope, title: String, tasks: HashMap<TaskId, TaskData>) -> Element {
    cx.render(rsx! {
        div {
            div { "{title}" },
            div {
                for (_, task) in tasks {
                    Task {
                        title: task.title,
                    }
                }
            },
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn Task(cx: Scope, title: String) -> Element {
    cx.render(rsx! {
        div {
            "{title}"
        }
    })
}


struct TaskData {
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub due: Option<DateTime<Utc>>,
    pub size: TaskSize,
    pub assignees: Vec<UserId>,
    pub blocks: Vec<TaskId>,
    pub blocked_by: Vec<TaskId>,
}

struct Tasks {
    to_do: HashMap<TaskId, TaskData>,
    in_progress: HashMap<TaskId, TaskData>,
    done: HashMap<TaskId, TaskData>,
}

async fn tasks() -> Tasks {
    todo!()
}

async fn users() -> HashMap<UserId, UserData> {
    todo!()
}
