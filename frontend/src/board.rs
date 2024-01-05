use std::fmt::Display;

use crate::route::Route;
use crate::user_search::UserSearch;
use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone};
use chrono::{Local, Utc};
use dioxus_router::hooks::use_navigator;
use reqwest::Client;
use shared_models::TaskStatus;
use shared_models::{TaskSize, UserId};

use crate::color_picker;
use crate::model::Model;
use crate::requests;
use crate::styles;
use dioxus::prelude::*;
use shared_models::{BoardName, TaskId};

use web_sys;

pub const COLUMN: &str = "
    flex flex-col gap-2 rounded bg-gray-900 border border-gray-700 p-4
    overflow-y-auto
";
pub const COLUMN_HEADING: &str = "text-3xl font-extrabold text-white";
pub const COLUMN_TASK_LIST: &str = "flex flex-col gap-2 overflow-y-scroll";
pub const TOOLTIP: &str = "
    pointer-events-none absolute -top-10 -left-2 w-max
    opacity-0 transition-opacity group-hover:opacity-100
    z-10 px-3 py-2 text-sm font-medium text-white
    rounded-lg shadow-sm tooltip bg-gray-800
    border border-gray-700";

enum ResponsiveLayout {
    Narrow,
    Wide,
}

impl ResponsiveLayout {
    fn from_window() -> Self {
        let width = web_sys::window()
            .unwrap()
            .inner_width()
            .unwrap()
            .as_f64()
            .unwrap();
        if width < 640.0 {
            ResponsiveLayout::Narrow
        } else {
            ResponsiveLayout::Wide
        }
    }
}

#[component]
pub fn Board(cx: Scope, board_name: BoardName) -> Element {
    let layout = ResponsiveLayout::from_window();
    cx.render(rsx! {
        match layout {
            ResponsiveLayout::Narrow => rsx! {
                OneColumnBoard {
                    board_name: board_name.clone(),
                }
            },
            ResponsiveLayout::Wide => rsx! {
                ThreeColumnBoard {
                    board_name: board_name.clone(),
                }
            }
        }
    })
}

