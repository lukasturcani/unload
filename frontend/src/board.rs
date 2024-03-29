use std::fmt::Display;

use crate::filter_bar::{FilterBar, SizeFilter, TagFilter, UserFilter};
use crate::responsive_layout::ResponsiveLayout;
use crate::route::Route;
use crate::tag_search::TagSearch;
use crate::user_search::UserSearch;
use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone};
use chrono::{Local, Utc};
use dioxus_router::hooks::use_navigator;
use itertools::Itertools;
use reqwest::Client;
use shared_models::{Color, QuickAddData, QuickAddTaskId, TagId, TaskData, TaskStatus};
use shared_models::{TaskSize, UserId};

use crate::model::Model;
use crate::requests;
use crate::{color_picker, styles};
use dioxus::prelude::*;
use shared_models::{BoardName, TaskId};

pub const COLUMN: &str = "
    grow flex flex-col gap-2 rounded bg-gray-900 pt-2 px-2 sm:pt-4 sm:px-4 overflow-y-auto
";
pub const COLUMN_HEADING: &str = "text-xl sm:text-3xl font-extrabold text-white";
pub const COLUMN_TASK_LIST: &str = "grow flex flex-col gap-2 overflow-y-scroll";
pub const DENSE_COLUMN_TASK_LIST: &str = "grow flex flex-col overflow-y-scroll";

#[component]
pub fn Board(cx: Scope, board_name: BoardName) -> Element {
    let layout = ResponsiveLayout::from_window();
    let eval = use_eval(cx);
    eval(&format!(r#"document.title = "{board_name}";"#)).unwrap();
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
    let show_filters = use_state(cx, || false);
    use_future(cx, (), |_| requests::board(model.clone()));
    cx.render(rsx! {
        div {
            class: "flex flex-col bg-gray-900 h-dvh w-screen gap-1",
            div {
                class: "grow grid grid-cols-1 p-1 overflow-y-auto",
                match (**column, model.read().dense_view) {
                    (TaskStatus::ToDo, false) => rsx! { ToDoColumn {} },
                    (TaskStatus::InProgress, false) => rsx! { InProgressColumn {} },
                    (TaskStatus::Done, false) => rsx! { DoneColumn {} },
                    (TaskStatus::ToDo, true) => rsx! { DenseToDoColumn {} },
                    (TaskStatus::InProgress, true) => rsx! { DenseInProgressColumn {} },
                    (TaskStatus::Done, true) => rsx! { DenseDoneColumn {} },
                }
            }
            div {
                class: "flex flex-col",
                if **show_filters {rsx!{
                   div {
                       class: "w-full bg-gray-800 flex flex-col gap-2 p-2",
                        TagFilter {}
                        UserFilter {}
                        SizeFilter {}
                   }
                }}
                div {
                    class: styles::BOTTOM_BAR,
                    button {
                        r#type: "button",
                        class: styles::BOTTOM_BAR_BUTTON,
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
                                class: "w-6 h-6 text-gray-400 group-active:text-blue-500",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    d: "M15.75 19.5 8.25 12l7.5-7.5",
                                }
                            }
                        }}
                    }
                    button {
                        r#type: "button",
                        class: styles::BOTTOM_BAR_BUTTON,
                        onclick: |_| {
                            nav.push(Route::TaskArchive {
                                board_name: board_name.clone(),
                            });
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "w-6 h-6 text-gray-400 group-active:text-blue-500",
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z",
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: if **show_filters {
                            styles::BOTTOM_BAR_ACTIVE_BUTTON
                        } else {
                            styles::BOTTOM_BAR_BUTTON
                        },
                        onclick: |_| show_filters.set(!**show_filters),
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: if **show_filters {
                                "w-6 h-6 text-blue-500"
                            } else {
                                "w-6 h-6 text-gray-400 group-active:text-blue-500"
                            },
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M12 3c2.755 0 5.455.232 8.083.678.533.09.917.556.917 1.096v1.044a2.25 2.25 0 0 1-.659 1.591l-5.432 5.432a2.25 2.25 0 0 0-.659 1.591v2.927a2.25 2.25 0 0 1-1.244 2.013L9.75 21v-6.568a2.25 2.25 0 0 0-.659-1.591L3.659 7.409A2.25 2.25 0 0 1 3 5.818V4.774c0-.54.384-1.006.917-1.096A48.32 48.32 0 0 1 12 3Z",
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: styles::BOTTOM_BAR_BUTTON,
                        onclick: |_| {
                            nav.push(Route::Tags {
                                board_name: board_name.clone(),
                            });
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "w-6 h-6 text-gray-400 group-active:text-blue-500",
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M9.568 3H5.25A2.25 2.25 0 0 0 3 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 0 0 5.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 0 0 9.568 3Z",
                            }
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M6 6h.008v.008H6V6Z",
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: styles::BOTTOM_BAR_BUTTON,
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
                            class: "w-6 h-6 text-gray-400 group-active:text-blue-500",
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M15 19.128a9.38 9.38 0 0 0 2.625.372 9.337 9.337 0 0 0 4.121-.952 4.125 4.125 0 0 0-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 0 1 8.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0 1 11.964-3.07M12 6.375a3.375 3.375 0 1 1-6.75 0 3.375 3.375 0 0 1 6.75 0Zm8.25 2.25a2.625 2.625 0 1 1-5.25 0 2.625 2.625 0 0 1 5.25 0Z",
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: styles::BOTTOM_BAR_BUTTON,
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
                                class: "w-6 h-6 text-gray-400 group-active:text-blue-500",
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
    if &model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    use_future(cx, (), |_| requests::board(model.clone()));
    cx.render(rsx! {
        div {
            class: "flex flex-col bg-gray-900 h-dvh w-screen",
            div {
                class: "grow flex flex-col gap-2 overflow-y-auto p-4 pb-2",
                div {
                    class: "grow w-full h-full overflow-y-auto",
                    div {
                        class: "w-full h-full grid grid-cols-3 gap-2 overflow-y-auto",
                        if model.read().dense_view {rsx!{
                            DenseToDoColumn {}
                            DenseInProgressColumn {}
                            DenseDoneColumn {}
                        }} else {rsx!{
                            ToDoColumn {}
                            InProgressColumn {}
                            DoneColumn {}
                        }}
                    },
                }
                FilterBar {}
            }
            BottomBar {
                board_name: board_name.clone(),
            }
        }
    })
}

#[component]
fn BottomBar(cx: Scope, board_name: BoardName) -> Element {
    let nav = use_navigator(cx);
    cx.render(rsx! {
        div {
            class: styles::BOTTOM_BAR,
            button {
                r#type: "button",
                class: styles::BOTTOM_BAR_BUTTON,
                onclick: |_| {
                    nav.push(Route::TaskArchive {
                        board_name: board_name.clone(),
                    });
                },
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
                        d: "m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z",
                    }
                }
            }
            button {
                r#type: "button",
                class: styles::BOTTOM_BAR_BUTTON,
                onclick: |_| {
                    nav.push(Route::Tags {
                        board_name: board_name.clone(),
                    });
                },
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
                        d: "M9.568 3H5.25A2.25 2.25 0 0 0 3 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 0 0 5.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 0 0 9.568 3Z",
                    }
                    path {
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        d: "M6 6h.008v.008H6V6Z",
                    }
                }
            }
            button {
                r#type: "button",
                class: styles::BOTTOM_BAR_BUTTON,
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
                    class: "w-6 h-6 text-gray-400 group-hover:text-blue-500",
                    path {
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        d: "M15 19.128a9.38 9.38 0 0 0 2.625.372 9.337 9.337 0 0 0 4.121-.952 4.125 4.125 0 0 0-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 0 1 8.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0 1 11.964-3.07M12 6.375a3.375 3.375 0 1 1-6.75 0 3.375 3.375 0 0 1 6.75 0Zm8.25 2.25a2.625 2.625 0 1 1-5.25 0 2.625 2.625 0 0 1 5.25 0Z",
                    }
                }
            }
         }
    })
}

