use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use itertools::Itertools;
use shared_models::{BoardName, TaskStatus};

use crate::{
    components::{
        form::{CancelButton, ConfirmButton},
        icons::{DoneIcon, EditIcon, InProgressIcon, SparklesIcon, StackIcon, ToDoIcon},
        input::TextInput,
        nav::NavBar,
        tooltip::Tooltip,
    },
    pages::board::{
        components::{
            AddTaskButton, DenseTask, FilterBarTagIcon, NewTaskForm, Task, ThemeButton, UserIcon,
        },
        model::{task_filter, Board, Dense, TagFilter, Tags, Tasks, UserFilter, Users},
        requests::{self, BoardSignals},
    },
    themes::Theme,
};

#[component]
pub fn ThreeColumnBoard(board_name: BoardName) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("{} {}", theme.text_color, theme.bg_color_1);
    let show_themes = use_signal(|| false);
    rsx! {
        div {
            class: "flex flex-col h-dvh w-screen {style}",
            Header {
                body: rsx!{
                    div {
                        class: "w-24"
                    }
                    Title {}
                    div {
                        class: "flex flex-row gap-2 w-24",
                        DenseButton {}
                        ToggleThemesButton { show_themes }
                    }
                }
            }
            div {
                class: "grow flex flex-col gap-2 overflow-y-auto p-4 pb-2",
                div {
                    class: "grow w-full h-full overflow-y-auto",
                    div {
                        class: "w-full h-full grid grid-cols-3 gap-2 overflow-y-auto",
                        Column { status: TaskStatus::ToDo }
                        Column { status: TaskStatus::InProgress }
                        Column { status: TaskStatus::Done }
                    },
                }
                if show_themes() {
                    ThemesBar {}
                }
                FilterBar {}
            }
            NavBar { board_name }
        }
    }
}

#[component]
fn Title() -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            TitleInput { editing }
        } else {
            TitleShow { editing }
        }
    }
}

#[component]
fn TitleInput(editing: Signal<bool>) -> Element {
    let board = use_context::<Signal<Board>>();
    let board = board.read();
    let board_signals = BoardSignals::default();
    rsx! {
        form {
            "aria-label": "update board title",
            class: "grow flex flex-row gap-2 items-center justify-center",
            onsubmit: move |event| {
                let title = event.values()["Title"].as_value();
                spawn_forever(requests::set_board_title(board_signals, title));
                editing.set(false);
            },
            TextInput {
                id: "board-title-input",
                label: "Title",
                value: board.title.clone(),
            }
            ConfirmButton { label: "set title" }
            CancelButton { label: "cancel title update", editing }
        }
    }
}

#[component]
fn TitleShow(editing: Signal<bool>) -> Element {
    let board = use_context::<Signal<Board>>();
    let board = board.read();
    rsx! {
        div {
            class: "grow flex flex-col items-center justify-center pb-1",
            div {
                class: "flex flex-row items-center justify-center gap-2",
                h1 {
                    class: "text-2xl font-extrabold",
                    {board.title.clone()}
                }
                EditTitleButton { editing }
            }
            p { "{board.board_name}" }
        }
    }
}

#[component]
fn EditTitleButton(editing: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "edit title",
                class: "block size-6",
                onclick: move |_| editing.set(true),
                EditIcon {}
            }
            Tooltip { content: "Edit Title", position: "" }
        }
    }
}

#[component]
fn ThemesBar() -> Element {
    let themes = use_context::<Signal<Vec<Theme>>>();
    rsx! {
        section {
            class: "flex flex-row gap-2 items-center",
            h2 { class: "text-xl", "Themes:" }
            div {
                class: "flex flex-row overflow-x-auto gap-2",
                for theme in themes.read().iter() {
                    ThemeButton { theme: *theme }
                }
            }
        }
    }
}

#[component]
fn FilterBar() -> Element {
    let tags = use_context::<Signal<Tags>>();
    let tags = &tags.read().0;
    let users = use_context::<Signal<Users>>();
    let users = &users.read().0;
    rsx! {
        section {
            "aria-label": "filters",
            class: "grid grid-cols-5 gap-1",
            div {
                class: "col-span-4 flex flex-row flex-wrap items-center gap-1",
                for tag_id in tags.keys().sorted_by_key(|tag_id| tags[tag_id].name.to_lowercase())
                {
                    FilterBarTagIcon {
                        tag_id: *tag_id,
                        tag_data: tags[tag_id].clone(),
                    }
                }
            }
            div {
                class: "col-span-1 flex flex-row flex-wrap items-center gap-1",
                for user_id in users.keys().sorted_by_key(|user_id| users[user_id].name.to_lowercase())
                {
                    UserIcon {
                        user_id: *user_id,
                        user_data: users[user_id].clone(),
                        size: "size-6",
                        dir: "rtl",
                        tooltip_position: "-top-11",
                    }
                }
            }
        }
    }
}

#[component]
fn DenseButton() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border-2 rounded {}", theme.button);
    let mut dense = use_context::<Signal<Dense>>();
    let mut dense_storage =
        use_synced_storage::<LocalStorage, bool>("dense".to_string(), move || false);
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "toggle dense view",
                class: "size-9 p-1 {style}",
                "aria-pressed": dense.read().0,
                onclick: move |_| {
                    let new_dense = !dense.read().0;
                    dense.set(Dense(new_dense));
                    dense_storage.set(new_dense);
                },
                StackIcon {}
            }
            Tooltip { content: "Toggle Dense View", position: "-left-10" }
        }
    }
}

#[component]
fn ToggleThemesButton(show_themes: Signal<bool>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border-2 rounded {}", theme.button);
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "toggle show themes",
                class: "size-9 p-1 {style}",
                "aria-pressed": show_themes(),
                onclick: move |_| {
                    show_themes.set(!show_themes());
                },
                SparklesIcon {}
            }
            Tooltip { content: "Toggle Show Themes", position: "-left-14" }
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
                flex flex-row items-center
                w-full h-14 shrink-0 grow-0
                px-24
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
fn Column(status: TaskStatus) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border {}", theme.border_color);
    let dense = use_context::<Signal<Dense>>().read().0;
    let gap = if dense { "" } else { "gap-2" };
    let adding_task = use_signal(|| false);
    rsx! {
        section {
            class: "flex flex-col overflow-y-auto px-2 pt-2 gap-2 {style}",
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
                    ColumnTasks { status }
                }
                if adding_task() {
                    NewTaskForm { status, adding_task }
                }
            }
            AddTaskButton { status, adding_task }
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
