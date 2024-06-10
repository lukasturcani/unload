use dioxus::prelude::*;

use crate::components::filter_bar::{TagFilter, UserFilter};
use crate::components::task::{DenseTask, Task};
use crate::components::three_column_board::ThreeColumnBoard;
use crate::responsive_layout::ResponsiveLayout;
use crate::route::Route;
use dioxus_router::hooks::use_navigator;
use itertools::Itertools;
use shared_models::UserId;
use shared_models::{Color, QuickAddTaskId, TaskData, TaskStatus};

use crate::model::Model;
use crate::requests::{self, BoardSignals};
use crate::{color_picker, styles};
use shared_models::{BoardName, TaskId};

pub const COLUMN: &str = "
    grow flex flex-col gap-2 rounded bg-gray-900 pt-2 px-2 sm:pt-4 sm:px-4 overflow-y-auto
";
pub const COLUMN_HEADING: &str = "text-xl sm:text-3xl font-extrabold text-white";
pub const COLUMN_TASK_LIST: &str = "grow flex flex-col gap-2 overflow-y-scroll";
pub const DENSE_COLUMN_TASK_LIST: &str = "grow flex flex-col overflow-y-scroll";

#[component]
pub fn Board(board_name: BoardName) -> Element {
    let layout = ResponsiveLayout::from_window();
    eval(&format!(r#"document.title = "{board_name}";"#));
    rsx! {
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
    }
}

#[component]
fn OneColumnBoard(board_name: BoardName) -> Element {
    let mut board_signals = BoardSignals::default();
    if board_signals.model.read().board_name != board_name {
        board_signals.model.write().board_name = board_name.clone();
        board_signals.board.write().board_name = board_name.clone();
    }
    let nav = use_navigator();

    let mut column_signal = use_signal(|| TaskStatus::ToDo);
    let column = column_signal();

    let mut show_filters_signal = use_signal(|| false);
    let show_filters = show_filters_signal();

    use_future(move || requests::board(board_signals));

    rsx! {
        div {
            class: "flex flex-col bg-gray-900 h-dvh w-screen gap-1 text-white stroke-white",
            div {
                class: "grow grid grid-cols-1 p-1 overflow-y-auto",
                match (column, board_signals.model.read().dense_view) {
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
                if show_filters {
                   div {
                       class: "w-full bg-gray-800 flex flex-col gap-2 p-2",
                        TagFilter {}
                        UserFilter {}
                   }
                }
                div {
                    class: styles::BOTTOM_BAR,
                    button {
                        r#type: "button",
                        class: styles::BOTTOM_BAR_BUTTON,
                        disabled: column == TaskStatus::ToDo,
                        onclick: move |_| {
                            match column {
                                TaskStatus::ToDo => column_signal.set(TaskStatus::ToDo),
                                TaskStatus::InProgress => column_signal.set(TaskStatus::ToDo),
                                TaskStatus::Done => column_signal.set(TaskStatus::InProgress),
                            }
                        },
                        if column != TaskStatus::ToDo {
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
                        }
                    }
                    button {
                        r#type: "button",
                        class: styles::BOTTOM_BAR_BUTTON,
                        onclick: {
                            let board_name = board_name.clone();
                            move |_| {
                                // nav.push(Route::TaskArchive {
                                //     board_name: board_name.clone(),
                                // });
                            }
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
                        class: if show_filters {
                            styles::BOTTOM_BAR_ACTIVE_BUTTON
                        } else {
                            styles::BOTTOM_BAR_BUTTON
                        },
                        onclick: move |_| show_filters_signal.set(!show_filters),
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: if show_filters {
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
                        onclick: {
                            let board_name = board_name.clone();
                            move |_| {
                                nav.push(Route::Tags {
                                    board_name: board_name.clone(),
                                });
                            }
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
                        onclick: move |_| {
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
                        disabled: column == TaskStatus::Done,
                        onclick: move |_| {
                            match column {
                                TaskStatus::ToDo => column_signal.set(TaskStatus::InProgress),
                                TaskStatus::InProgress => column_signal.set(TaskStatus::Done),
                                TaskStatus::Done => column_signal.set(TaskStatus::Done),
                            }
                        },
                        if column != TaskStatus::Done {
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
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ToDoColumn() -> Element {
    let mut model = use_context::<Signal<Model>>();
    let read_model = model.read();
    let nav = use_navigator();

    let mut show_quick_add_signal = use_signal(|| false);
    let show_quick_add = show_quick_add_signal();

    rsx! {
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
                        onclick: move |event| {
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
                            task: read_model.tasks[&task_id].clone(),
                            status: TaskStatus::ToDo,
                        }
                    }
                },
            }
            if show_quick_add {
                QuickAddTasks {
                    status: TaskStatus::ToDo,
                }
            }
            div {
                class: "grid grid-cols-2 divide-x border-gray-700",
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: move |_| {
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
                    onclick: move |_| show_quick_add_signal.set(!show_quick_add),
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
    }
}

#[component]
fn DenseToDoColumn() -> Element {
    let mut model = use_context::<Signal<Model>>();
    let read_model = model.read();
    let nav = use_navigator();

    let mut show_quick_add_signal = use_signal(|| false);
    let show_quick_add = show_quick_add_signal();

    rsx! {
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
                        onclick: move |event| {
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
                            task: read_model.tasks[&task_id].clone(),
                            status: TaskStatus::ToDo,
                        }
                    }
                },
            }
            if show_quick_add {
                QuickAddTasks {
                    status: TaskStatus::ToDo,
                }
            }
            div {
                class: "grid grid-cols-2 divide-x border-gray-700",
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: move |_| {
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
                    onclick: move |_| show_quick_add_signal.set(!show_quick_add),
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
    }
}

#[component]
fn InProgressColumn() -> Element {
    let mut model = use_context::<Signal<Model>>();
    let read_model = model.read();
    let nav = use_navigator();

    let mut show_quick_add_signal = use_signal(|| false);
    let show_quick_add = show_quick_add_signal();

    rsx! {
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
                        onclick: move |event| {
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
                            task: read_model.tasks[&task_id].clone(),
                            status: TaskStatus::InProgress,
                        }
                    }
                },
            }
            if show_quick_add {
                QuickAddTasks {
                    status: TaskStatus::InProgress,
                }
            }
            div {
                class: "grid grid-cols-2 divide-x border-gray-700",
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: move |_| {
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
                    onclick: move |_| show_quick_add_signal.set(!show_quick_add),
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
    }
}

#[component]
fn DenseInProgressColumn() -> Element {
    let mut model = use_context::<Signal<Model>>();
    let read_model = model.read();
    let nav = use_navigator();

    let mut show_quick_add_signal = use_signal(|| false);
    let show_quick_add = show_quick_add_signal();

    rsx! {
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
                        onclick: move |event| {
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
                            task: read_model.tasks[&task_id].clone(),

                        }
                    }
                },
            }
            if show_quick_add {
                QuickAddTasks {
                    status: TaskStatus::InProgress,
                }
            }
            div {
                class: "grid grid-cols-2 divide-x border-gray-700",
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: move |_| {
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
                    onclick: move |_| show_quick_add_signal.set(!show_quick_add),
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
    }
}

#[component]
fn DoneColumn() -> Element {
    let mut model = use_context::<Signal<Model>>();
    let read_model = model.read();
    let nav = use_navigator();

    let mut show_quick_add_signal = use_signal(|| false);
    let show_quick_add = show_quick_add_signal();

    rsx! {
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
                        onclick: move |event| {
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
                            task: read_model.tasks[&task_id].clone(),
                            status: TaskStatus::Done,
                        }
                    }
                },
            }
            if show_quick_add {
                QuickAddTasks {
                    status: TaskStatus::Done,
                }
            }
            div {
                class: "grid grid-cols-2 divide-x border-gray-700",
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: move |_| {
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
                    onclick: move |_| show_quick_add_signal.set(!show_quick_add),
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
    }
}

#[component]
fn DenseDoneColumn() -> Element {
    let mut model = use_context::<Signal<Model>>();
    let read_model = model.read();
    let nav = use_navigator();

    let mut show_quick_add_signal = use_signal(|| false);
    let show_quick_add = show_quick_add_signal();

    rsx! {
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
                        onclick: move |event| {
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
                            task: read_model.tasks[&task_id].clone(),
                        }
                    }
                },
            }
            if show_quick_add {
                QuickAddTasks {
                    status: TaskStatus::Done,
                }
            }
            div {
                class: "grid grid-cols-2 divide-x border-gray-700",
                button {
                    r#type: "button",
                    class: " grid place-items-center group p-2 border-t border-gray-700",
                    onclick: move |_| {
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
                    onclick: move |_| show_quick_add_signal.set(!show_quick_add),
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
    }
}

#[component]
fn QuickAddTasks(status: TaskStatus) -> Element {
    let model = use_context::<Signal<Model>>();
    rsx! {
        ul {
            class: "
                text-sm text-gray-200
                border-t border-gray-700 divide-y divide-gray-700
                shrink-0 h-1/4 overflow-y-scroll
            ",
            for &task_id in model
                .read()
                .quick_add
                .keys()
                .sorted()
            {
                QuickAddTask {
                    key: "{task_id}",
                    task_id,
                    status,
                }
            }
        }
    }
}

#[component]
fn QuickAddTask(task_id: QuickAddTaskId, status: TaskStatus) -> Element {
    let board_signals = BoardSignals::default();
    let mut model = use_context::<Signal<Model>>();
    let read_model = model.read();
    let data = &read_model.quick_add[&task_id];
    let users: Vec<_> = data
        .assignees
        .iter()
        .map(|user_id| (user_id, &read_model.users[user_id]))
        .collect();
    rsx! {
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
                    let data = &read_model.quick_add[&task_id];
                    event.stop_propagation();
                    create_task(board_signals, TaskData {
                        title: data.title.clone(),
                        description: data.description.clone(),
                        size: data.size,
                        assignees: data.assignees.clone(),
                        status,
                        tags: data.tags.clone(),
                        due: None,
                    })
                },
                div {
                    class: "flex flex-row justify-between",
                    "{data.title}"
                    div {
                        class: "flex flex-row gap-1",
                        for (&user_id, user) in users {
                            div {
                                class: "group relative",
                                onclick: |event| event.stop_propagation(),
                                div {
                                    class: "
                                        w-5 h-5 rounded cursor-pointer
                                        border-2 {color_picker::border_class(&user.color)}
                                        {user_bg(model, &user_id, &user.color)}
                                        {color_picker::bg_hover_class(&user.color)}
                                    ",
                                    onclick: move |event| {
                                        event.stop_propagation();
                                        let mut model = model.write();
                                        if model.user_filter.contains(&user_id) {
                                            model.user_filter.remove(&user_id);
                                        } else {
                                            model.user_filter.insert(user_id);
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
                        }
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "w-6 h-6 cursor-pointer text-red-600",
                            onclick: move |event| {
                                event.stop_propagation();
                                delete_quick_add_task(board_signals, task_id)
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
    }
}

fn user_bg(model: Signal<Model>, user_id: &UserId, user_color: &Color) -> String {
    if model.read().user_filter.contains(user_id) {
        format!("{} ring ring-blue-500", color_picker::bg_class(user_color))
    } else {
        "bg-inherit".into()
    }
}

#[component]
fn Users(task_id: TaskId, on_click_assign_user: EventHandler<MouseEvent>) -> Element {
    let mut model = use_context::<Signal<Model>>();
    let read_model = model.read();
    let data = &read_model.tasks[&task_id];
    let users: Vec<_> = data
        .assignees
        .iter()
        .map(|user_id| (user_id, &read_model.users[user_id]))
        .collect();
    rsx! {
        div {
            class: "flex flex-col gap-2",
            div {
                class: "flex flex-row flex-wrap gap-2",
                for (&user_id, user) in users {
                    div {
                        class: "group relative",
                        onclick: |event| event.stop_propagation(),
                        div {
                            class: "
                                w-6 h-6 rounded cursor-pointer
                                border-2 {color_picker::border_class(&user.color)}
                                {user_bg(model, &user_id, &user.color)}
                                {color_picker::bg_hover_class(&user.color)}
                            ",
                            onclick:  move |event| {
                                event.stop_propagation();
                                let mut model = model.write();
                                if model.user_filter.contains(&user_id) {
                                    model.user_filter.remove(&user_id);
                                } else {
                                    model.user_filter.insert(user_id);
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
                }
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
                        onclick: move |event| on_click_assign_user.call(event),
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
    }
}

async fn delete_quick_add_task(signals: BoardSignals, task_id: QuickAddTaskId) {
    if send_delete_quick_add_task_request(signals, task_id)
        .await
        .is_ok()
    {
        requests::board(signals).await;
    }
}

async fn send_delete_quick_add_task_request(
    signals: BoardSignals,
    task_id: QuickAddTaskId,
) -> Result<(), anyhow::Error> {
    let url = {
        let board = signals.board.read();
        board.url.join(&format!(
            "/api/boards/{}/quick-add/{}",
            board.board_name, task_id
        ))?
    };
    Ok(reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn create_task(signals: BoardSignals, task_data: TaskData) {
    if task_data.title.is_empty() {
        log::info!("empty task title, doing nothing");
        return;
    }
    if let Ok(task_id) = requests::create_task(signals.board, &task_data).await {
        log::info!("created task: {task_id}");
    }
    requests::board(signals).await;
}
