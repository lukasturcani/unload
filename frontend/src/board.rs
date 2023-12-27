use std::fmt::Display;

use crate::route::Route;
use chrono::DateTime;
use chrono::Utc;
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
    let now = chrono::offset::Utc::now();
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
                svg {
                    class: "w-6 h-6 text-gray-400",
                    "aria-hidden": "true",
                    "xmlns": "http://www.w3.org/2000/svg",
                    "fill": "none",
                    "viewBox": "0 0 20 20",
                    path {
                        fill: "currentColor",
                        d: "M6 1a1 1 0 0 0-2 0h2ZM4 4a1 1 0 0 0 2 0H4Zm7-3a1 1 0 1 0-2 0h2ZM9 4a1 1 0 1 0 2 0H9Zm7-3a1 1 0 1 0-2 0h2Zm-2 3a1 1 0 1 0 2 0h-2ZM1 6a1 1 0 0 0 0 2V6Zm18 2a1 1 0 1 0 0-2v2ZM5 11v-1H4v1h1Zm0 .01H4v1h1v-1Zm.01 0v1h1v-1h-1Zm0-.01h1v-1h-1v1ZM10 11v-1H9v1h1Zm0 .01H9v1h1v-1Zm.01 0v1h1v-1h-1Zm0-.01h1v-1h-1v1ZM10 15v-1H9v1h1Zm0 .01H9v1h1v-1Zm.01 0v1h1v-1h-1Zm0-.01h1v-1h-1v1ZM15 15v-1h-1v1h1Zm0 .01h-1v1h1v-1Zm.01 0v1h1v-1h-1Zm0-.01h1v-1h-1v1ZM15 11v-1h-1v1h1Zm0 .01h-1v1h1v-1Zm.01 0v1h1v-1h-1Zm0-.01h1v-1h-1v1ZM5 15v-1H4v1h1Zm0 .01H4v1h1v-1Zm.01 0v1h1v-1h-1Zm0-.01h1v-1h-1v1ZM2 4h16V2H2v2Zm16 0h2a2 2 0 0 0-2-2v2Zm0 0v14h2V4h-2Zm0 14v2a2 2 0 0 0 2-2h-2Zm0 0H2v2h16v-2ZM2 18H0a2 2 0 0 0 2 2v-2Zm0 0V4H0v14h2ZM2 4V2a2 2 0 0 0-2 2h2Zm2-3v3h2V1H4Zm5 0v3h2V1H9Zm5 0v3h2V1h-2ZM1 8h18V6H1v2Zm3 3v.01h2V11H4Zm1 1.01h.01v-2H5v2Zm1.01-1V11h-2v.01h2Zm-1-1.01H5v2h.01v-2ZM9 11v.01h2V11H9Zm1 1.01h.01v-2H10v2Zm1.01-1V11h-2v.01h2Zm-1-1.01H10v2h.01v-2ZM9 15v.01h2V15H9Zm1 1.01h.01v-2H10v2Zm1.01-1V15h-2v.01h2Zm-1-1.01H10v2h.01v-2ZM14 15v.01h2V15h-2Zm1 1.01h.01v-2H15v2Zm1.01-1V15h-2v.01h2Zm-1-1.01H15v2h.01v-2ZM14 11v.01h2V11h-2Zm1 1.01h.01v-2H15v2Zm1.01-1V11h-2v.01h2Zm-1-1.01H15v2h.01v-2ZM4 15v.01h2V15H4Zm1 1.01h.01v-2H5v2Zm1.01-1V15h-2v.01h2Zm-1-1.01H5v2h.01v-2Z",
                    }
                }
                if let Some(due) = data.due {rsx!{
                    p {
                        class: "font-normal text-gray-400",
                        "{due} ({time_delta(&now, &due)})"
                    }
                }}
            }
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

struct TimeDelta {
    days: i32,
    hours: i8,
    minutes: i8,
}

impl Display for TimeDelta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d {}h {}m", self.days, self.hours, self.minutes)
    }
}

fn time_delta(start: &DateTime<Utc>, stop: &DateTime<Utc>) -> TimeDelta {
    let duration = stop.naive_utc() - start.naive_utc();
    let days = duration.num_days();
    let hours = duration.num_hours() - duration.num_days() * 24;
    let minutes = duration.num_minutes() - (days * 24 * 60) - (hours * 60);
    TimeDelta {
        days: days as i32,
        hours: hours as i8,
        minutes: minutes as i8,
    }
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
