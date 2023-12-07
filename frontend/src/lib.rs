use std::collections::HashMap;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use shared_models::{Color, TaskId, TaskSize, TaskStatus, UserData, UserId};

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
    let model = use_ref(cx, Model::default);
    let to_do_tasks = model.read().to_do_props();
    cx.render(rsx! {
        Board {
            to_do_tasks: to_do_tasks.clone(),
            in_progress_tasks: to_do_tasks.clone(),
            done_tasks: to_do_tasks.clone(),
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn Board<'a>(
    cx: Scope,
    to_do_tasks: Vec<TaskProps<'a>>,
    in_progress_tasks: Vec<TaskProps<'a>>,
    done_tasks: Vec<TaskProps<'a>>,
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
fn TaskColumn<'a>(cx: Scope, title: &'a str, tasks: &'a Vec<TaskProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            div { "{title}" },
            div {
                for task in tasks {
                    match task {
                        TaskProps::Collapsed(props) => rsx! { CollapsedTask {
                            key: "{props.task_id}",
                            task_id: props.task_id,
                            title: props.title,
                            due: props.due,
                            size: props.size,
                            assignees: props.assignees
                        } },
                        TaskProps::Expanded(props) => rsx! { ExpandedTask {
                            key: "{props.task_id}",
                            task_id: props.task_id,
                            title: props.title,
                            description: props.description,
                            created: props.created,
                            updated: props.updated,
                            due: props.due,
                            size: props.size,
                            assignees: props.assignees,
                            blocks: props.blocks,
                            blocked_by: props.blocked_by,
                        } }
                    }
                }
            },
        }
    })
}

#[allow(non_snake_case)]
fn CollapsedTask<'a>(cx: Scope<'a, CollapsedTaskProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            "{cx.props.title}"
        }
    })
}

#[allow(non_snake_case)]
fn ExpandedTask<'a>(cx: Scope<'a, ExpandedTaskProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            "{cx.props.title}"
        }
    })
}

#[derive(PartialEq, Eq, Props, Clone)]
struct CollapsedTaskProps<'a> {
    task_id: TaskId,
    title: &'a str,
    #[props(!optional)]
    due: Option<DateTime<Utc>>,
    size: TaskSize,
    assignees: &'a Vec<UserLink<'a>>,
}

#[derive(PartialEq, Eq, Props, Clone)]
struct ExpandedTaskProps<'a> {
    task_id: TaskId,
    title: &'a str,
    description: &'a str,
    #[props(!optional)]
    created: DateTime<Utc>,
    #[props(!optional)]
    updated: DateTime<Utc>,
    #[props(!optional)]
    due: Option<DateTime<Utc>>,
    size: TaskSize,
    assignees: &'a Vec<UserLink<'a>>,
    blocks: &'a Vec<TaskLink<'a>>,
    blocked_by: &'a Vec<TaskLink<'a>>,
}

#[derive(PartialEq, Eq, Clone)]
enum TaskProps<'a> {
    Collapsed(CollapsedTaskProps<'a>),
    Expanded(ExpandedTaskProps<'a>),
}

#[derive(PartialEq, Eq, Clone)]
struct TaskLink<'a> {
    task_id: TaskId,
    title: &'a str,
    status: TaskStatus,
}

#[derive(PartialEq, Eq, Clone)]
struct UserLink<'a> {
    user_id: UserId,
    name: &'a str,
    color: Color,
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

impl Model {
    fn to_do_props(&self) -> Vec<TaskProps> {
        todo!()
    }

    fn in_progress_props(&self) -> Vec<TaskProps> {
        todo!()
    }

    fn done_props(&self) -> Vec<TaskProps> {
        todo!()
    }
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
