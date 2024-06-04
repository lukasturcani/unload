use dioxus::prelude::*;
use shared_models::{BoardName, TaskStatus};

use crate::{
    components::{
        icons::{DoneIcon, InProgressIcon, ToDoIcon},
        nav::NavBar,
        task::{DenseTask, Task},
    },
    model::{task_filter, Board, TagFilter, Tasks, UserFilter},
    requests::{self, BoardSignals},
    themes::Theme,
};

#[component]
pub fn ThreeColumnBoard(board_name: BoardName) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        text-white stroke-white
        {}
    ",
        theme.bg_color_1
    );
    let dense = use_signal(|| false);
    let dense_ = dense();
    let mut board_signals = BoardSignals::default();
    if board_signals.model.read().board_name != board_name {
        board_signals.model.write().board_name = board_name.clone();
        board_signals.board.write().board_name = board_name.clone();
    }
    use_future(move || requests::board(board_signals));
    rsx! {
        div {
            class: "flex flex-col h-dvh w-screen {style}",
            div {
                class: "grow flex flex-col gap-2 overflow-y-auto p-4 pb-2",
                div {
                    class: "grow w-full h-full overflow-y-auto",
                    div {
                        class: "w-full h-full grid grid-cols-3 gap-2 overflow-y-auto",
                        Column { status: TaskStatus::ToDo, dense: dense_ }
                        Column { status: TaskStatus::InProgress, dense: dense_ }
                        Column { status: TaskStatus::Done, dense: dense_ }
                    },
                }
                // FilterBar {}
            }
            NavBar { board_name }
        }
    }
}

#[component]
fn Column(status: TaskStatus, dense: bool) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let heading_style = "text-3xl font-extrabold";
    let style = format!(
        "
        border {}
        ",
        theme.border_color
    );
    rsx! {
        section {
            class: "flex flex-col overflow-y-auto {style}",
            div {
                class: "flex items-center gap-2",
                match status {
                    TaskStatus::ToDo => rsx! {
                        div { class: "size-8", ToDoIcon {} }
                        h2 {
                            class: heading_style,
                            "To Do"
                        }
                    },
                    TaskStatus::InProgress => rsx! {
                        div { class: "size-8", InProgressIcon {} }
                        h2 {
                            class: heading_style,
                            "In Progress"
                        }
                    },
                    TaskStatus::Done => rsx! {
                        div { class: "size-8", DoneIcon {} }
                        h2 {
                            class: heading_style,
                            "Done"
                        }
                    }
                }
            }
            div {
                class: "grow flex flex-col gap-2 overflow-y-scroll",
                if dense {
                    DenseColumnTasks { status }
                } else {
                    ColumnTasks { status }
                }
            }
        }
    }
}

#[component]
fn ColumnTasks(status: TaskStatus) -> Element {
    let tasks = use_context::<Signal<Tasks>>();
    let tasks = tasks.read();
    let board = use_context::<Signal<Board>>();
    let board = board.read();
    let user_filter = use_context::<Signal<UserFilter>>();
    let user_filter = user_filter.read();
    let tag_filter = use_context::<Signal<TagFilter>>();
    let tag_filter = tag_filter.read();
    let column_tasks = match status {
        TaskStatus::ToDo => &board.to_do,
        TaskStatus::InProgress => &board.in_progress,
        TaskStatus::Done => &board.done,
    };
    rsx! {
        for task_id in column_tasks
            .iter()
            .filter(|task_id| {
                task_filter(task_id, &tasks.0, &user_filter.0, &tag_filter.0)
            })
        {
            Task {
                key: "{task_id}",
                task_id: *task_id,
                task: tasks.0[task_id].clone(),
                status: status,
            }
        }
    }
}

#[component]
fn DenseColumnTasks(status: TaskStatus) -> Element {
    let tasks = use_context::<Signal<Tasks>>();
    let tasks = tasks.read();
    let board = use_context::<Signal<Board>>();
    let board = board.read();
    let user_filter = use_context::<Signal<UserFilter>>();
    let user_filter = user_filter.read();
    let tag_filter = use_context::<Signal<TagFilter>>();
    let tag_filter = tag_filter.read();
    let column_tasks = match status {
        TaskStatus::ToDo => &board.to_do,
        TaskStatus::InProgress => &board.in_progress,
        TaskStatus::Done => &board.done,
    };
    rsx! {
        for task_id in column_tasks
            .iter()
            .filter(|task_id| {
                task_filter(task_id, &tasks.0, &user_filter.0, &tag_filter.0)
            })
        {
            DenseTask {
                key: "{task_id}",
                task_id: *task_id,
                task: tasks.0[task_id].clone(),
                status: status,
            }
        }
    }
}
