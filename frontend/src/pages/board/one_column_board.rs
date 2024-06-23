use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use itertools::Itertools;
use shared_models::{BoardName, TaskStatus};

use crate::{
    components::{
        icons::{
            BarsIcon, CancelIcon, DoneIcon, ElipsisHorizontalIcon, FilterIcon, InProgressIcon,
            SparklesIcon, StackIcon, ToDoIcon,
        },
        nav::NavBar,
    },
    pages::board::{
        components::{
            AddTaskButton, DenseTask, FilterBarTagIcon, NewTaskForm, Task, ThemeButton, UserIcon,
        },
        model::{task_filter, Board, TagFilter, Tags, Tasks, UserFilter, Users},
    },
    themes::Theme,
};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Panel {
    None,
    Actions,
    Navigation,
    Status,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ExtraBar {
    None,
    Filter,
    Themes,
}

#[component]
pub fn OneColumnBoard(board_name: BoardName) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("{} {}", theme.text_color, theme.bg_color_1);
    let status = use_signal(|| TaskStatus::ToDo);
    let status_ = status();
    let mut panel = use_signal(|| Panel::None);
    let column_label = match status_ {
        TaskStatus::ToDo => "To Do",
        TaskStatus::InProgress => "In Progress",
        TaskStatus::Done => "Done",
    };
    let extra_bar = use_signal(|| ExtraBar::None);
    let adding_task = use_signal(|| false);
    rsx! {
        div {
            onclick: move |_| panel.set(Panel::None),
            class: "flex flex-col h-dvh w-screen {style}",
            Header {
                body: rsx! {
                    ToggleNavDrawerButton { panel }
                    h1 {
                        class: "font-extrabold",
                        "{board_name}"
                    }
                    div {
                        class: "flex flex-row gap-2 items-center",
                        ToggleFiltersButton { extra_bar }
                        ToggleActionsDrawerButton { panel }
                    }
                }
            }
            section {
                class: "grow flex flex-col overflow-y-auto",
                "aria-label": "{column_label} tasks",
                div {
                    class: "
                        w-full shrink-0 grow-0
                        flex flex-row items-center
                        pb-1 px-2
                    ",
                    ColumnSwitcher { status, panel }
                }
                Column { status: status_, adding_task }
            }
            AddTaskButton { status: status_, adding_task }
            match extra_bar() {
                ExtraBar::Filter => rsx! { FilterBar { extra_bar } },
                ExtraBar::Themes => rsx! { ThemesBar { extra_bar } },
                ExtraBar::None => rsx! {},
            }
            NavBar { board_name }
        }
        match panel() {
            Panel::Actions => rsx! { ActionsSheet { panel, extra_bar } },
            _ => rsx! {},
        }
    }
}

#[component]
fn BottomSheet(panel: Signal<Panel>, body: Element) -> Element {
    rsx! {
        div {
            class: "
                size-full absolute inset-0 z-10
                flex flex-col
            ",
            div {
                class: "grow backdrop-blur-sm",
                onclick: move |_| panel.set(Panel::None),
            }
            {body}
        }
    }
}

