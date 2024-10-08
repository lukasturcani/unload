use crate::{
    components::icons::UpIcon,
    datetime,
    description_parser::{parse_blocks, Block, Line},
};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_sdk::{i18n::use_i18, translate};
use shared_models::{Color, TagId, TaskId, TaskStatus, UserId};

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
                Task {
                    key: "{task.id}",
                    task_id: task.id,
                    title: task.title.clone(),
                    description: task.description.clone(),
                    status: task.status,
                    assignees: task.assignees.clone(),
                    tags: task.tags.clone(),
                    due: task.due,
                }
            }
        }
    }
}

#[component]
fn Task(
    task_id: TaskId,
    title: ReadOnlySignal<String>,
    description: ReadOnlySignal<String>,
    status: TaskStatus,
    assignees: ReadOnlySignal<Vec<UserId>>,
    tags: ReadOnlySignal<Vec<TagId>>,
    due: Option<DateTime<Utc>>,
) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
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
            id: "task-{task_id}-article",
            aria_label: title,
            class: "flex flex-col gap-2 p-2.5 {style}",
            div {
                class: "flex flex-row justify-between",
                div {
                    class: "flex flex-row items-center gap-1",
                    ToggleExpanded { task_id, expanded, size: "size-7" }
                    Title { title }
                }
                section {
                    aria_label: translate!(i18, "task_status_section_label"),
                    match status {
                        TaskStatus::ToDo => rsx! {
                            div {
                                class: "group relative",
                                div { class: "size-8 stroke-red-600", ToDoIcon {} }
                                Tooltip {
                                    content: translate!(i18, "to_do_button_tooltip"),
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
                                    content: translate!(i18, "in_progress_button_tooltip"),
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
                                    content: translate!(i18, "done_button_tooltip"),
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
                Assignees { task_id,  assignees }
                TaskActions { task_id }
            }
            TaskTags { task_id, tags }
            if due.is_some() {
                ShowDue { due }
            }
            if expanded() {
                Description { description }
                AdditionalActions { task_id }
            }
        }
    }
}

#[component]
fn ToggleExpanded(task_id: TaskId, expanded: Signal<bool>, size: &'static str) -> Element {
    let i18 = use_i18();
    let mut scroll_target = use_context::<Signal<ScrollTarget>>();
    let style = "rounded";
    let expanded_ = expanded();
    rsx! {
        button {
            aria_label: translate!(i18, "toggle_expand_task_button_label"),
            aria_pressed: expanded(),
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
fn Title(title: ReadOnlySignal<String>) -> Element {
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
    assignees: ReadOnlySignal<Vec<UserId>>,
    icon_size: Option<&'static str>,
    tooltip_position: Option<&'static str>,
    dir: Option<&'static str>,
) -> Element {
    let i18 = use_i18();
    let users = use_context::<Signal<Users>>();
    let users = &users.read().0;
    let size = icon_size.unwrap_or("size-6");
    rsx! {
        section {
            id: "task-{task_id}-assignees",
            aria_label: translate!(i18, "assignees_section_label"),
            class: "flex flex-row flex-wrap items-center gap-2",
            for (&user_id, user) in assignees
                .read()
                .iter()
                .map(|id| (id, &users[id]))
            {
                UserIcon {
                    user_id,
                    name: user.name.clone(),
                    color: user.color,
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
    name: ReadOnlySignal<String>,
    color: Color,
    size: &'static str,
    tooltip_position: Option<&'static str>,
    dir: Option<&'static str>,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let color = match color {
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
                aria_label: name,
                div { class: "size-full" }
            }
            Tooltip {
                content: name,
                position: tooltip_position,
                dir,
            }
        }
    }
}

#[component]
fn TaskActions(task_id: TaskId) -> Element {
    let i18 = use_i18();
    let url = use_context::<Signal<BoardUrl>>();
    let tasks = use_context::<Signal<TaskEntries>>();
    rsx! {
        section {
            aria_label: translate!(i18, "task_actions_section_label"),
            class: "flex flex-row gap-1",
            ActionButton {
                onclick: move |_| {
                    spawn_forever(requests::set_task_archived(tasks, url, task_id));
                },
                tooltip: translate!(i18, "unarchive_task_button_tooltip"),
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
                aria_label,
                class: "block size-6 {style}",
                onclick: move |event| onclick.call(event),
                {body}
            }
            Tooltip { position: "", content: tooltip, dir: "rtl" }
        }
    }
}

#[component]
fn TaskTags(task_id: TaskId, tags: ReadOnlySignal<Vec<TagId>>) -> Element {
    let i18 = use_i18();
    let tag_data = use_context::<Signal<Tags>>();
    let tag_data = &tag_data.read().0;
    rsx! {
        section {
            id: "task-{task_id}-tags",
            aria_label: translate!(i18, "tags_section_label"),
            class: "flex flex-row flex-wrap gap-2 items-center",
            for (&tag_id, tag) in tags
                .read()
                .iter()
                .map(|id| (id, &tag_data[id]))
            {
                TaskTagIcon {
                    task_id,
                    tag_id,
                    name: tag.name.clone(),
                    color: tag.color,
                }
            }
        }
    }
}

#[component]
pub fn TaskTagIcon(
    task_id: TaskId,
    tag_id: TagId,
    name: ReadOnlySignal<String>,
    color: Color,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let color = match color {
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
                aria_label: name,
                "# {name}"
            }
        }
    }
}

#[component]
fn Description(description: ReadOnlySignal<String>) -> Element {
    let i18 = use_i18();
    rsx! {
        section {
            aria_label: translate!(i18, "description_section_label"),
            class: "flex flex-col gap-1",
            DescriptionContent { description }
        }
    }
}

#[component]
fn DescriptionContent(description: ReadOnlySignal<String>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "p-4 rounded border whitespace-pre-wrap break-words {} {}",
        theme.bg_color_1, theme.border_color
    );
    rsx! {
        div {
            class: style,
            for block in parse_blocks(&description.read()) {
                match block {
                    Block::Text(text) => rsx!{
                        p { {text} }
                    },
                    Block::Bullet(lines) => rsx!{
                        ul {
                            class:" list-disc list-inside",
                            for line in lines {
                                Bullet { line }
                            }
                        }
                    },
                    Block::Checkbox(lines) => rsx!{
                        ul {
                            for line in lines {
                                Checkbox { line }
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn Checkbox(line: Line) -> Element {
    let (head, tail) = line.content.split_once(']').unwrap();
    rsx! {
        li {
            label {
                input {
                    disabled: true,
                    checked: head.ends_with('x'),
                    r#type: "checkbox",
                }
                {tail}
            }
        }
    }
}

#[component]
fn Bullet(line: String) -> Element {
    line.drain(..2);
    rsx! { li { {line} } }
}

#[component]
fn AdditionalActions(task_id: TaskId) -> Element {
    let i18 = use_i18();
    rsx! {
        section {
            aria_label: translate!(i18, "additional_actions_section_label"),
            class: "grid grid-rows-1 justify-items-end",
            DeleteTaskButton { task_id }
        }
    }
}

#[component]
fn DeleteTaskButton(task_id: TaskId) -> Element {
    let i18 = use_i18();
    let url = use_context::<Signal<BoardUrl>>();
    let tasks = use_context::<Signal<TaskEntries>>();
    let style = "stroke-red-600";
    let aria_label = translate!(i18, "delete_task_tooltip");
    rsx! {
        div {
            class: "group relative",
            button {
                aria_label,
                class: "block size-6 {style}",
                onclick: move |_| {
                    spawn_forever(requests::delete_task(tasks, url, task_id));
                },
                TrashIcon {}
            }
            Tooltip {
                content: aria_label.clone(),
                position: "-top-10 -left-20",
            }
        }
    }
}

#[component]
fn ShowDue(due: Option<DateTime<Utc>>) -> Element {
    let i18 = use_i18();
    rsx! {
        section {
            aria_label: translate!(i18, "due_date_section_label"),
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
