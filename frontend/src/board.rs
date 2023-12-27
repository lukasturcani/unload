use crate::route::Route;
use dioxus_router::hooks::use_navigator;
use reqwest::Client;
use shared_models::TaskSize;
use shared_models::TaskStatus;

use crate::color_picker;
use crate::model::Model;
use crate::requests;
use crate::styles;
use dioxus::prelude::*;
use shared_models::{BoardName, TaskId};

pub const COLUMN: &str = "
    flex flex-col gap-2 flex-1 rounded bg-gray-900 border border-gray-700 p-4
";
pub const COLUMN_HEADING: &str = "text-3xl font-extrabold text-white";
pub const COLUMN_TASK_LIST: &str = "flex flex-col gap-2 w-full h-full";
pub const TOOLTIP: &str = "
    pointer-events-none absolute -top-10 -left-2 w-max
    opacity-0 transition-opacity group-hover:opacity-100
    z-10 inline-block px-3 py-2 text-sm font-medium text-white
    rounded-lg shadow-sm opacity-0 tooltip bg-gray-800
    border border-gray-700";

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
            class: "flex flex-col bg-gray-900 min-h-screen min-w-screen",
            div {
                class: "flex flex-col flex-1 my-5 mx-5",
                div {
                    class: "flex-1 flex flex-cols-3 gap-2",
                    ToDoColumn {},
                    InProgressColumn {},
                    DoneColumn {},
                },
            }
            div {
                class: "flex flex-row justify-center gap-2 mb-4",
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
        }
    })
}

#[component]
fn ToDoColumn(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap().read();
    cx.render(rsx! {
        div {
            class: COLUMN,
            div {
                class: "flex items-center",
                span {
                    class: "flex w-4 h-4 bg-red-500 rounded-full me-2 flex-shrink-0",
                }
                h2 {
                    class: COLUMN_HEADING,
                    "To Do"
                }
            },
            div {
                class: COLUMN_TASK_LIST,
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
            class: COLUMN,
            div {
                class: "flex items-center",
                span {
                    class: "flex w-4 h-4 bg-yellow-300 rounded-full me-2 flex-shrink-0",
                }
                h2 {
                    class: COLUMN_HEADING,
                    "In Progress"
                }
            },
            div {
                class: COLUMN_TASK_LIST,
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
            class: COLUMN,
            div {
                class: "flex items-center",
                span {
                    class: "flex w-4 h-4 bg-green-500 rounded-full me-2 flex-shrink-0",
                }
                h2 {
                    class: COLUMN_HEADING,
                    "Done"
                }
            },
            div {
                class: COLUMN_TASK_LIST,
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
    let model = use_shared_state::<Model>(cx).unwrap();
    let expanded = use_state(cx, || false);
    let read_model = model.read();
    let data = &read_model.tasks[task_id];
    let users: Vec<_> = data
        .assignees
        .iter()
        .map(|user_id| &read_model.users[user_id])
        .collect();
    cx.render(rsx! {
        div {
            draggable: true,
            prevent_default: "onclick",
            onclick: |_| expanded.set(!**expanded),
            class: "
                flex flex-col gap-2 block w-full p-3 border rounded-lg shadow
                bg-gray-800 border-gray-700 hover:bg-gray-700",
            div {
                class: "grid grid-cols-2",
                h5 {
                    class: "text-xl font-bold tracking-tight text-white underline underline-offset-8",
                    "{data.title}",
                },
                div {
                    class: "grid grid-rows-1 justify-items-end",
                    div {
                        class: "flex flex-row items-center",
                        div {
                            class: "group relative",
                            div {
                                class: "cursor-pointer flex w-4 h-4 bg-red-500 rounded-full me-2 flex-shrink-0",
                                onclick: |event| {
                                    event.stop_propagation();
                                    set_task_status(model.clone(), *task_id, TaskStatus::ToDo)
                                },
                            }
                            div {
                                class: TOOLTIP,
                                "To Do"
                                div {
                                    class: "tooltip-arrow",
                                    "data-popper-arrow": "",
                                }
                            }
                        }
                        div {
                            class: "group relative",
                            div {
                                class: "cursor-pointer flex w-4 h-4 bg-yellow-300 rounded-full me-2 flex-shrink-0",
                                onclick: |event| {
                                    event.stop_propagation();
                                    set_task_status(model.clone(), *task_id, TaskStatus::InProgress)
                                },
                            }
                            div {
                                class: TOOLTIP,
                                "In Progress"
                                div {
                                    class: "tooltip-arrow",
                                    "data-popper-arrow": "",
                                }
                            }
                        }
                        div {
                            class: "group relative",
                            div {
                                class: "cursor-pointer flex w-4 h-4 bg-green-500 rounded-full me-2 flex-shrink-0",
                                onclick: |event| {
                                    event.stop_propagation();
                                    set_task_status(model.clone(), *task_id, TaskStatus::Done)
                                },
                            }
                            div {
                                class: TOOLTIP,
                                "Done"
                                div {
                                    class: "tooltip-arrow",
                                    "data-popper-arrow": "",
                                }
                            }
                        }
                    }
                }
            }
            div{
                match data.size {
                    TaskSize::Small => {rsx!{
                        span {
                            class: "bg-green-100 text-green-800 text-sm font-medium me-2 px-2.5 py-0.5 rounded dark:bg-green-900 dark:text-green-300",
                            "Small",
                        }
                    }}
                    TaskSize::Medium => {rsx!{
                        span {
                            class: "bg-yellow-100 text-yellow-800 text-sm font-medium me-2 px-2.5 py-0.5 rounded dark:bg-yellow-900 dark:text-yellow-300",
                            "Medium",
                        }
                    }}
                    TaskSize::Large => {rsx!{
                        span {
                            class: "bg-red-100 text-red-800 text-sm font-medium me-2 px-2.5 py-0.5 rounded dark:bg-red-900 dark:text-red-300",
                            "Large",
                        }
                    }}
                }
            },
            div {
                class: "flex flex-row gap-2",
                for user in users {rsx!{
                    div {
                        class: "group relative",
                        div {
                            class: "w-6 h-6 rounded cursor-pointer {color_picker::class(&user.color)}",
                        },
                        div {
                            class: TOOLTIP,
                            "{user.name}"
                            div {
                                class: "tooltip-arrow",
                                "data-popper-arrow": "",
                            }
                        }
                    }
                }}
            }
            if **expanded {rsx!{
                div {
                    class: "p-4 bg-gray-900 rounded border border-gray-700",
                    pre {
                        class: "mb-3 text-white",
                        "{data.description}"
                    }
                }
            }}
        }
    })
}

async fn set_task_status(model: UseSharedState<Model>, task_id: TaskId, status: TaskStatus) {
    if send_set_task_status_request(model.clone(), task_id, status)
        .await
        .is_ok()
    {
        requests::board(model.clone()).await;
    }
}

async fn send_set_task_status_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
    status: TaskStatus,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/status",
            model.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .put(url)
        .json(&status)
        .send()
        .await?
        .json::<()>()
        .await?)
}