#[component]
fn OneColumnBoard(cx: Scope, board_name: BoardName) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    if &model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    let nav = use_navigator(cx);
    let column = use_state(cx, || TaskStatus::ToDo);
    use_future(cx, (), |_| requests::board(model.clone()));
    cx.render(rsx! {
        div {
            class: "bg-gray-900 h-screen w-screen grid grid-cols-1 gap-2",
            div {
                class: "p-2 overflow-y-auto",
                match **column {
                    TaskStatus::ToDo => rsx! { ToDoColumn {} },
                    TaskStatus::InProgress => rsx! { InProgressColumn {} },
                    TaskStatus::Done => rsx! { DoneColumn {} },
                }
            }
            div {
                class: "w-full h-16 self-end border-t bg-gray-700 border-gray-600", 
                div {
                    class: "grid h-full max-w-lg grid-cols-5 mx-auto font-medium", 
                    button {
                        r#type: "button" ,
                        class: "inline-flex flex-col items-center justify-center px-5 border-x enabled:hover:bg-gray-800 group border-gray-600",
                        disabled: **column == TaskStatus::ToDo,
                        onclick: |_| {
                            match **column {
                                TaskStatus::ToDo => column.set(TaskStatus::ToDo),
                                TaskStatus::InProgress => column.set(TaskStatus::ToDo),
                                TaskStatus::Done => column.set(TaskStatus::InProgress),
                            }
                        },
                        if **column != TaskStatus::ToDo {rsx!{
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                "viewBox": "0 0 24 24",
                                "stroke-width": "1.5", 
                                stroke: "currentColor",
                                class: "w-6 h-6 text-gray-400 group-hover:text-blue-500",
                                path {
                                    "stroke-linecap": "round", 
                                    "stroke-linejoin": "round",
                                    d: "M15.75 19.5 8.25 12l7.5-7.5",
                                }
                            }

                        }}
                    }
                    button {
                        r#type: "button" ,
                        class: "inline-flex flex-col items-center justify-center px-5 border-e hover:bg-gray-800 group border-gray-600",
                        onclick: |_| {
                            nav.push(Route::AddUser {
                                board_name: board_name.clone(),
                            });
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none", 
                            "viewBox": "0 0 24 24", 
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "mb-2 w-6 h-6 text-gray-400 group-hover:text-blue-500",
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M18 7.5v3m0 0v3m0-3h3m-3 0h-3m-2.25-4.125a3.375 3.375 0 1 1-6.75 0 3.375 3.375 0 0 1 6.75 0ZM3 19.235v-.11a6.375 6.375 0 0 1 12.75 0v.109A12.318 12.318 0 0 1 9.374 21c-2.331 0-4.512-.645-6.374-1.766Z",
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex flex-col items-center justify-center px-5 border-e hover:bg-gray-800 group border-gray-600",
                        onclick: |_| {
                            nav.push(Route::Users {
                                board_name: board_name.clone(),
                            });
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none", 
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5", 
                            stroke: "currentColor",
                            class: "mb-2 w-6 h-6 text-gray-400 group-hover:text-blue-500",
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M15 19.128a9.38 9.38 0 0 0 2.625.372 9.337 9.337 0 0 0 4.121-.952 4.125 4.125 0 0 0-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 0 1 8.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0 1 11.964-3.07M12 6.375a3.375 3.375 0 1 1-6.75 0 3.375 3.375 0 0 1 6.75 0Zm8.25 2.25a2.625 2.625 0 1 1-5.25 0 2.625 2.625 0 0 1 5.25 0Z",
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex flex-col items-center justify-center px-5 hover:bg-gray-800 group",
                        onclick: |_| {
                            nav.push(Route::AddTask {
                                board_name: board_name.clone(),
                            });
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none", 
                            "viewBox": "0 0 24 24", 
                            "stroke-width": "1.5", 
                            "stroke": "currentColor", 
                            class: "mb-2 w-6 h-6 text-gray-400 group-hover:text-blue-500",
                            path {
                                "stroke-linecap": "round", 
                                "stroke-linejoin": "round", 
                                d: "M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex flex-col items-center justify-center px-5 enabled:hover:bg-gray-800 group border-x border-gray-600",
                        disabled: **column == TaskStatus::Done,
                        onclick: |_| {
                            match **column {
                                TaskStatus::ToDo => column.set(TaskStatus::InProgress),
                                TaskStatus::InProgress => column.set(TaskStatus::Done),
                                TaskStatus::Done => column.set(TaskStatus::Done),
                            }
                        },
                        if **column != TaskStatus::Done {rsx!{
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none", 
                                "viewBox": "0 0 24 24", 
                                "stroke-width": "1.5", 
                                stroke: "currentColor", 
                                class: "w-6 h-6 text-gray-400 group-hover:text-blue-500",
                                path {
                                    "stroke-lineca": "round",
                                    "stroke-linejoin": "round", 
                                    d: "m8.25 4.5 7.5 7.5-7.5 7.5",
                                }
                            }
                        }}
                    }
                }
            }
    }
    })
}

#[component]
fn ThreeColumnBoard(cx: Scope, board_name: BoardName) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let nav = use_navigator(cx);
    if &model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    use_future(cx, (), |_| requests::board(model.clone()));
    cx.render(rsx! {
        div {
            class: "flex flex-col bg-gray-900 h-screen w-screen p-4 gap-2",
            div {
                class: "grow grid grid-cols-3 gap-2 overflow-y-auto",
                ToDoColumn {},
                InProgressColumn {},
                DoneColumn {},
            },
            div {
                class: "flex flex-row justify-center gap-2",
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
                class: "flex items-center gap-2",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    "viewBox": "0 0 24 24",
                    "stroke-width": "1.5",
                    stroke: "white",
                    class: "w-8 h-8",
                    path {
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        d: "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                    }
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
                        status: TaskStatus::ToDo,
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
                class: "flex items-center gap-2",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    "viewBox": "0 0 24 24",
                    "stroke-width": "1.5",
                    "stroke": "white",
                    "class": "w-8 h-8",
                    path {
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        d: "M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                    }
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
                        status: TaskStatus::InProgress,
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
                class: "flex items-center gap-2",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    "viewBox": "0 0 24 24",
                    "stroke-width": "1.5",
                    stroke: "white",
                    class: "w-8 h-8",
                    path {
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        d: "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                    }
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
                        status: TaskStatus::Done,
                    }
                }
            },
        }
    })
}

#[component]
fn Task(cx: Scope, task_id: TaskId, status: TaskStatus) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let expanded = use_state(cx, || false);
    let editing_title = use_state(cx, || false);
    let new_title = use_state(cx, String::new);
    let editing_description = use_state(cx, || false);
    let new_description = use_state(cx, String::new);
    let editing_size = use_state(cx, || false);
    let read_model = model.read();
    let data = &read_model.tasks[task_id];
    let title = data.title.clone();
    let description = data.description.clone();
    let hover_status = use_state(cx, || None::<TaskStatus>);
    cx.render(rsx! {
        div {
            draggable: true,
            prevent_default: "onclick",
            onclick: |_| expanded.set(!**expanded),
            class: "
                flex flex-col gap-2 p-3 border rounded-lg shadow
                bg-gray-800 border-gray-700 hover:bg-gray-700",
            div {
                class: "grid grid-cols-2",
                if **editing_title {rsx!{
                    input {
                        class: "
                            bg-inherit text-xl font-bold tracking-tight
                            text-white underline underline-offset-8 rounded
                        ",
                        r#type: "text",
                        oninput: |event| new_title.set(event.value.clone()),
                        onfocusout: |_| {
                            editing_title.set(false);
                            set_task_title(model.clone(), *task_id, (**new_title).clone())
                        },
                        value: "{new_title}",
                    }
                }} else {rsx!{
                    div {
                        class: "grid grid-rows-1 justify-items-start",
                        h5 {
                            class: "text-xl font-bold tracking-tight text-white underline underline-offset-8",
                            onclick: move |_| {
                                editing_title.set(true);
                                new_title.set(title.clone());
                            },
                            "{data.title}",
                        },
                    }
                }}
                div {
                    class: "grid grid-rows-1 justify-items-end",
                    div {
                        class: "flex flex-row gap-1 items-center",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: match **hover_status {
                                Some(TaskStatus::ToDo) => "red",
                                _ => "white",
                            },
                            class: "cursor-pointer w-8 h-8",
                            onmouseover: |_| hover_status.set(Some(TaskStatus::ToDo)),
                            onmouseout: |_| hover_status.set(None),
                            onclick: |event| {
                                event.stop_propagation();
                                set_task_status(model.clone(), *task_id, TaskStatus::ToDo)
                            },
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                            }
                        }
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: match **hover_status {
                                Some(TaskStatus::InProgress) => "yellow",
                                _ => "white",
                            },
                            "class": "cursor-pointer w-8 h-8",
                            onmouseover: |_| hover_status.set(Some(TaskStatus::InProgress)),
                            onmouseout: |_| hover_status.set(None),
                            onclick: |event| {
                                event.stop_propagation();
                                set_task_status(model.clone(), *task_id, TaskStatus::InProgress)
                            },
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                            }
                        }
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: match **hover_status {
                                Some(TaskStatus::Done) => "green",
                                _ => "white",
                            },
                            class: "cursor-pointer w-8 h-8",
                            onmouseover: |_| hover_status.set(Some(TaskStatus::Done)),
                            onmouseout: |_| hover_status.set(None),
                            onclick: |event| {
                                event.stop_propagation();
                                set_task_status(model.clone(), *task_id, TaskStatus::Done)
                            },
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                            }
                        }
                    }
                }
            }
            if **editing_size {rsx!{
                div {
                    class: "flex flex-row gap-1",
                    span {
                        class: "text-sm font-medium px-2.5 py-0.5 rounded bg-green-900 text-green-300 cursor-pointer",
                        onclick: |_| {
                            editing_size.set(false);
                            set_task_size(model.clone(), *task_id, TaskSize::Small)
                        },
                        "Small",
                    }
                    span {
                        class: "text-sm font-medium px-2.5 py-0.5 rounded bg-yellow-900 text-yellow-300 cursor-pointer",
                        onclick: |_| {
                            editing_size.set(false);
                            set_task_size(model.clone(), *task_id, TaskSize::Medium)
                        },
                        "Medium",
                    }
                    span {
                        class: "text-sm font-medium px-2.5 py-0.5 rounded bg-red-900 text-red-300 cursor-pointer",
                        onclick: |_| {
                            editing_size.set(false);
                            set_task_size(model.clone(), *task_id, TaskSize::Large)
                        },
                        "Large",
                    }
                },
            }} else {rsx!{
                div {
                    match data.size {
                        TaskSize::Small => {rsx!{
                            span {
                                class: "text-sm font-medium px-2.5 py-0.5 rounded bg-green-900 text-green-300 cursor-pointer",
                                onclick: |_| editing_size.set(true),
                                "Small",
                            }
                        }}
                        TaskSize::Medium => {rsx!{
                            span {
                                class: "text-sm font-medium px-2.5 py-0.5 rounded bg-yellow-900 text-yellow-300 cursor-pointer",
                                onclick: |_| editing_size.set(true),
                                "Medium",
                            }
                        }}
                        TaskSize::Large => {rsx!{
                            span {
                                class: "text-sm font-medium px-2.5 py-0.5 rounded bg-red-900 text-red-300 cursor-pointer",
                                onclick: |_| editing_size.set(true),
                                "Large",
                            }
                        }}
                    }
                },
            }}
            Users {
                task_id: *task_id,
            },
            if let Some(due_value) = data.due {rsx!{
                Due {
                    task_id: *task_id,
                    due: DueOptions{
                        due: due_value,
                        show_time_left: match status {
                            TaskStatus::ToDo | TaskStatus::InProgress => true,
                            TaskStatus::Done => false,
                        }
                    }
                }
            }}
            if **expanded && data.due.is_none() {rsx!{
                Due{
                    task_id: *task_id,
                }
            }}
            if **expanded {rsx!{
                if **editing_description {rsx!{
                    textarea {
                        class: "p-4 bg-gray-900 rounded border border-gray-700 text-white",
                        rows: data.description.lines().count() as i64,
                        oninput: |event| new_description.set(event.value.clone()),
                        onfocusout: |_| {
                            editing_description.set(false);
                            set_task_description(model.clone(), *task_id, (**new_description).clone())
                        },
                        value: "{new_description}",
                    }

                }} else {rsx!{
                    div {
                        class: "p-4 bg-gray-900 rounded border border-gray-700",
                        onclick: move |_| {
                            editing_description.set(true);
                            new_description.set(description.clone());
                        },
                        pre {
                            class: "mb-3 text-white",
                            "{data.description}"
                        }
                    }
                }}
            }}
        }
    })
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DueOptions {
    due: DateTime<Utc>,
    show_time_left: bool,
}