#[component]
fn ActionsSheet(panel: Signal<Panel>, extra_bar: Signal<ExtraBar>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
                rounded-t-2xl text-lg border-t
                {} {} {}
            ",
        theme.bg_color_1, theme.text_color, theme.border_color
    );
    let mut dense = use_synced_storage::<LocalStorage, bool>("dense".to_string(), move || false);
    rsx! {
        BottomSheet {
            panel
            body: rsx! {
                section {
                    "aria-label": "actions",
                    class: "flex flex-col gap-2 pt-2 pb-20 {style}",
                    button {
                        class: "flex flex-row gap-2 items-center justify-left px-1",
                        onclick: move |_| {
                            panel.set(Panel::None);
                            dense.set(!dense());
                        },
                        div { class: "size-5", StackIcon {} }
                        "Toggle dense view"
                    }
                    button {
                        class: "flex flex-row gap-2 items-center justify-left px-1",
                        onclick: move |_| {
                            extra_bar.set(ExtraBar::Themes);
                            panel.set(Panel::None);
                        },
                        div { class: "size-5", SparklesIcon {} }
                        "Change theme"
                    }
                }
            },
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
fn Column(status: TaskStatus, adding_task: Signal<bool>) -> Element {
    let dense = use_synced_storage::<LocalStorage, bool>("dense".to_string(), move || false);
    rsx! {
        div {
            class: "grow flex flex-col overflow-y-auto",
            if dense() {
                DenseColumnTasks { status }
            } else {
                ColumnTasks { status }
            }
            if adding_task() {
                NewTaskForm { status, adding_task }
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

#[component]
fn ToggleNavDrawerButton(panel: Signal<Panel>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border rounded {}", theme.button);
    rsx! {
        button {
            class: "size-7 p-1 {style}",
            "aria-pressed": panel() == Panel::Navigation,
            onclick: move |event| {
                event.stop_propagation();
                if panel() == Panel::Navigation {
                    panel.set(Panel::None)
                } else {
                    panel.set(Panel::Navigation)
                }
            },
            BarsIcon {}
        }
    }
}

#[component]
fn ToggleActionsDrawerButton(panel: Signal<Panel>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border rounded {}", theme.button);
    rsx! {
        button {
            "aria-label": "toggle actions drawer",
            class: "size-7 p-1 {style}",
            "aria-pressed": panel() == Panel::Actions,
            onclick: move |event| {
                event.stop_propagation();
                if panel() == Panel::Actions {
                    panel.set(Panel::None)
                } else {
                    panel.set(Panel::Actions)
                }
            },
            ElipsisHorizontalIcon {}
        }
    }
}

#[component]
fn ToggleFiltersButton(extra_bar: Signal<ExtraBar>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border rounded {}", theme.button);
    rsx! {
        button {
            "aria-label": "toggle show filters",
            class: "size-7 p-1 {style}",
            "aria-pressed": extra_bar() == ExtraBar::Filter,
            onclick: move |event| {
                event.stop_propagation();
                if extra_bar() == ExtraBar::Filter {
                    extra_bar.set(ExtraBar::None)
                } else {
                    extra_bar.set(ExtraBar::Filter)
                }
            },
            FilterIcon {}
        }
    }
}

#[component]
fn ColumnSwitcher(status: Signal<TaskStatus>, panel: Signal<Panel>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let status_style = "border rounded";
    let dropdown_style = format!(
        "
            border divide-y
            rounded-lg shadow-sm
            {} {} {}
        ",
        theme.border_color, theme.divide_color, theme.bg_color_2,
    );
    rsx! {
        div {
            class: "group relative",
            button {
                class: "
                    py-0.5 px-1
                    flex flex-row gap-1 items-center
                    text-sm
                    {status_style}
                ",
                onclick: move |event| {
                    if panel() == Panel::Status {
                        panel.set(Panel::None);
                    } else {
                        panel.set(Panel::Status);
                    }
                    event.stop_propagation();
                },
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
            if panel() == Panel::Status {
                div {
                    class: "
                        absolute -bottom-24
                        z-10
                        flex flex-col
                        text-lg
                        {dropdown_style}
                    ",
                    button {
                        class: "flex flex-row gap-1 items-center text-nowrap px-1",
                        onclick: move |event| {
                            status.set(TaskStatus::ToDo);
                            panel.set(Panel::None);
                            event.stop_propagation();
                        },
                        div { class: "size-5", ToDoIcon {} }
                        "To Do",
                    }
                    button {
                        class: "flex flex-row gap-1 items-center text-nowrap px-1",
                        onclick: move |event| {
                            status.set(TaskStatus::InProgress);
                            panel.set(Panel::None);
                            event.stop_propagation();
                        },
                        div { class: "size-5", InProgressIcon {} }
                        "In Progress",
                    }
                    button {
                        class: "flex flex-row gap-1 items-center text-nowrap px-1",
                        onclick: move |event| {
                            status.set(TaskStatus::Done);
                            panel.set(Panel::None);
                            event.stop_propagation();
                        },
                        div { class: "size-5", DoneIcon {} }
                        "Done",
                    }
                }
            }
        }
    }
}

#[component]
fn ThemesBar(extra_bar: Signal<ExtraBar>) -> Element {
    let themes = use_context::<Signal<Vec<Theme>>>();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "border-t divide-x-2 {} {}",
        theme.border_color, theme.divide_color
    );
    rsx! {
        section {
            class: "
                flex flex-row gap-2 items-center justify-between
                py-2
                {style}
            ",
            "aria-label": "themes",
            div {
                class: "flex flex-row overflow-x-auto gap-2",
                for theme in themes.read().iter() {
                    ThemeButton { theme: *theme }
                }
            }
            button {
                "aria-label": "close theme selector",
                class: "size-6 shrink-0",
                onclick: move |_| extra_bar.set(ExtraBar::None),
                CancelIcon {}
            }
        }
    }
}

#[component]
fn FilterBar(extra_bar: Signal<ExtraBar>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border-t {}", theme.border_color);
    let tags = use_context::<Signal<Tags>>();
    let tags = &tags.read().0;
    let users = use_context::<Signal<Users>>();
    let users = &users.read().0;
    rsx! {
        section {
            "aria-label": "filters",
            class: "flex flex-col gap-1 px-2 py-1 {style}",
            div {
                class: "flex flex-row px-1 justify-end",
                button {
                    "aria-label": "close filters",
                    class: "size-6",
                    onclick: move |_| extra_bar.set(ExtraBar::None),
                    CancelIcon {}
                }
            }
            div {
                class: "flex flex-row gap-1 flex-wrap items-center justify-center",
                for tag_id in tags.keys().sorted_by_key(|tag_id| tags[tag_id].name.to_lowercase())
                {
                    FilterBarTagIcon {
                        tag_id: *tag_id,
                        tag_data: tags[tag_id].clone(),
                    }
                }
            }
            div {
                class: "flex flex-row gap-1 flex-wrap items-center justify-center",
                for user_id in users.keys().sorted_by_key(|user_id| users[user_id].name.to_lowercase())
                {
                    UserIcon {
                        user_id: *user_id,
                        user_data: users[user_id].clone(),
                        size: "size-6",
                        tooltip_position: "-left-3 -top-10",
                    }
                }
            }

        }
    }
}
