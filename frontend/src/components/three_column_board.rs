use dioxus::prelude::*;
use shared_models::{BoardName, TaskId, TaskStatus};

use crate::{
    components::{
        form::{CancelButton, ConfirmButton},
        icons::{CircledPlusIcon, DoneIcon, InProgressIcon, StackIcon, ToDoIcon},
        input::TextInput,
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
            Header {
                body: rsx!{
                    div {}
                    h1 {
                        class: "text-3xl font-extrabold",
                        "{board_name}"
                    }
                    DenseButton { dense }
                }
            }
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
fn DenseButton(dense: Signal<bool>) -> Element {
    let style = "
        border-2 rounded
        aria-pressed:bg-white aria-pressed:stroke-black
    ";
    let dense_ = dense();
    rsx! {
        button {
            class: "size-9 p-1 {style}",
            "aria-pressed": dense_,
            onclick: move |_| dense.set(!dense_),
            StackIcon {}
        }
    }
}

#[component]
fn Header(body: Element) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        border-b {}
        ",
        theme.border_color
    );
    rsx! {
        header {
            class: "
                flex flex-row items-center justify-around
                w-full h-14 shrink-0 grow-0
                {style}
            ",
            {body}
        }
    }
}

#[component]
fn ColumnHeading(value: String) -> Element {
    let style = "text-3xl font-extrabold";
    rsx! {
        h2 { class: style, {value} }
    }
}

#[component]
fn Column(status: TaskStatus, dense: bool) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border {}", theme.border_color);
    let gap = if dense { "" } else { "gap-2" };
    let adding_task = use_signal(|| false);
    let new_task = use_signal(|| None);
    let scroll_to = use_signal(|| None);
    rsx! {
        section {
            class: "flex flex-col overflow-y-auto px-2 pt-2 {style}",
            div {
                class: "flex items-center gap-2",
                match status {
                    TaskStatus::ToDo => rsx! {
                        div { class: "size-8", ToDoIcon {} }
                        ColumnHeading { value: "To Do" }
                    },
                    TaskStatus::InProgress => rsx! {
                        div { class: "size-8", InProgressIcon {} }
                        ColumnHeading { value: "In Progress" }
                    },
                    TaskStatus::Done => rsx! {
                        div { class: "size-8", DoneIcon {} }
                        ColumnHeading { value: "Done" }
                    }
                }
            }
            div {
                class: "grow flex flex-col {gap} overflow-y-scroll pt-2",
                if dense {
                    DenseColumnTasks { status }
                } else {
                    ColumnTasks { status, scroll_to: scroll_to() }
                }
                if adding_task() {
                    NewTaskForm { status, adding_task, new_task, scroll_to }
                }
            }
            AddTaskButton { adding_task, new_task }
        }
    }
}

#[component]
fn NewTaskForm(
    status: TaskStatus,
    adding_task: Signal<bool>,
    new_task: Signal<Option<MountedEvent>>,
    scroll_to: Signal<Option<TaskId>>,
) -> Element {
    let board_signals = BoardSignals::default();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        border
        rounded-lg
        shadow
        {} {}
        ",
        theme.border_color, theme.bg_color_2,
    );
    rsx! {
        form {
            class: "flex flex-row gap-1 p-2.5 items-center {style}",
            onsubmit: move |event| {
                let title = event.values()["Title"].as_value();
                spawn_forever(create_task(board_signals, title, status, scroll_to));
                adding_task.set(false);
            },
            TextInput {
                onmounted: Some(EventHandler::new(move |e: MountedEvent| {
                    new_task.set(Some(e.clone()));
                    spawn(async move {
                        let _ = e.set_focus(true).await;
                    });
                })),
                id: "new-{status:#?}-task-title-input",
                label: "Title",
            }
            ConfirmButton { label: "add task" }
            CancelButton { label: "cancel adding task", editing: adding_task }
        }
    }
}

#[component]
fn AddTaskButton(adding_task: Signal<bool>, new_task: Signal<Option<MountedEvent>>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border-t {}", theme.border_color);
    rsx! {
        button {
            class: "
                h-12 shrink-0 grow-0 flex flex-row justify-center items-center
                {style}
            ",
            onclick: move |_| async move {
                if adding_task() {
                    let _ = new_task.unwrap().set_focus(true).await;
                } else {
                    adding_task.set(true);
                }
            },
            div {
                class: "size-6",
                CircledPlusIcon {}
            }
        }
    }
}

#[component]
fn ColumnTasks(status: TaskStatus, scroll_to: Option<TaskId>) -> Element {
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
                scroll_to: scroll_to.map(|id| id == *task_id),
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
                status,
            }
        }
    }
}

async fn create_task(
    signals: BoardSignals,
    title: String,
    status: TaskStatus,
    mut scroll_to: Signal<Option<TaskId>>,
) {
    if let Ok(task_id) = requests::create_task(
        signals.board,
        &shared_models::TaskData {
            title,
            description: String::new(),
            due: None,
            size: shared_models::TaskSize::Small,
            status,
            assignees: Vec::new(),
            tags: Vec::new(),
        },
    )
    .await
    {
        log::info!("created task: {task_id}");
        scroll_to.set(Some(task_id));
    }
    requests::board(signals).await;
}