#[component]
fn Due(cx: Scope, task_id: TaskId, due: Option<DueOptions>) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let editing = use_state(cx, || false);
    let new_date = use_state(cx, || None::<NaiveDate>);
    let new_time = use_state(cx, || NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    let now = Utc::now();
    cx.render(rsx! {
        if **editing {rsx!{
            div {
                class: "flex flex-row gap-2 items-center",
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
                div {
                    class: "grid grid-cols-2 gap-2 place-items-center",
                    if let Some(new_date_value) = **new_date {rsx!{
                        input {
                            class: "bg-inherit border text-sm rounded-lg block w-full p-2.5 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500",
                            r#type: "date",
                            value: "{new_date_value.format(\"%Y-%m-%d\")}",
                            oninput: |event| {
                                if event.value.is_empty() {
                                    new_date.set(None);
                                    new_time.set(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                                } else if let Ok(date) = NaiveDate::parse_from_str(&event.value, "%Y-%m-%d") {
                                    new_date.set(Some(date))
                                }
                            },
                        },
                        select {
                            class: "bg-inherit border text-sm rounded-lg block w-full p-2.5 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500",
                            value: "{format_due_time(&**new_time)}",
                            onchange: |event| {
                                if let Ok(time) = NaiveTime::parse_from_str(&event.value, "%H:%M") {
                                    new_time.set(time);
                                }
                            },
                            option {
                                value: "{format_due_time(&**new_time)}",
                                "{format_due_time(&**new_time)}"
                            },
                            for hour in 0..24 {
                                for minute in [0, 15, 30, 45] {
                                    rsx!{
                                        option {
                                            value: "{hour:02}:{minute:02}",
                                            "{hour:02}:{minute:02}"
                                        },
                                    }
                                }
                            }
                        },
                    }} else {rsx!{
                        input {
                            class: "bg-inherit border text-sm rounded-lg block w-full p-2.5 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500",
                            r#type: "date",
                            oninput: |event| {
                                if event.value.is_empty() {
                                    new_date.set(None)
                                } else if let Ok(date) = NaiveDate::parse_from_str(&event.value, "%Y-%m-%d") {
                                    new_date.set(Some(date))
                                }
                            },
                        },
                    }}
                }
                button {
                    r#type: "button",
                    class: "
                        rounded-lg p-1.5 inline-flex items-center justify-center h-8 w-8
                        border border-green-500 text-green-500 hover:bg-green-500 hover:text-white",
                    prevent_default: "onclick",
                    onclick: |event| {
                        event.stop_propagation();
                        editing.set(false);
                        set_task_due(
                            model.clone(),
                            *task_id,
                            new_date.map(|date| {
                                Local.from_local_datetime(&date.and_time(**new_time))
                                .unwrap()
                                .into()
                            })
                        )
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "w-6 h-6",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "m4.5 12.75 6 6 9-13.5",
                        }
                    }
                }
                button {
                    r#type: "button",
                    class: "
                        rounded-lg p-1.5 inline-flex items-center justify-center h-8 w-8
                        border border-red-500 text-red-500 hover:bg-red-500 hover:text-white",
                    prevent_default: "onclick",
                    onclick: |event| {
                        event.stop_propagation();
                        editing.set(false);
                    },
                    svg {
                        class: "stroke-red-500",
                        stroke: "currentColor",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        class: "w-6 h-6",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M6 18 18 6M6 6l12 12",
                        }
                    }
                }
            }
        }} else {rsx!{
            if let Some(DueOptions{due: due_value, show_time_left}) = due {rsx!{
                div {
                    class: "flex flex-row gap-2",
                    onclick: move |_| {
                        editing.set(true);
                        let local = utc_to_local(due_value);
                        new_date.set(Some(local.date_naive()));
                        new_time.set(local.time());
                    },
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
                    p {
                        class: "font-normal text-gray-400",
                        if *show_time_left {rsx!{
                            "{format_datetime(utc_to_local(&due_value))} ({time_delta(&now, &due_value)})"
                        }} else {rsx!{
                            "{format_datetime(utc_to_local(&due_value))}"
                        }}
                    }
                }
            }} else {rsx!{
                div {
                    class: "flex flex-row gap-2",
                    onclick: move |_| {
                        editing.set(true);
                        new_date.set(None);
                        new_time.set(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                    },
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
                }}
            }
        }}
    })
}