#[component]
fn ToDoColumn(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let read_model = model.read();
    let nav = use_navigator(cx);
    let show_quick_add = use_state(cx, || false);
    cx.render(rsx! {
        div {
            class: "flex flex-col overflow-y-auto border border-gray-700",
            div {
                class: COLUMN,
                div {
                    class: "flex flex-row justify-between",
                    div {
                        class: "flex items-center gap-2",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "white",
                            class: "w-6 h-6 sm:w-8 sm:h-8",
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
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "w-6 h-6 sm:w-8 sm:h-8 text-white cursor-pointer",
                        onclick: |event| {
                            event.stop_propagation();
                            model.write().dense_view = true;
                        },
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M6.429 9.75 2.25 12l4.179 2.25m0-4.5 5.571 3 5.571-3m-11.142 0L2.25 7.5 12 2.25l9.75 5.25-4.179 2.25m0 0L21.75 12l-4.179 2.25m0 0 4.179 2.25L12 21.75 2.25 16.5l4.179-2.25m11.142 0-5.571 3-5.571-3",
                        }
                    }
                },
                div {
                    class: COLUMN_TASK_LIST,
                    for task_id in
                        read_model
                        .to_do
                        .iter()
                        .filter(|task_id| read_model.show_task(**task_id))
                    {
                        Task {
                            key: "{task_id}",
                            task_id: *task_id,
                            status: TaskStatus::ToDo,
                        }
                    }
                },
            }
            if **show_quick_add {rsx!{
                QuickAddTasks {
                    status: TaskStatus::ToDo,
                }
            }}
            div {
                class: "grid grid-cols-2 divide-x border-gray-700",
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: |_| {
                        nav.push(Route::AddToDoTask {
                            board_name: model.read().board_name.clone(),
                        });
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        "stroke": "currentColor",
                        class: "
                            w-6 h-6 text-white
                            group-active:text-blue-500 sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                        }
                    }
                }
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: |_| show_quick_add.set(!**show_quick_add),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        "stroke": "currentColor",
                        class: "
                            w-6 h-6 text-white
                            group-active:text-blue-500 sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z",
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn DenseToDoColumn(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let read_model = model.read();
    let nav = use_navigator(cx);
    let show_quick_add = use_state(cx, || false);
    cx.render(rsx! {
        div {
            class: "flex flex-col overflow-y-auto border border-gray-700",
            div {
                class: COLUMN,
                div {
                    class: "flex flex-row justify-between",
                    div {
                        class: "flex items-center gap-2",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "white",
                            class: "w-6 h-6 sm:w-8 sm:h-8",
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
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "w-6 h-6 sm:w-8 sm:h-8 text-blue-500 cursor-pointer",
                        onclick: |event| {
                            event.stop_propagation();
                            model.write().dense_view = false;
                        },
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M6.429 9.75 2.25 12l4.179 2.25m0-4.5 5.571 3 5.571-3m-11.142 0L2.25 7.5 12 2.25l9.75 5.25-4.179 2.25m0 0L21.75 12l-4.179 2.25m0 0 4.179 2.25L12 21.75 2.25 16.5l4.179-2.25m11.142 0-5.571 3-5.571-3",
                        }
                    }
                },
                div {
                    class: DENSE_COLUMN_TASK_LIST,
                    for task_id in
                        read_model
                        .to_do
                        .iter()
                        .filter(|task_id| read_model.show_task(**task_id))
                    {
                        DenseTask {
                            key: "{task_id}",
                            task_id: *task_id,
                            status: TaskStatus::ToDo,
                        }
                    }
                },
            }
            if **show_quick_add {rsx!{
                QuickAddTasks {
                    status: TaskStatus::ToDo,
                }
            }}
            div {
                class: "grid grid-cols-2 divide-x border-gray-700",
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: |_| {
                        nav.push(Route::AddToDoTask {
                            board_name: model.read().board_name.clone(),
                        });
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        "stroke": "currentColor",
                        class: "
                            w-6 h-6 text-white
                            group-active:text-blue-500 sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                        }
                    }
                }
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: |_| show_quick_add.set(!**show_quick_add),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        "stroke": "currentColor",
                        class: "
                            w-6 h-6 text-white
                            group-active:text-blue-500 sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z",
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn InProgressColumn(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let read_model = model.read();
    let nav = use_navigator(cx);
    let show_quick_add = use_state(cx, || false);
    cx.render(rsx! {
        div {
            class: "flex flex-col overflow-y-auto border border-gray-700",
            div {
                class: COLUMN,
                div {
                    class: "flex flex-row justify-between",
                    div {
                        class: "flex items-center gap-2",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            "stroke": "white",
                            "class": "w-6 h-6 sm:w-8 sm:h-8",
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
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "w-6 h-6 sm:w-8 sm:h-8 text-white cursor-pointer",
                        onclick: |event| {
                            event.stop_propagation();
                            model.write().dense_view = true;
                        },
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M6.429 9.75 2.25 12l4.179 2.25m0-4.5 5.571 3 5.571-3m-11.142 0L2.25 7.5 12 2.25l9.75 5.25-4.179 2.25m0 0L21.75 12l-4.179 2.25m0 0 4.179 2.25L12 21.75 2.25 16.5l4.179-2.25m11.142 0-5.571 3-5.571-3",
                        }
                    }
                },
                div {
                    class: COLUMN_TASK_LIST,
                    for task_id in
                        read_model
                        .in_progress
                        .iter()
                        .filter(|task_id| read_model.show_task(**task_id))
                    {
                        Task {
                            key: "{task_id}",
                            task_id: *task_id,
                            status: TaskStatus::InProgress,
                        }
                    }
                },
            }
            if **show_quick_add {rsx!{
                QuickAddTasks {
                    status: TaskStatus::InProgress,
                }
            }}
            div {
                class: "grid grid-cols-2 divide-x border-gray-700",
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: |_| {
                        nav.push(Route::AddInProgressTask {
                            board_name: model.read().board_name.clone(),
                        });
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        "stroke": "currentColor",
                        class: "
                            w-6 h-6 text-white
                            group-active:text-blue-500 sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                        }
                    }
                }
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: |_| show_quick_add.set(!**show_quick_add),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        "stroke": "currentColor",
                        class: "
                            w-6 h-6 text-white
                            group-active:text-blue-500 sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z",
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn DenseInProgressColumn(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let read_model = model.read();
    let nav = use_navigator(cx);
    let show_quick_add = use_state(cx, || false);
    cx.render(rsx! {
        div {
            class: "flex flex-col overflow-y-auto border border-gray-700",
            div {
                class: COLUMN,
                div {
                    class: "flex flex-row justify-between",
                    div {
                        class: "flex items-center gap-2",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            "stroke": "white",
                            class: "w-6 h-6 sm:w-8 sm:h-8",
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
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "w-6 h-6 sm:w-8 sm:h-8 text-blue-500 cursor-pointer",
                        onclick: |event| {
                            event.stop_propagation();
                            model.write().dense_view = false;
                        },
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M6.429 9.75 2.25 12l4.179 2.25m0-4.5 5.571 3 5.571-3m-11.142 0L2.25 7.5 12 2.25l9.75 5.25-4.179 2.25m0 0L21.75 12l-4.179 2.25m0 0 4.179 2.25L12 21.75 2.25 16.5l4.179-2.25m11.142 0-5.571 3-5.571-3",
                        }
                    }
                },
                div {
                    class: DENSE_COLUMN_TASK_LIST,
                    for task_id in
                        read_model
                        .in_progress
                        .iter()
                        .filter(|task_id| read_model.show_task(**task_id))
                    {
                        DenseTask {
                            key: "{task_id}",
                            task_id: *task_id,
                            status: TaskStatus::InProgress,
                        }
                    }
                },
            }
            if **show_quick_add {rsx!{
                QuickAddTasks {
                    status: TaskStatus::InProgress,
                }
            }}
            div {
                class: "grid grid-cols-2 divide-x border-gray-700",
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: |_| {
                        nav.push(Route::AddInProgressTask {
                            board_name: model.read().board_name.clone(),
                        });
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        "stroke": "currentColor",
                        class: "
                            w-6 h-6 text-white
                            group-active:text-blue-500 sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                        }
                    }
                }
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: |_| show_quick_add.set(!**show_quick_add),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        "stroke": "currentColor",
                        class: "
                            w-6 h-6 text-white
                            group-active:text-blue-500 sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z",
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn DoneColumn(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let read_model = model.read();
    let nav = use_navigator(cx);
    let show_quick_add = use_state(cx, || false);
    cx.render(rsx! {
        div {
            class: "flex flex-col overflow-y-auto border border-gray-700",
            div {
                class: COLUMN,
                div {
                    class: "flex flex-row justify-between",
                    div {
                        class: "flex items-center gap-2",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "white",
                            class: "w-6 h-6 sm:w-8 sm:h-8",
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
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "w-6 h-6 sm:w-8 sm:h-8 text-white cursor-pointer",
                        onclick: |event| {
                            event.stop_propagation();
                            model.write().dense_view = true;
                        },
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M6.429 9.75 2.25 12l4.179 2.25m0-4.5 5.571 3 5.571-3m-11.142 0L2.25 7.5 12 2.25l9.75 5.25-4.179 2.25m0 0L21.75 12l-4.179 2.25m0 0 4.179 2.25L12 21.75 2.25 16.5l4.179-2.25m11.142 0-5.571 3-5.571-3",
                        }
                    }
                },
                div {
                    class: COLUMN_TASK_LIST,
                    for task_id in
                        read_model
                        .done
                        .iter()
                        .filter(|task_id| read_model.show_task(**task_id))
                    {
                        Task {
                            key: "{task_id}",
                            task_id: *task_id,
                            status: TaskStatus::Done,
                        }
                    }
                },
            }
            if **show_quick_add {rsx!{
                QuickAddTasks {
                    status: TaskStatus::Done,
                }
            }}
            div {
                class: "grid grid-cols-2 divide-x border-gray-700",
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: |_| {
                        nav.push(Route::AddDoneTask {
                            board_name: model.read().board_name.clone(),
                        });
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        "stroke": "currentColor",
                        class: "
                            w-6 h-6 text-white
                            group-active:text-blue-500 sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                        }
                    }
                }
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: |_| show_quick_add.set(!**show_quick_add),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        "stroke": "currentColor",
                        class: "
                            w-6 h-6 text-white
                            group-active:text-blue-500 sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z",
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn DenseDoneColumn(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let read_model = model.read();
    let nav = use_navigator(cx);
    let show_quick_add = use_state(cx, || false);
    cx.render(rsx! {
        div {
            class: "flex flex-col overflow-y-auto border border-gray-700",
            div {
                class: COLUMN,
                div {
                    class: "flex flex-row justify-between",
                    div {
                        class: "flex items-center gap-2",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "white",
                            class: "w-6 h-6 sm:w-8 sm:h-8",
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
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "w-6 h-6 sm:w-8 sm:h-8 text-blue-500 cursor-pointer",
                        onclick: |event| {
                            event.stop_propagation();
                            model.write().dense_view = false;
                        },
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M6.429 9.75 2.25 12l4.179 2.25m0-4.5 5.571 3 5.571-3m-11.142 0L2.25 7.5 12 2.25l9.75 5.25-4.179 2.25m0 0L21.75 12l-4.179 2.25m0 0 4.179 2.25L12 21.75 2.25 16.5l4.179-2.25m11.142 0-5.571 3-5.571-3",
                        }
                    }
                },
                div {
                    class: DENSE_COLUMN_TASK_LIST,
                    for task_id in
                        read_model
                        .done
                        .iter()
                        .filter(|task_id| read_model.show_task(**task_id))
                    {
                        DenseTask {
                            key: "{task_id}",
                            task_id: *task_id,
                            status: TaskStatus::Done,
                        }
                    }
                },
            }
            if **show_quick_add {rsx!{
                QuickAddTasks {
                    status: TaskStatus::Done,
                }
            }}
            div {
                class: "grid grid-cols-2 divide-x border-gray-700",
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: |_| {
                        nav.push(Route::AddDoneTask {
                            board_name: model.read().board_name.clone(),
                        });
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        "stroke": "currentColor",
                        class: "
                            w-6 h-6 text-white
                            group-active:text-blue-500 sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                        }
                    }
                }
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: |_| show_quick_add.set(!**show_quick_add),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        "stroke": "currentColor",
                        class: "
                            w-6 h-6 text-white
                            group-active:text-blue-500 sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z",
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn QuickAddTasks(cx: Scope, status: TaskStatus) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let read_model = model.read();
    cx.render(rsx! {
        ul {
            class: "
                text-sm text-gray-200
                border-t border-gray-700 divide-y divide-gray-700
                shrink-0 h-1/4 overflow-y-scroll
            ",
            read_model.quick_add.keys().sorted().map(|task_id| {
                rsx!(QuickAddTask {
                    key: "{task_id}",
                    task_id: *task_id,
                    status: *status,
                })
            })
        }
    })
}

#[component]
fn QuickAddTask(cx: Scope, task_id: QuickAddTaskId, status: TaskStatus) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let read_model = model.read();
    let data = &read_model.quick_add[task_id];
    let users: Vec<_> = data
        .assignees
        .iter()
        .map(|user_id| (user_id, &read_model.users[user_id]))
        .collect();
    cx.render(rsx! {
        li {
            key: "{task_id}",
            button {
                r#type: "button",
                class: "
                    text-left w-full px-4 py-2
                    active:bg-gray-800 active:text-white
                    sm:hover:bg-gray-800 sm:hover:text-white
                ",
                prevent_default: "onmousedown",
                onmousedown: |_| {},
                onclick: move |event| {
                    let read_model = model.read();
                    let data = &read_model.quick_add[task_id];
                    event.stop_propagation();
                    create_task(model.clone(), TaskData {
                        title: data.title.clone(),
                        description: data.description.clone(),
                        size: data.size,
                        assignees: data.assignees.clone(),
                        status: *status,
                        tags: data.tags.clone(),
                        due: None,
                    })
                },
                div {
                    class: "flex flex-row justify-between",
                    "{data.title}"
                    div {
                        class: "flex flex-row gap-1",
                        for (user_id, user) in users {rsx!{
                            div {
                                class: "group relative",
                                onclick: |event| event.stop_propagation(),
                                div {
                                    class: "
                                        w-5 h-5 rounded cursor-pointer
                                        border-2 {color_picker::border_class(&user.color)}
                                        {user_bg(&model, user_id, &user.color)}
                                        {color_picker::bg_hover_class(&user.color)}
                                    ",
                                    onclick: {
                                        let user_id = *user_id;
                                        move |event| {
                                            event.stop_propagation();
                                            let mut model = model.write();
                                            if model.user_filter.contains(&user_id) {
                                                model.user_filter.remove(&user_id);
                                            } else {
                                                model.user_filter.insert(user_id);
                                            }
                                        }
                                    },
                                },
                                div {
                                    dir: "rtl",
                                    div {
                                        class: "
                                            pointer-events-none absolute start-0 w-max
                                            opacity-0 transition-opacity group-hover:opacity-100
                                            z-10 px-3 py-2 text-sm font-medium text-white
                                            rounded-lg shadow-sm tooltip bg-gray-800
                                            border border-gray-700
                                        ",
                                        "{user.name}"
                                    }
                                }
                            }
                        }}
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "w-6 h-6 cursor-pointer text-red-600",
                            onclick: move |event| {
                                event.stop_propagation();
                                delete_quick_add_task(model.clone(), *task_id)
                            },
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0",
                            }
                        }
                    }
                }
            }
        }

    })
}

fn size_bg(model: &UseSharedState<Model>, size: &TaskSize) -> &'static str {
    if model
        .read()
        .size_filter
        .map_or(false, |filter| &filter == size)
    {
        match size {
            TaskSize::Small => "bg-emerald-700 ring ring-blue-500",
            TaskSize::Medium => "bg-yellow-900 ring ring-blue-500",
            TaskSize::Large => "bg-red-900 ring ring-blue-500",
        }
    } else {
        "bg-inherit"
    }
}

#[component]
fn DenseTask(cx: Scope, task_id: TaskId, status: TaskStatus) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let read_model = model.read();
    let data = &read_model.tasks[task_id];
    let users: Vec<_> = data
        .assignees
        .iter()
        .map(|user_id| (user_id, &read_model.users[user_id]))
        .collect();
    let expanded = use_state(cx, || false);
    let editing_title = use_state(cx, || false);
    let new_title = use_state(cx, String::new);
    let editing_description = use_state(cx, || false);
    let new_description = use_state(cx, String::new);
    let editing_size = use_state(cx, || false);
    let show_assign_user = use_state(cx, || false);
    let show_assign_tag = use_state(cx, || false);
    let assignees = use_ref(cx, Vec::new);
    cx.render(rsx! {
        div {
            class: "
                first:border-t border-b border-gray-700 p-1
                flex flex-col gap-1
            ",
            onclick: |event| {
                event.stop_propagation();
                expanded.set(!**expanded);
            },
            div {
                class: "flex justify-between",
                div {
                    class: "flex flex-row gap-1 items-center",
                    if **editing_title {rsx!{
                        input {
                            class: styles::TEXT_INPUT,
                            r#type: "text",
                            oninput: |event| new_title.set(event.value.clone()),
                            value: "{new_title}",
                        }
                        button {
                            r#type: "button",
                            class: "
                                rounded-lg p-1.5 inline-flex items-center justify-center h-8 w-8
                                border border-green-500 text-green-500
                                active:bg-green-500 active:text-white
                                sm:hover:bg-green-500 sm:hover:text-white
                            ",
                            prevent_default: "onclick",
                            onclick: |event| {
                                event.stop_propagation();
                                editing_title.set(false);
                                set_task_title(model.clone(), *task_id, (**new_title).clone())
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
                                border border-red-500 text-red-500
                                active:bg-red-500 active:text-white
                                sm:hover:bg-red-500 sm:hover:text-white
                            ",
                            prevent_default: "onclick",
                            onclick: |event| {
                                event.stop_propagation();
                                editing_title.set(false);
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
                    }} else {rsx!{
                        p {
                            class: "text-sm tracking-tight text-white",
                            "{data.title}"
                        }
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "w-4 h-4 text-white cursor-pointer",
                            onclick: move |event| {
                                event.stop_propagation();
                                new_title.set(model.read().tasks[&task_id].title.clone());
                                editing_title.set(true);
                            },
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10",
                            }
                        }
                    }}
                }
                div {
                    class: "flex flex-row gap-1",
                    for (user_id, user) in users {rsx!{
                        div {
                            class: "group relative",
                            onclick: |event| event.stop_propagation(),
                            div {
                                class: "
                                    w-5 h-5 rounded cursor-pointer
                                    border-2 {color_picker::border_class(&user.color)}
                                    {user_bg(&model, user_id, &user.color)}
                                    {color_picker::bg_hover_class(&user.color)}
                                ",
                                onclick: {
                                    let user_id = *user_id;
                                    move |event| {
                                        event.stop_propagation();
                                        let mut model = model.write();
                                        if model.user_filter.contains(&user_id) {
                                            model.user_filter.remove(&user_id);
                                        } else {
                                            model.user_filter.insert(user_id);
                                        }
                                    }
                                },
                            },
                            div {
                                dir: "rtl",
                                div {
                                    class: "
                                        pointer-events-none absolute start-0 w-max
                                        opacity-0 transition-opacity group-hover:opacity-100
                                        z-10 px-3 py-2 text-sm font-medium text-white
                                        rounded-lg shadow-sm tooltip bg-gray-800
                                        border border-gray-700
                                    ",
                                    "{user.name}"
                                }
                            }
                        }
                    }}
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "white",
                        class: "w-5 h-5 border border-white rounded cursor-pointer",
                        prevent_default: "onclick",
                        onclick: move |event| {
                            event.stop_propagation();
                            if !show_assign_user {
                                *assignees.write() = model.read().tasks[task_id].assignees.clone();
                                show_assign_user.set(true);
                            } else {
                               show_assign_user.set(false);
                            }
                        },
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M12 4.5v15m7.5-7.5h-15",
                        }
                    }
                }
            }
            if **show_assign_user {rsx!{
                div {
                    class: "p-2 rounded-lg",
                    onclick: |event| event.stop_propagation(),
                    UserSearch {
                        task_id: *task_id,
                        badge_style: "bg-inherit border border-gray-700",
                        ul_style: "border border-gray-700 divide-y divide-gray-700",
                        hover_style: "active:bg-gray-800 sm:hover:bg-gray-800",
                        text_input_style: "bg-gray-800",
                    }
                }
            }}
            if **expanded {rsx!{
                if **editing_description {rsx! {
                    textarea {
                        class: "p-4 bg-gray-900 rounded border border-gray-700 text-white",
                        rows: 8.max(data.description.lines().count() as i64),
                        oninput: |event| new_description.set(event.value.clone()),
                        value: "{new_description}",
                    }
                    div {
                        class: "grid grid-rows-1 justify-items-end",
                        div {
                            class: "flex flex-row gap-1",
                            button {
                                r#type: "button",
                                class: "
                                    rounded-lg p-1.5 inline-flex items-center justify-center h-8 w-8
                                    border border-green-500 text-green-500
                                    active:bg-green-500 active:text-white
                                    sm:hover:bg-green-500 sm:hover:text-white
                                ",
                                prevent_default: "onclick",
                                onclick: |event| {
                                    event.stop_propagation();
                                    editing_description.set(false);
                                    set_task_description(model.clone(), *task_id, (**new_description).clone())
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
                                    border border-red-500 text-red-500
                                    active:bg-red-500 active:text-white
                                    sm:hover:bg-red-500 sm:hover:text-white
                                ",
                                prevent_default: "onclick",
                                onclick: |event| {
                                    event.stop_propagation();
                                    editing_description.set(false);
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
                    }
                }} else {rsx! {
                    div {
                        class: "
                            text-sm text-gray-400 whitespace-pre-wrap break-words
                        ",
                        if data.description.is_empty() {rsx!{
                            "Description "
                        }} else {rsx!{
                            "{data.description} "
                        }}
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "w-4 h-4 text-gray-400 cursor-pointer inline-block",
                            onclick: move |event| {
                                event.stop_propagation();
                                new_description.set(model.read().tasks[&task_id].description.clone());
                                editing_description.set(true);
                            },
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10",
                            }
                        }
                    }
                }}
                div {
                    class: "grid grid-cols-4",
                    div {
                        class: "
                            col-span-3
                            flex flex-row items-center
                        ",
                        if let Some(due) = data.due {rsx! {
                            Due {
                                task_id: *task_id,
                                due: DueOptions {due, show_time_left: *status != TaskStatus::Done},
                                svg_style: "w-4 h-4",
                                p_style: "text-sm",
                            }
                        }} else {rsx! {
                            Due {
                                task_id: *task_id,
                                svg_style: "w-4 h-4",
                                p_style: "text-sm",
                            }
                        }}
                    }
                    div {
                        class: "grid grid-rows-1 justify-items-end",
                        div {
                            class: "flex flex-row gap-1 items-center",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                "viewBox": "0 0 24 24",
                                "stroke-width": "1.5",
                                stroke: "currentColor",
                                class: "cursor-pointer w-8 h-8 text-white active:text-red-600 sm:hover:text-red-600",
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
                                stroke: "currentColor",
                                "class": "cursor-pointer w-8 h-8 text-white active:text-yellow-300 sm:hover:text-yellow-300",
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
                                stroke: "currentColor",
                                class: "cursor-pointer w-8 h-8 text-white active:text-green-500 sm:hover:text-green-500",
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
                div {
                    class:"flex flex-row justify-between",
                    div {
                        class: "flex flex-row gap-2",
                        if **editing_size {rsx! {
                            span {
                                class: "
                                    text-sm font-medium px-2.5 py-0.5 rounded cursor-pointer
                                    border-2 border-emerald-700
                                    sm:hover:bg-emerald-700 bg-inherit text-green-300
                                ",
                                onclick: |event| {
                                    event.stop_propagation();
                                    editing_size.set(false);
                                    set_task_size(model.clone(), *task_id, TaskSize::Small)
                                },
                                "Small",
                            }
                            span {
                                class: "
                                    text-sm font-medium px-2.5 py-0.5 rounded cursor-pointer
                                    border-2 border-yellow-900
                                    sm:hover:bg-yellow-900 bg-inherit text-yellow-300
                                ",
                                onclick: |event| {
                                    event.stop_propagation();
                                    editing_size.set(false);
                                    set_task_size(model.clone(), *task_id, TaskSize::Medium)
                                },
                                "Medium",
                            }
                            span {
                                class: "
                                    text-sm font-medium px-2.5 py-0.5 rounded cursor-pointer
                                    border-2 border-red-900
                                    sm:hover:bg-red-900 bg-inherit text-red-300
                                ",
                                onclick: |event| {
                                    event.stop_propagation();
                                    editing_size.set(false);
                                    set_task_size(model.clone(), *task_id, TaskSize::Large)
                                },
                                "Large",
                            }
                        }} else {rsx! {
                            match data.size {
                                TaskSize::Small => {rsx!{
                                    span {
                                        class: "
                                            text-sm font-medium px-2.5 py-0.5 rounded  cursor-pointer
                                            flex flex-row gap-2 items-center
                                            border-2 border-emerald-700
                                            {size_bg(model, &data.size)}
                                            sm:hover:bg-emerald-700
                                            text-green-300
                                        ",
                                        onclick: |event| {
                                            event.stop_propagation();
                                            let mut model = model.write();
                                            if model.size_filter == Some(TaskSize::Small) {
                                                model.size_filter = None;
                                            } else {
                                                model.size_filter = Some(TaskSize::Small);
                                            }
                                        },
                                        "Small",
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "none",
                                            "viewBox": "0 0 24 24",
                                            "stroke-width": "1.5",
                                            stroke: "currentColor",
                                            class: "w-4 h-4",
                                            onclick: |event| {
                                                event.stop_propagation();
                                                editing_size.set(true);
                                            },
                                            path {
                                                "stroke-linecap": "round",
                                                "stroke-linejoin": "round",
                                                d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10",
                                            }
                                        }
                                    }
                                }}
                                TaskSize::Medium => {rsx!{
                                    span {
                                        class: "
                                            text-sm font-medium px-2.5 py-0.5 rounded cursor-pointer
                                            flex flex-row gap-2 items-center
                                            border-2 border-yellow-900
                                            sm:hover:bg-yellow-900
                                            {size_bg(model, &data.size)} text-yellow-300
                                        ",
                                        onclick: |event| {
                                            event.stop_propagation();
                                            let mut model = model.write();
                                            if model.size_filter == Some(TaskSize::Medium) {
                                                model.size_filter = None;
                                            } else {
                                                model.size_filter = Some(TaskSize::Medium);
                                            }
                                        },
                                        "Medium",
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "none",
                                            "viewBox": "0 0 24 24",
                                            "stroke-width": "1.5",
                                            stroke: "currentColor",
                                            class: "w-4 h-4",
                                            onclick: |event| {
                                                event.stop_propagation();
                                                editing_size.set(true);
                                            },
                                            path {
                                                "stroke-linecap": "round",
                                                "stroke-linejoin": "round",
                                                d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10",
                                            }
                                        }
                                    }
                                }}
                                TaskSize::Large => {rsx!{
                                    span {
                                        class: "
                                            text-sm font-medium px-2.5 py-0.5 rounded  cursor-pointer
                                            flex flex-row gap-2 items-center
                                            border-2 border-red-900
                                            sm:hover:bg-red-900
                                            {size_bg(model, &data.size)} text-red-300
                                        ",
                                        onclick: |event| {
                                            event.stop_propagation();
                                            let mut model = model.write();
                                            if model.size_filter == Some(TaskSize::Large) {
                                                model.size_filter = None;
                                            } else {
                                                model.size_filter = Some(TaskSize::Large);
                                            }
                                        },
                                        "Large",
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "none",
                                            "viewBox": "0 0 24 24",
                                            "stroke-width": "1.5",
                                            stroke: "currentColor",
                                            class: "w-4 h-4",
                                            onclick: |event| {
                                                event.stop_propagation();
                                                editing_size.set(true);
                                            },
                                            path {
                                                "stroke-linecap": "round",
                                                "stroke-linejoin": "round",
                                                d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10",
                                            }
                                        }
                                    }
                                }}
                            }
                        }}
                    }
                    div {
                        class: "flex flex-row gap-1",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "
                                w-6 h-6 cursor-pointer text-white
                                sm:hover:text-blue-500 active:text-blue-500
                            ",
                            onclick: move |event| {
                                event.stop_propagation();
                                create_quick_add_task(model.clone(), *task_id)
                            },
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z",
                            }
                        }
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "
                                w-6 h-6 cursor-pointer text-white
                                sm:hover:text-blue-500 active:text-blue-500
                            ",
                            onclick: move |event| {
                                event.stop_propagation();
                                clone_task(model.clone(), *task_id)
                            },
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 0 1-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 0 1 1.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 0 0-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 0 1-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 0 0-3.375-3.375h-1.5a1.125 1.125 0 0 1-1.125-1.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H9.75",
                            }
                        }
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "
                                w-6 h-6 cursor-pointer text-white
                                sm:hover:text-blue-500 active:text-blue-500
                            ",
                            onclick: move |event| {
                                event.stop_propagation();
                                archive_task(model.clone(), *task_id)
                            },
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z",
                            }
                        }
                    }
                }
                div {
                    class: "grid grid-cols-8",
                    div {
                        class: "col-span-7 flex flex-row flex-wrap gap-2",
                        Tags {
                            task_id: *task_id,
                            on_click_assign_tag: move |event: Event<MouseData>| {
                                event.stop_propagation();
                                show_assign_tag.set(!**show_assign_tag);
                            },
                        }
                    }
                }
                if **show_assign_tag {rsx!{
                    TagSearch {
                        task_id: *task_id,
                        ul_style: "border border-gray-700 divide-y divide-gray-700",
                        hover_style: "active:bg-gray-800 sm:hover:bg-gray-800",
                        text_input_style: "bg-gray-800"
                    }
                }}
                div {
                    class: "grid grid-rows-1 justify-items-end",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "w-6 h-6 cursor-pointer text-red-600",
                        onclick: move |event| {
                            event.stop_propagation();
                            delete_task(model.clone(), *task_id)
                        },
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0",
                        }
                    }
                }
            }}
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
    let draggable = use_state(cx, || true);
    let show_assign_user = use_state(cx, || false);
    let show_assign_tag = use_state(cx, || false);
    cx.render(rsx! {
        div {
            prevent_default: "onclick",
            draggable: **draggable,
            onclick: |_| expanded.set(!**expanded),
            class: "
                flex flex-col gap-2 p-3 border rounded-lg shadow
                bg-gray-800 border-gray-700 sm:hover:bg-gray-700",
            div {
                class: "grid grid-cols-2",
                if **editing_title {rsx!{
                    input {
                        class: styles::TEXT_INPUT,
                        r#type: "text",
                        oninput: |event| new_title.set(event.value.clone()),
                        onfocusout: |_| {
                            editing_title.set(false);
                            set_task_title(model.clone(), *task_id, (**new_title).clone())
                        },
                        onmouseenter: |_| draggable.set(false),
                        onmouseleave: |_| draggable.set(true),
                        value: "{new_title}",
                    }
                }} else {rsx!{
                    div {
                        class: "grid grid-rows-1 justify-items-start",
                        h5 {
                            class: "text-lg sm:text-xl font-bold tracking-tight text-white underline underline-offset-8",
                            onclick: move |event| {
                                event.stop_propagation();
                                editing_title.set(true);
                                new_title.set(model.read().tasks[&task_id].title.clone());
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
                            stroke: "currentColor",
                            class: "cursor-pointer w-8 h-8 text-white active:text-red-600 sm:hover:text-red-600",
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
                            stroke: "currentColor",
                            "class": "cursor-pointer w-8 h-8 text-white active:text-yellow-300 sm:hover:text-yellow-300",
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
                            stroke: "currentColor",
                            class: "cursor-pointer w-8 h-8 text-white active:text-green-500 sm:hover:text-green-500",
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
            div {
                class: "flex flex-row justify-between",
                Users {
                    task_id: *task_id,
                    on_click_assign_user: move |event: Event<MouseData>| {
                        event.stop_propagation();
                        show_assign_user.set(!**show_assign_user);
                    },
                },
                div {
                    class: "flex flex-row gap-1",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "
                            w-6 h-6 cursor-pointer text-white
                            sm:hover:text-blue-500 active:text-blue-500
                        ",
                        onclick: move |event| {
                            event.stop_propagation();
                            create_quick_add_task(model.clone(), *task_id)
                        },
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z",
                        }
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "
                            w-6 h-6 cursor-pointer text-white
                            sm:hover:text-blue-500 active:text-blue-500
                        ",
                        onclick: move |event| {
                            event.stop_propagation();
                            clone_task(model.clone(), *task_id)
                        },
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 0 1-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 0 1 1.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 0 0-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 0 1-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 0 0-3.375-3.375h-1.5a1.125 1.125 0 0 1-1.125-1.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H9.75",
                        }
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "
                            w-6 h-6 cursor-pointer text-white
                            sm:hover:text-blue-500 active:text-blue-500
                        ",
                        onclick: move |event| {
                            event.stop_propagation();
                            archive_task(model.clone(), *task_id)
                        },
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z",
                        }
                    }
                }
            }
            if **show_assign_user {rsx!{
                div {
                    class: "
                        bg-gray-800 p-2
                        rounded-lg border border-gray-700
                    ",
                    onclick: |event| event.stop_propagation(),
                    UserSearch {
                        task_id: *task_id,
                        badge_style: "",
                        ul_style: "
                            bg-gray-800
                            rounded-lg border border-gray-700
                            divide-y divide-gray-700
                        ",
                        hover_style: "active:bg-gray-700 sm:hover:bg-gray-700",
                        text_input_style: "",
                    }
                }
            }}
            div {
                class: "grid grid-cols-2",
                div {
                    class: "flex flex-row gap-2",
                    if **editing_size {rsx!{
                        span {
                            class: "
                                text-sm font-medium px-2.5 py-0.5 rounded cursor-pointer
                                border-2 border-emerald-700
                                sm:hover:bg-emerald-700 bg-inherit text-green-300
                            ",
                            onclick: |event| {
                                event.stop_propagation();
                                editing_size.set(false);
                                set_task_size(model.clone(), *task_id, TaskSize::Small)
                            },
                            "Small",
                        }
                        span {
                            class: "
                                text-sm font-medium px-2.5 py-0.5 rounded cursor-pointer
                                border-2 border-yellow-900
                                sm:hover:bg-yellow-900 bg-inherit text-yellow-300
                            ",
                            onclick: |event| {
                                event.stop_propagation();
                                editing_size.set(false);
                                set_task_size(model.clone(), *task_id, TaskSize::Medium)
                            },
                            "Medium",
                        }
                        span {
                            class: "
                                text-sm font-medium px-2.5 py-0.5 rounded cursor-pointer
                                border-2 border-red-900
                                sm:hover:bg-red-900 bg-inherit text-red-300
                            ",
                            onclick: |event| {
                                event.stop_propagation();
                                editing_size.set(false);
                                set_task_size(model.clone(), *task_id, TaskSize::Large)
                            },
                            "Large",
                        }
                    }} else {rsx!{
                        match data.size {
                            TaskSize::Small => {rsx!{
                                span {
                                    class: "
                                        text-sm font-medium px-2.5 py-0.5 rounded  cursor-pointer
                                        flex flex-row gap-2 items-center
                                        border-2 border-emerald-700
                                        {size_bg(model, &data.size)}
                                        sm:hover:bg-emerald-700
                                        text-green-300
                                    ",
                                    onclick: |event| {
                                        event.stop_propagation();
                                        let mut model = model.write();
                                        if model.size_filter == Some(TaskSize::Small) {
                                            model.size_filter = None;
                                        } else {
                                            model.size_filter = Some(TaskSize::Small);
                                        }
                                    },
                                    "Small",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        fill: "none",
                                        "viewBox": "0 0 24 24",
                                        "stroke-width": "1.5",
                                        stroke: "currentColor",
                                        class: "w-4 h-4",
                                        onclick: |event| {
                                            event.stop_propagation();
                                            editing_size.set(true);
                                        },
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10",
                                        }
                                    }
                                }
                            }}
                            TaskSize::Medium => {rsx!{
                                span {
                                    class: "
                                        text-sm font-medium px-2.5 py-0.5 rounded cursor-pointer
                                        flex flex-row gap-2 items-center
                                        border-2 border-yellow-900
                                        sm:hover:bg-yellow-900
                                        {size_bg(model, &data.size)} text-yellow-300
                                    ",
                                    onclick: |event| {
                                        event.stop_propagation();
                                        let mut model = model.write();
                                        if model.size_filter == Some(TaskSize::Medium) {
                                            model.size_filter = None;
                                        } else {
                                            model.size_filter = Some(TaskSize::Medium);
                                        }
                                    },
                                    "Medium",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        fill: "none",
                                        "viewBox": "0 0 24 24",
                                        "stroke-width": "1.5",
                                        stroke: "currentColor",
                                        class: "w-4 h-4",
                                        onclick: |event| {
                                            event.stop_propagation();
                                            editing_size.set(true);
                                        },
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10",
                                        }
                                    }
                                }
                            }}
                            TaskSize::Large => {rsx!{
                                span {
                                    class: "
                                        text-sm font-medium px-2.5 py-0.5 rounded  cursor-pointer
                                        flex flex-row gap-2 items-center
                                        border-2 border-red-900
                                        sm:hover:bg-red-900
                                        {size_bg(model, &data.size)} text-red-300
                                    ",
                                    onclick: |event| {
                                        event.stop_propagation();
                                        let mut model = model.write();
                                        if model.size_filter == Some(TaskSize::Large) {
                                            model.size_filter = None;
                                        } else {
                                            model.size_filter = Some(TaskSize::Large);
                                        }
                                    },
                                    "Large",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        fill: "none",
                                        "viewBox": "0 0 24 24",
                                        "stroke-width": "1.5",
                                        stroke: "currentColor",
                                        class: "w-4 h-4",
                                        onclick: |event| {
                                            event.stop_propagation();
                                            editing_size.set(true);
                                        },
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10",
                                        }
                                    }
                                }
                            }}
                        }
                    }}
                }
            }
            if let Some(due_value) = data.due {rsx!{
                Due {
                    task_id: *task_id,
                    due: DueOptions{
                        due: due_value,
                        show_time_left: match status {
                            TaskStatus::ToDo | TaskStatus::InProgress => true,
                            TaskStatus::Done => false,
                        }
                    },
                    svg_style: "w-6 h-6",
                    p_style: "",
                }
            }}
            if **expanded && data.due.is_none() {rsx!{
                Due {
                    task_id: *task_id,
                    svg_style: "w-6 h-6",
                    p_style: "",
                }
            }}
            if **expanded {rsx!{
                if **editing_description {rsx!{
                    textarea {
                        class: "p-4 bg-gray-900 rounded border border-gray-700 text-white",
                        rows: 8.max(data.description.lines().count() as i64),
                        oninput: |event| new_description.set(event.value.clone()),
                        onfocusout: |_| {
                            editing_description.set(false);
                            set_task_description(model.clone(), *task_id, (**new_description).clone())
                        },
                        onmouseenter: |_| draggable.set(false),
                        onmouseleave: |_| draggable.set(true),
                        value: "{new_description}",
                    }

                }} else {rsx!{
                    div {
                        class: "
                            p-4 bg-gray-900 rounded border border-gray-700 text-white
                            whitespace-pre-wrap break-words
                        ",
                        onclick: move |event| {
                            event.stop_propagation();
                            editing_description.set(true);
                            new_description.set(model.read().tasks[task_id].description.clone());
                        },
                        "{data.description}"
                    }
                }}
                div {
                    class: "grid grid-cols-8",
                    div {
                        class: "col-span-7 flex flex-row flex-wrap gap-2",
                        Tags {
                            task_id: *task_id,
                            on_click_assign_tag: move |event: Event<MouseData>| {
                                event.stop_propagation();
                                show_assign_tag.set(!**show_assign_tag);
                            },
                        }
                    }
                }
                if **show_assign_tag {rsx!{
                    TagSearch {
                        task_id: *task_id,
                        ul_style: "
                            bg-gray-800
                            border border-gray-700
                            divide-y divide-gray-700
                        ",
                        hover_style: "active:bg-gray-700 sm:hover:bg-gray-700",
                        text_input_style: "",
                    }
                }}
            div {
                class: "grid grid-rows-1 justify-items-end",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    "viewBox": "0 0 24 24",
                    "stroke-width": "1.5",
                    stroke: "currentColor",
                    class: "w-6 h-6 cursor-pointer text-red-600",
                    onclick: move |event| {
                        event.stop_propagation();
                        delete_task(model.clone(), *task_id)
                    },
                    path {
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        d: "m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0",
                    }
                }
            }
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
fn Due(
    cx: Scope,
    task_id: TaskId,
    due: Option<DueOptions>,
    svg_style: &'static str,
    p_style: &'static str,
) -> Element {
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
                    class: "text-gray-400 cursor-pointer {svg_style}",
                    "aria-hidden": "true",
                    "xmlns": "http://www.w3.org/2000/svg",
                    "fill": "none",
                    "viewBox": "0 0 20 20",
                    onclick: |event| {
                        event.stop_propagation();
                        editing.set(false);
                    },
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
                        border border-green-500 text-green-500
                        active:bg-green-500 active:text-white
                        sm:hover:bg-green-500 sm:hover:text-white
                    ",
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
                        border border-red-500 text-red-500
                        active:bg-red-500 active:text-white
                        sm:hover:bg-red-500 sm:hover:text-white
                    ",
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
                        class: "text-gray-400 cursor-pointer {svg_style}",
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
                        class: "font-normal text-gray-400 {p_style}",
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
                    svg {
                        class: "text-gray-400 cursor-pointer {svg_style}",
                        onclick: move |_| {
                            editing.set(true);
                            new_date.set(None);
                            new_time.set(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                        },
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

fn user_bg(model: &UseSharedState<Model>, user_id: &UserId, user_color: &Color) -> String {
    if model.read().user_filter.contains(user_id) {
        format!("{} ring ring-blue-500", color_picker::bg_class(user_color))
    } else {
        "bg-inherit".into()
    }
}

#[component]
fn Users<'a>(
    cx: Scope,
    task_id: TaskId,
    on_click_assign_user: EventHandler<'a, Event<MouseData>>,
) -> Element<'a> {
    let model = use_shared_state::<Model>(cx).unwrap();
    let read_model = model.read();
    let data = &read_model.tasks[task_id];
    let users: Vec<_> = data
        .assignees
        .iter()
        .map(|user_id| (user_id, &read_model.users[user_id]))
        .collect();
    cx.render(rsx! {
        div {
            class: "flex flex-col gap-2",
            div {
                class: "flex flex-row flex-wrap gap-2",
                for (user_id, user) in users {rsx!{
                    div {
                        class: "group relative",
                        onclick: |event| event.stop_propagation(),
                        div {
                            class: "
                                w-6 h-6 rounded cursor-pointer
                                border-2 {color_picker::border_class(&user.color)}
                                {user_bg(&model, user_id, &user.color)}
                                {color_picker::bg_hover_class(&user.color)}
                            ",
                            onclick: {
                                let user_id = *user_id;
                                move |event| {
                                    event.stop_propagation();
                                    let mut model = model.write();
                                    if model.user_filter.contains(&user_id) {
                                        model.user_filter.remove(&user_id);
                                    } else {
                                        model.user_filter.insert(user_id);
                                    }
                                }
                            },
                        },
                        div {
                            class: styles::TOOLTIP,
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
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "white",
                        class: "w-6 h-6 border border-white rounded cursor-pointer",
                        prevent_default: "onclick",
                        onclick: |event| on_click_assign_user.call(event),
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M12 4.5v15m7.5-7.5h-15",
                        }
                    }
                    div {
                        class: styles::TOOLTIP,
                        "Assign User"
                        div {
                            class: "tooltip-arrow",
                            "data-popper-arrow": "",
                        }
                    }
                }
            }
        }
    })
}

fn tag_bg(model: &UseSharedState<Model>, tag_id: &TagId, tag_color: &Color) -> String {
    if model.read().tag_filter.contains(tag_id) {
        format!("{} ring ring-blue-500", color_picker::bg_class(tag_color))
    } else {
        "bg-inherit".into()
    }
}

#[component]
fn Tags<'a>(
    cx: Scope,
    task_id: TaskId,
    on_click_assign_tag: EventHandler<'a, Event<MouseData>>,
) -> Element<'a> {
    let model = use_shared_state::<Model>(cx).unwrap();
    let read_model = model.read();
    cx.render(rsx! {
        for (tag_id, tag) in read_model
            .tasks[task_id]
            .tags
            .iter()
            .map(|tag_id| (tag_id, &read_model.tags[tag_id]))
        {rsx!{
            span {
                class: "
                    {styles::TAG_BADGE_SPAN}
                    {tag_bg(model, tag_id, &tag.color)}
                    {color_picker::bg_hover_class(&tag.color)}
                    {color_picker::border_class(&tag.color)}
                ",
                onclick: {
                    let tag_id = *tag_id;
                    move |event| {
                        event.stop_propagation();
                        let mut model = model.write();
                        if model.tag_filter.contains(&tag_id) {
                            model.tag_filter.remove(&tag_id);
                        } else {
                            model.tag_filter.insert(tag_id);
                        }
                    }
                },
                "# {tag.name}",
                button {
                    r#type: "button",
                    class: "{styles::TAG_BADGE_BUTTON}",
                    onclick: {
                        let task_id = *task_id;
                        let tag_id = *tag_id;
                        move |event| {
                            event.stop_propagation();
                            delete_task_tag(model.clone(), task_id, tag_id)
                        }
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
                            d: "m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
                        }
                    }
                }
            }
        }}
        div {
            class: "group relative",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                "viewBox": "0 0 24 24",
                "stroke-width": "1.5",
                stroke: "white",
                class: "w-6 h-6 border border-white rounded cursor-pointer",
                prevent_default: "onclick",
                onclick: |event| on_click_assign_tag.call(event),
                path {
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    d: "M12 4.5v15m7.5-7.5h-15",
                }
            }
            div {
                class: styles::TOOLTIP,
                "Assign Tag"
                div {
                    class: "tooltip-arrow",
                    "data-popper-arrow": "",
                }
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

async fn archive_task(model: UseSharedState<Model>, task_id: TaskId) {
    if send_archive_task_request(model.clone(), task_id)
        .await
        .is_ok()
    {
        requests::board(model.clone()).await;
    }
}

async fn send_archive_task_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/archived",
            model.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .put(url)
        .json(&true)
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

async fn delete_task(model: UseSharedState<Model>, task_id: TaskId) {
    if send_delete_task_request(model.clone(), task_id)
        .await
        .is_ok()
    {
        requests::board(model).await;
    }
}

async fn send_delete_task_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}",
            model.board_name, task_id
        ))?
    };
    Ok(reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn delete_task_tag(model: UseSharedState<Model>, task_id: TaskId, tag_id: TagId) {
    if send_delete_task_tag_request(model.clone(), task_id, tag_id)
        .await
        .is_ok()
    {
        requests::board(model).await;
    }
}

async fn send_delete_task_tag_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
    tag_id: TagId,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/tags/{}",
            model.board_name, task_id, tag_id
        ))?
    };
    Ok(reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn clone_task(model: UseSharedState<Model>, task_id: TaskId) {
    if send_clone_task_request(model.clone(), task_id)
        .await
        .is_ok()
    {
        requests::board(model).await;
    }
}

async fn send_clone_task_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
) -> Result<TaskId, anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/clone",
            model.board_name, task_id
        ))?
    };
    Ok(reqwest::Client::new()
        .post(url)
        .send()
        .await?
        .json::<TaskId>()
        .await?)
}

