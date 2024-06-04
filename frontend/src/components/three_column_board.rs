use dioxus::prelude::*;
use shared_models::{BoardName, TaskStatus};

use crate::{
    components::{nav::NavBar, task::Task},
    model::{Board, TagFilter, Tasks, UserFilter},
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
                        if board_signals.model.read().dense_view {
                            // DenseToDoColumn {}
                            // DenseInProgressColumn {}
                            // DenseDoneColumn {}
                        } else {
                            Column { status: TaskStatus::ToDo }
                            Column { status: TaskStatus::InProgress }
                            Column { status: TaskStatus::Done }
                        }
                    },
                }
                // FilterBar {}
            }
            NavBar { board_name }
        }
    }
}

#[component]
fn Column(status: TaskStatus) -> Element {
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
        div {
            for task_id in column_tasks
                .iter()
                .filter(|task_id| {
                    let task = &tasks.0[task_id];
                    if user_filter
                        .0
                        .iter()
                        .any(|user_id| !task.assignees.contains(user_id))
                    {
                        return false;
                    }
                    if tag_filter
                        .0
                        .iter()
                        .any(|tag_id| !task.tags.contains(tag_id))
                    {
                        return false;
                    }
                    true
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
}
