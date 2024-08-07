use crate::{components::icons::UpIcon, datetime};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use shared_models::{Color, TagData, TagId, TaskEntry, TaskId, TaskStatus, UserData, UserId};

use crate::{
    commands::ScrollTarget,
    components::{
        icons::{
            CalendarIcon, DoneIcon, DownIcon, InProgressIcon, ToDoIcon, TrashIcon, UnarchiveIcon,
        },
        tooltip::Tooltip,
    },
    pages::archive::{
        model::{BoardUrl, Tags, TaskEntries, Users},
        requests,
    },
    themes::Theme,
};

#[component]
pub fn TaskList() -> Element {
    let tasks = use_context::<Signal<TaskEntries>>();
    rsx! {
        div {
            class: "
                overflow-y-auto w-full max-w-lg
                flex flex-col sm:gap-2
            ",
            for task in tasks.read().0.iter() {
                Task { task: task.clone() }
            }
        }
    }
}

#[component]
fn Task(task: TaskEntry) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let label = task.title.clone();
    let expanded = use_signal(|| false);
    let style = format!(
        "
        first:border-t
        border-b
        sm:border
        sm:rounded-lg
        sm:shadow
        {} {}
        ",
        theme.border_color, theme.bg_color_2,
    );
    rsx! {
        article {
            id: "task-{task.id}-article",
            "aria-label": label,
            class: "flex flex-col gap-2 p-2.5 {style}",
            div {
                class: "flex flex-row justify-between",
                div {
                    class: "flex flex-row items-center gap-1",
                    ToggleExpanded { task_id: task.id, expanded, size: "size-7" }
                    Title { title: task.title }
                }
                section {
                    "aria-label": "task status",
                    match task.status {
                        TaskStatus::ToDo => rsx! {
                            div {
                                class: "group relative",
                                div { class: "size-8 stroke-red-600", ToDoIcon {} }
                                Tooltip {
                                    content: "To Do",
                                    position: "",
                                    dir: "rtl"
                                }
                            }
                        },
                        TaskStatus::InProgress => rsx! {
                            div {
                                class: "group relative",
                                div { class: "size-8 stroke-fuchsia-600", InProgressIcon {} }
                                Tooltip {
                                    content: "In Progress",
                                    position: "",
                                    dir: "rtl"
                                }
                            }
                        },
                        TaskStatus::Done => rsx! {
                            div {
                                class: "group relative",
                                div { class: "size-8 stroke-green-500", DoneIcon {} }
                                Tooltip {
                                    content: "Done",
                                    position: "",
                                    dir: "rtl"
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "flex flex-row justify-between items-center",
                Assignees { task_id: task.id, assignees: task.assignees.clone() }
                TaskActions { task_id: task.id }
            }
            TaskTags { task_id: task.id, tags: task.tags.clone() }
            if task.due.is_some() {
                ShowDue { due: task.due }
            }
            if expanded() {
                Description { description: task.description }
                SpecialActions { task_id: task.id }
            }
        }
    }
}

#[component]
fn ToggleExpanded(task_id: TaskId, expanded: Signal<bool>, size: &'static str) -> Element {
    let mut scroll_target = use_context::<Signal<ScrollTarget>>();
    let style = "rounded";
    let expanded_ = expanded();
    rsx! {
        button {
            "aria-label": "toggle expand task",
            "aria-pressed": expanded(),
            class: "{size} p-1 {style}",
            onclick: move |_| {
                if !expanded() {
                    scroll_target.set(ScrollTarget(Some(format!("task-{task_id}-article"))));
                }
                expanded.set(!expanded_);
             },
            if expanded_ {
                UpIcon {}
            } else {
                DownIcon {}
            }
        }
    }
}

#[component]
fn Title(title: String) -> Element {
    rsx! {
        h3 {
            class: "
                text-lg sm:text-xl
                font-bold tracking-tight
            ",
            {title}
        }
    }
}

#[component]
fn Assignees(
    task_id: TaskId,
    assignees: Vec<UserId>,
    icon_size: Option<&'static str>,
    tooltip_position: Option<&'static str>,
    dir: Option<&'static str>,
) -> Element {
    let users = use_context::<Signal<Users>>();
    let users = &users.read().0;
    let size = icon_size.unwrap_or("size-6");
    rsx! {
        section {
            id: "task-{task_id}-assignees",
            "aria-label": "assignees",
            class: "flex flex-row flex-wrap items-center gap-2",
            for user_id in assignees {
                UserIcon {
                    user_id,
                    user_data: users[&user_id].clone(),
                    size,
                    tooltip_position,
                    dir
                }
            }
        }
    }
}

#[component]
pub fn UserIcon(
    user_id: UserId,
    user_data: UserData,
    size: &'static str,
    tooltip_position: Option<&'static str>,
    dir: Option<&'static str>,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let color = match user_data.color {
        Color::Black => theme.color1_button,
        Color::White => theme.color2_button,
        Color::Gray => theme.color3_button,
        Color::Silver => theme.color4_button,
        Color::Maroon => theme.color5_button,
        Color::Red => theme.color6_button,
        Color::Purple => theme.color7_button,
        Color::Fushsia => theme.color8_button,
        Color::Green => theme.color9_button,
        Color::Lime => theme.color10_button,
        Color::Olive => theme.color11_button,
        Color::Yellow => theme.color12_button,
        Color::Navy => theme.color13_button,
        Color::Blue => theme.color14_button,
        Color::Teal => theme.color15_button,
        Color::Aqua => theme.color16_button,
    };
    let style = "
        rounded border-2
        sm:hover:border-4 active:border-4 sm:hover:scale-110 active:scale-110
    ";
    rsx! {
        div {
            class: "group relative",
            div {
                class: "block {size} {style} {color}",
                "aria-label": user_data.name,
                div { class: "size-full" }
            }
            Tooltip {
                content: user_data.name.clone(),
                position: tooltip_position,
                dir,
            }
        }
    }
}

#[component]
fn TaskActions(task_id: TaskId) -> Element {
    let url = use_context::<Signal<BoardUrl>>();
    let tasks = use_context::<Signal<TaskEntries>>();
    rsx! {
        section {
            "aria-label": "task actions",
            class: "flex flex-row gap-1",
            ActionButton {
                onclick: move |_| {
                    spawn_forever(requests::set_task_archived(tasks, url, task_id));
                },
                tooltip: "Restore Task",
                body: rsx!(UnarchiveIcon {}),
            }
        }
    }
}

#[component]
fn ActionButton(tooltip: String, body: Element, onclick: EventHandler<MouseEvent>) -> Element {
    let style = "sm:hover:stroke-blue-500 active:stroke-blue-500";
    let aria_label = tooltip.clone();
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": aria_label,
                class: "block size-6 {style}",
                onclick: move |event| onclick.call(event),
                {body}
            }
            Tooltip { position: "", content: tooltip, dir: "rtl" }
        }
    }
}