#[component]
fn Users(cx: Scope, task_id: TaskId) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let read_model = model.read();
    let data = &read_model.tasks[task_id];
    let users: Vec<_> = data
        .assignees
        .iter()
        .map(|user_id| &read_model.users[user_id])
        .collect();
    let show_assign_user = use_state(cx, || false);
    let assignees = use_ref(cx, Vec::new);
    cx.render(rsx! {
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
            div {
                class: "group relative",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    "viewBox": "0 0 24 24" ,
                    "stroke-width": "1.5" ,
                    stroke: "white" ,
                    class: "w-6 h-6 border border-white rounded cursor-pointer",
                    prevent_default: "onclick",
                    onclick: |event| {
                        *assignees.write() = model.read().tasks[task_id].assignees.clone();
                        show_assign_user.set(true);
                        event.stop_propagation()
                    },
                    path {
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        d: "M12 4.5v15m7.5-7.5h-15",
                    }
                }
                if **show_assign_user {rsx!{
                    div {
                        class: "
                            flex flex-col gap-2
                            absolute -top-10 -left-2 w-72
                            z-10 px-3 py-2 text-sm font-medium text-white
                            rounded-lg shadow-sm bg-gray-800
                            border border-gray-700",
                        UserSearch {
                            id: "assign_user_modal",
                            on_select_user: |user_id| assignees.write().push(user_id),
                            on_remove_user: |user_id| assignees.write().retain(|&value| value != user_id),
                            initial_users: assignees.read().clone(),
                            always_show_suggestions: true,
                        }
                        div {
                            class: "flex flex-row gap-2 justify-end",
                            button {
                                r#type: "button",
                                class: "
                                    rounded-lg p-1.5 inline-flex items-center justify-center h-8 w-8
                                    border border-green-500 text-green-500 hover:bg-green-500 hover:text-white",
                                prevent_default: "onclick",
                                onclick: |event| {
                                    event.stop_propagation();
                                    show_assign_user.set(false);
                                    set_task_assignees(
                                        model.clone(),
                                        *task_id, assignees.read().clone()
                                    )
                                },
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    "viewBox": "0 0 24 24",
                                    "stroke-width": "1.5",
                                    stroke: "currentColor",
                                    class: "w-6 h-6",
                                    path {
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        d: "m4.5 12.75 6 6 9-13.5",
                                    }
                                }
                            }
                            button {
                                r#type: "button",
                                prevent_default: "onclick",
                                class: "
                                    rounded-lg p-1.5 inline-flex items-center justify-center h-8 w-8
                                    border border-red-500 text-red-500 hover:bg-red-500 hover:text-white",
                                onclick: |event| {
                                    event.stop_propagation();
                                    show_assign_user.set(false);
                                },
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    "viewBox": "0 0 24 24",
                                    "stroke-width": "1.5",
                                    stroke: "currentColor",
                                    class: "w-6 h-6",
                                    path {
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        d: "M6 18 18 6M6 6l12 12",
                                    }
                                }
                            }
                        }
                    }
                }} else {rsx!{
                    div {
                        class: TOOLTIP,
                        "Assign User"
                        div {
                            class: "tooltip-arrow",
                            "data-popper-arrow": "",
                        }
                    }
                }}
            }
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