async fn create_quick_add_task(model: UseSharedState<Model>, task_id: TaskId) {
    if send_create_quick_add_task_request(model.clone(), task_id)
        .await
        .is_ok()
    {
        requests::board(model).await;
    }
}

async fn send_create_quick_add_task_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
) -> Result<TaskId, anyhow::Error> {
    let (url, task_data) = {
        let model = model.read();
        let url = model
            .url
            .join(&format!("/api/boards/{}/quick-add", model.board_name))?;
        let task = &model.tasks[&task_id];
        (
            url,
            QuickAddData {
                title: task.title.clone(),
                description: task.description.clone(),
                size: task.size.clone(),
                tags: task.tags.clone(),
                assignees: task.assignees.clone(),
            },
        )
    };
    Ok(reqwest::Client::new()
        .post(url)
        .json(&task_data)
        .send()
        .await?
        .json::<TaskId>()
        .await?)
}

async fn delete_quick_add_task(model: UseSharedState<Model>, task_id: QuickAddTaskId) {
    if send_delete_quick_add_task_request(model.clone(), task_id)
        .await
        .is_ok()
    {
        requests::board(model).await;
    }
}

async fn send_delete_quick_add_task_request(
    model: UseSharedState<Model>,
    task_id: QuickAddTaskId,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/quick-add/{}",
            model.board_name, task_id
        ))?
    };
    Ok(reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn create_task(model: UseSharedState<Model>, task_data: TaskData) {
    if task_data.title.is_empty() {
        log::info!("empty task title, doing nothing");
        return;
    }
    if let Ok(task_id) = requests::create_task(&model, &task_data).await {
        log::info!("created task: {task_id}");
    }
    requests::board(model).await;
}
