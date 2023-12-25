use crate::route::Route;
use dioxus_router::hooks::use_navigator;

use crate::model::Model;
use crate::requests;
use crate::styles;
use dioxus::prelude::*;
use shared_models::{BoardName, TaskId};

#[component]
pub fn Board(cx: Scope, board_name: BoardName) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let nav = use_navigator(cx);
    if &model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    use_future(cx, (), |_| requests::board(model.clone()));
    cx.render(rsx! {
        div {
            class: "bg-gray-900 min-h-screen min-w-screen",
            div {
                class: "grid grid-cols-3",
                ToDoColumn {},
                InProgressColumn {},
                DoneColumn {},
            },
            button {
                class: styles::BUTTON,
                onclick: |_| {
                    nav.push(Route::AddUser {
                        board_name: board_name.clone(),
                    });
                },
                "Add User",
            }
            button {
                class: styles::BUTTON,
                onclick: |_| {
                    nav.push(Route::Users {
                        board_name: board_name.clone(),
                    });
                },
                "Show Users",
            }
            button {
                class: styles::BUTTON,
                onclick: |_| {
                    nav.push(Route::AddTask {
                        board_name: board_name.clone(),
                    });
                },
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
    let data = &model.tasks[task_id];
    cx.render(rsx! {
        div {
            "{data.title}"
        }
    })
}