fn utc_to_local(time: &DateTime<Utc>) -> DateTime<Local> {
    chrono::DateTime::<chrono::offset::Local>::from_naive_utc_and_offset(
        time.naive_utc(),
        *chrono::offset::Local::now().offset(),
    )
}

fn format_datetime(time: DateTime<Local>) -> String {
    format!("{}", time.format("%Y-%m-%d %I:%M %p"))
}

fn format_due_time(time: &NaiveTime) -> String {
    format!("{}", time.format("%H:%M"))
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

async fn set_task_title(model: UseSharedState<Model>, task_id: TaskId, title: String) {
    if title.is_empty() {
        return;
    }
    if send_set_task_title_request(model.clone(), task_id, title)
        .await
        .is_ok()
    {
        requests::board(model.clone()).await;
    }
}

async fn send_set_task_title_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
    title: String,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/title",
            model.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .put(url)
        .json(&title)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn set_task_description(model: UseSharedState<Model>, task_id: TaskId, description: String) {
    if send_set_task_description_request(model.clone(), task_id, description)
        .await
        .is_ok()
    {
        requests::board(model.clone()).await;
    }
}

async fn send_set_task_description_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
    description: String,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/description",
            model.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .put(url)
        .json(&description)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn set_task_size(model: UseSharedState<Model>, task_id: TaskId, size: TaskSize) {
    if send_set_task_size_request(model.clone(), task_id, size)
        .await
        .is_ok()
    {
        requests::board(model.clone()).await;
    }
}

async fn send_set_task_size_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
    size: TaskSize,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/size",
            model.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .put(url)
        .json(&size)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn set_task_due(model: UseSharedState<Model>, task_id: TaskId, due: Option<DateTime<Utc>>) {
    if send_set_task_due_request(model.clone(), task_id, due)
        .await
        .is_ok()
    {
        requests::board(model.clone()).await;
    }
}

async fn send_set_task_due_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
    due: Option<DateTime<Utc>>,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/due",
            model.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .put(url)
        .json(&due)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn set_task_assignees(model: UseSharedState<Model>, task_id: TaskId, assignees: Vec<UserId>) {
    if send_set_task_assignees_request(model.clone(), task_id, assignees)
        .await
        .is_ok()
    {
        requests::board(model.clone()).await;
    }
}

async fn send_set_task_assignees_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
    assignees: Vec<UserId>,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/assignees",
            model.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .put(url)
        .json(&assignees)
        .send()
        .await?
        .json::<()>()
        .await?)
}
