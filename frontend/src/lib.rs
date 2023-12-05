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
#[inline_props]
fn Board(
    cx: Scope,
    to_do_tasks: Vec<TaskData>,
    in_progress_tasks: Vec<TaskData>,
    done_tasks: Vec<TaskData>,
) -> Element {
    cx.render(rsx! {
        div {
            class: "grid-cols-3",
            display: "grid",
            TaskColumn {
                title: "To Do",
                tasks: to_do_tasks,
            },
            TaskColumn {
                title: "In Progress",
                tasks: in_progress_tasks,
            },
            TaskColumn {
                title: "Done",
                tasks: done_tasks,
            },
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn TaskColumn<'a>(cx: Scope, title: &'a str, tasks: &'a Vec<TaskData>) -> Element {
    cx.render(rsx! {
        div {
            div { "{title}" },
            div { "items" },
        }
    })
}

#[allow(non_snake_case)]
fn CollapsedTask(cx: Scope<CollapsedTaskData>) -> Element {
    cx.render(rsx! {
        div {
            "{cx.props.title}"
        }
    })
}

#[allow(non_snake_case)]
fn ExpandedTask(cx: Scope<ExpandedTaskData>) -> Element {
    cx.render(rsx! {
        div {
            "{cx.props.title}"
        }
    })
}

#[derive(PartialEq, Eq, Props)]
struct CollapsedTaskData {
    title: String,
    due: Option<DateTime<Utc>>,
    size: TaskSize,
    assignees: Vec<UserData>,
}

#[derive(PartialEq, Eq, Props)]
struct ExpandedTaskData {
    title: String,
    description: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    due: Option<DateTime<Utc>>,
    size: TaskSize,
    assignees: Vec<UserData>,
    blocks: Vec<TaskLink>,
    blocked_by: Vec<TaskLink>,
}

#[derive(PartialEq, Eq)]
enum TaskData {
    Collapsed(CollapsedTaskData),
    Expanded(ExpandedTaskData),
}

#[derive(PartialEq, Eq)]
struct TaskLink {
    task_id: TaskId,
    title: String,
}

async fn tasks() {
    todo!()
}

async fn users() -> HashMap<UserId, UserData> {
    todo!()
}
