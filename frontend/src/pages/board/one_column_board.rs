use dioxus::prelude::*;
use shared_models::{BoardName, TaskStatus};

use crate::{
    components::{
        icons::{BarsIcon, DoneIcon, ElipsisHorizontalIcon, InProgressIcon, ToDoIcon},
        nav::NavBar,
    },
    pages::board::{
        components::Task,
        model::{task_filter, Board, TagFilter, Tasks, UserFilter},
    },
    themes::Theme,
};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Drawer {
    None,
    Actions,
    Navigation,
}

#[component]
pub fn OneColumnBoard(board_name: BoardName) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("{} {}", theme.text_color, theme.bg_color_1);
    let status = use_signal(|| TaskStatus::ToDo);
    let drawer = use_signal(|| Drawer::None);
    let column_label = match status() {
        TaskStatus::ToDo => "To Do",
        TaskStatus::InProgress => "In Progress",
        TaskStatus::Done => "Done",
    };
    rsx! {
        div {
            class: "flex flex-col h-dvh w-screen {style}",
            Header {
                body: rsx! {
                    ToggleNavDrawerButton { drawer }
                    h1 {
                        class: "font-extrabold",
                        "{board_name}"
                    }
                    ToggleActionsDrawerButton { drawer }
                }
            }
            section {
                class: "grow flex flex-col overflow-y-auto",
                "aria-label": "{column_label} tasks",
                div {
                    class: "
                        w-full shrink-0 grow-0
                        flex flex-row items-center justify-center
                        pb-1
                    ",
                    ColumnSwitcher { status }
                }
                Column { status: status() }
            }
            NavBar { board_name }
        }
    }
}

#[component]
fn Header(body: Element) -> Element {
    rsx! {
        header {
            class: "
                flex flex-row items-center justify-between
                w-full h-10 shrink-0 grow-0 px-2
            ",
            {body}
        }
    }
}

#[component]
fn Column(status: TaskStatus) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("divide-y {}", theme.divide_color);
    rsx! {
        div {
            class: "
                grow flex flex-col overflow-y-auto
                {style}
            ",
            ColumnTasks { status }
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
                status,
            }
        }
    }
}

#[component]
fn ToggleNavDrawerButton(drawer: Signal<Drawer>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border rounded {}", theme.button);
    rsx! {
        button {
            class: "size-6 p-1 {style}",
            "aria-pressed": drawer() == Drawer::Navigation,
            onclick: move |_| {
                if drawer() == Drawer::Navigation {
                    drawer.set(Drawer::None)
                } else {
                    drawer.set(Drawer::Navigation)
                }
            },
            BarsIcon {}
        }
    }
}

#[component]
fn ToggleActionsDrawerButton(drawer: Signal<Drawer>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border rounded {}", theme.button);
    rsx! {
        button {
            class: "size-6 p-1 {style}",
            "aria-pressed": drawer() == Drawer::Actions,
            onclick: move |_| {
                if drawer() == Drawer::Actions {
                    drawer.set(Drawer::None)
                } else {
                    drawer.set(Drawer::Actions)
                }
            },
            ElipsisHorizontalIcon {}
        }
    }
}

#[component]
fn ColumnSwitcher(status: Signal<TaskStatus>) -> Element {
    let style = "border rounded";
    rsx! {
        button {
            class: "
                py-0.5 px-1
                flex flex-row gap-1 items-center
                text-xs
                {style}
            ",
            match status() {
                TaskStatus::ToDo => rsx! {
                    div { class: "size-3", ToDoIcon {} }
                    "To Do"
                },
                TaskStatus::InProgress => rsx! {
                    div { class: "size-3", InProgressIcon {} }
                    "In Progress"
                },
                TaskStatus::Done => rsx! {
                    div { class: "size-3", DoneIcon {} }
                    "Done"
                }
            }
        }
    }
}