#[component]
fn TaskTags(task_id: TaskId, tags: Vec<TagId>) -> Element {
    let tag_data = use_context::<Signal<Tags>>();
    let tag_data = &tag_data.read().0;
    rsx! {
        section {
            id: "task-{task_id}-tags",
            "aria-label": "tags",
            class: "flex flex-row flex-wrap gap-2 items-center",
            for tag_id in tags {
                TaskTagIcon { task_id, tag_id, tag_data: tag_data[&tag_id].clone() }
            }
        }
    }
}

#[component]
pub fn TaskTagIcon(task_id: TaskId, tag_id: TagId, tag_data: TagData) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let color = match tag_data.color {
        Color::Black => theme.color1_button,
        Color::White => theme.color2_button,
        Color::Gray => theme.color3_button,
        Color::Silver => theme.color4_button,
        Color::Maroon => theme.color5_button,
        Color::Red => theme.color6_button,
        Color::Purple => theme.color7_button,
        Color::Fushsia => theme.color8_button,
        Color::Green => theme.color9_button,
        Color::Lime => theme.color10_button,
        Color::Olive => theme.color11_button,
        Color::Yellow => theme.color12_button,
        Color::Navy => theme.color13_button,
        Color::Blue => theme.color14_button,
        Color::Teal => theme.color15_button,
        Color::Aqua => theme.color16_button,
    };
    let style = "rounded border-2";
    rsx! {
        div {
            class: "
                group
                flex flex-row items-center
                px-1.5 py-0.5
                {style} {color}
            ",
            div {
                class: "text-sm pr-1",
                "aria-label": tag_data.name,
                "# {tag_data.name}"
            }
        }
    }
}

#[component]
fn Description(description: String) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "p-4 rounded border whitespace-pre-wrap break-words {} {}",
        theme.bg_color_1, theme.border_color
    );
    rsx! {
        section {
            "aria-label": "description",
            class: "flex flex-col gap-1",
            p { class: style, {description} }
        }
    }
}

#[component]
fn SpecialActions(task_id: TaskId) -> Element {
    rsx! {
        section {
            "aria-label": "special actions",
            class: "grid grid-rows-1 justify-items-end",
            DeleteTaskButton { task_id }
        }
    }
}

#[component]
fn DeleteTaskButton(task_id: TaskId) -> Element {
    let url = use_context::<Signal<BoardUrl>>();
    let tasks = use_context::<Signal<TaskEntries>>();
    let style = "stroke-red-600";
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "delete task",
                class: "block size-6 {style}",
                onclick: move |_| {
                    spawn_forever(requests::delete_task(tasks, url, task_id));
                },
                TrashIcon {}
            }
            Tooltip { content: "Delete Task", position: "-top-10 -left-20" }
        }
    }
}

#[component]
fn ShowDue(due: Option<DateTime<Utc>>) -> Element {
    rsx! {
        section {
            "aria-label": "due date",
            class: "flex flex-row gap-2 items-center",
            div { class: "size-8", CalendarIcon {} }
            if let Some(due_value) = due {
                p {
                    "{datetime::format(datetime::utc_to_local(&due_value))}"
                }
            }
        }
    }
}
