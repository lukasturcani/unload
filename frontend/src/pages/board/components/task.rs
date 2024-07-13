use dioxus::prelude::*;
use reqwest::Client;
use shared_models::{Color, TagData, TagId, TaskId, TaskStatus, UserData, UserId};

use crate::{
    commands::ScrollTarget,
    components::{
        color_picker::ColorPicker,
        form::{CancelButton, ConfirmButton},
        icons::{
            ArchiveIcon, CancelIcon, CopyIcon, DoneIcon, DownIcon, EditIcon, InProgressIcon,
            PlusIcon, ToDoIcon, TrashIcon, UpIcon,
        },
        input::TextInput,
        tooltip::Tooltip,
    },
    model::UnloadUrl,
    pages::board::{
        components::{
            task::{
                due::{Due, DueOptions},
                title::{DenseTitle, Title},
            },
            TaskTagIcon, UserIcon,
        },
        model::{Board, Tags, TaskData, Users},
        requests::{self, BoardSignals},
    },
    themes::Theme,
};

mod due;
mod title;

fn is_late(task: &TaskData) -> bool {
    task.due.map_or(false, |due| due < chrono::Utc::now())
}

#[component]
pub fn Task(task_id: TaskId, task: TaskData, status: TaskStatus) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let is_late = is_late(&task);
    let style = format!(
        "
        first:border-t
        border-b
        sm:rounded-lg
        sm:shadow
        {} {} {}
        ",
        if is_late && status != TaskStatus::Done {
            theme.late_border_color
        } else {
            theme.border_color
        },
        if is_late { "sm:border-2" } else { "sm:border" },
        theme.bg_color_2,
    );
    let expanded = use_signal(|| false);
    let expanded_ = expanded();
    let select_assignees = use_signal(|| false);
    let select_tags = use_signal(|| false);
    let label = task.title.clone();
    rsx! {
        article {
            id: "task-{task_id}-article",
            "aria-label": label,
            class: "flex flex-col gap-2 p-2.5 {style}",
            div {
                class: "flex flex-row justify-between items-center",
                div {
                    class: "flex flex-row items-center gap-1",
                    ToggleExpanded { task_id, expanded, size: "size-7" }
                    Title { task_id, title: task.title }
                }
                StatusButtons { task_id }
            }
            div {
                class: "flex flex-row justify-between items-center",
                Assignees { task_id, assignees: task.assignees.clone(), select_assignees }
                TaskActions { task_id }
            }
            if select_assignees() {
                AssigneeSelection { task_id, assignees: task.assignees }
            }
            TaskTags { task_id, tags: task.tags.clone(), select_tags }
            if select_tags() {
                TagSelection { task_id, tags: task.tags }
            }
            if expanded_ || (is_late && status != TaskStatus::Done) {
                Due {
                    task_id,
                    due: task.due.map(|due| DueOptions {
                        due,
                        show_time_left: status != TaskStatus::Done,
                        is_late,
                    })
                }
            }
            if expanded() {
                Description { task_id, description: task.description }
                SpecialActions { task_id }
            }
        }
    }
}

#[component]
pub fn DenseTask(task_id: TaskId, task: TaskData, status: TaskStatus) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        first:border-t
        border-b
        {}
        ",
        theme.border_color
    );
    let expanded = use_signal(|| false);
    let expanded_ = expanded();
    let select_tags = use_signal(|| false);
    let select_assignees = use_signal(|| false);
    let label = task.title.clone();
    let is_late = is_late(&task);
    rsx! {
        article {
            id: "task-{task_id}-article",
            "aria-label": label,
            class: "flex flex-col gap-2 p-1 {style}",
            div {
                class: "flex flex-row justify-between",
                div {
                    class: "flex flex-row items-center gap-1",
                    ToggleExpanded { task_id, expanded, size: "size-5" }
                    DenseTitle { task_id, title: task.title, is_late, expanded: expanded_ }
                }
                Assignees {
                    task_id,
                    assignees: task.assignees.clone(),
                    select_assignees,
                    icon_size: if expanded_ { "size-6" } else { "size-5" },
                    tooltip_position: "",
                    dir: "rtl",
                }
            }
            if select_assignees() {
                AssigneeSelection { task_id, assignees: task.assignees }
            }
            if expanded_ {
                div {
                    class: "flex flex-row justify-center items-center",
                    StatusButtons { task_id }
                }
                Description { task_id, description: task.description }
                Due {
                    task_id,
                    due: task.due.map(|due| DueOptions {
                        due,
                        show_time_left: status != TaskStatus::Done,
                        is_late,
                    })
                }
                div {
                    class: "flex flex row justify-center items-center",
                    TaskActions { task_id }
                }
                TaskTags { task_id, tags: task.tags.clone(), select_tags }
                if select_tags() {
                    TagSelection { task_id, tags: task.tags }
                }
                SpecialActions { task_id }
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
            class: "shrink-0 {size} p-1 {style}",
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
fn Description(task_id: TaskId, description: String) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            DescriptionInput { task_id, editing, description }
        } else {
            DescriptionShow { task_id, editing, description }
        }
    }
}

#[component]
fn DescriptionInput(task_id: TaskId, editing: Signal<bool>, description: String) -> Element {
    let board_signals = BoardSignals::default();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "rounded-lg border {} {} {}",
        theme.bg_color_2, theme.border_color, theme.focus_color
    );
    rsx! {
        form {
            "aria-label": "update description",
            class: "flex flex-col gap-2",
            onsubmit: move |event| {
                let description = event.values()["Description"].as_value();
                spawn_forever(set_task_description(board_signals, task_id, description));
                editing.set(false);
            },
            textarea {
                id: "task-{task_id}-description-input",
                rows: 8.max(description.lines().count() as i64),
                class: "p-2.5 {style}",
                name: "Description",
                required: false,
                value: description,
            }
            div {
                class: "flex flex-row gap-2 items-center justify-center",
                ConfirmButton { label: "set description" }
                CancelButton { label: "cancel description update", editing }
            }
        }
    }
}

#[component]
fn DescriptionShow(task_id: TaskId, description: String, editing: Signal<bool>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "p-4 rounded border whitespace-pre-wrap break-words {} {}",
        theme.bg_color_1, theme.border_color
    );
    let edit_button_style = "rounded border";
    rsx! {
        section {
            "aria-label": "description",
            class: "flex flex-col gap-1",
            p { class: style, {description} }
            div {
                class: "flex flex-row justify-center",
                button {
                    "aria-label": "edit description",
                    class: "
                        group
                        flex flex-row justify-center items-center
                        py-1 px-6
                        {edit_button_style}
                    ",
                    onclick: move |_| editing.set(true),
                    div {
                        class: "relative",
                        div { class: "size-5", EditIcon {} }
                        Tooltip { content: "Edit Description", position: "-top-12 -left-10" }
                    }
                }
            }
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
    let style = "stroke-red-600";
    let board_signals = BoardSignals::default();
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "delete task",
                class: "block size-6 {style}",
                onclick: move |_| {
                    spawn_forever(delete_task(board_signals, task_id));
                },
                TrashIcon {}
            }
            Tooltip { content: "Delete Task", position: "-top-10 -left-20" }
        }
    }
}

#[component]
fn StatusButtons(task_id: TaskId) -> Element {
    rsx! {
        section {
            "aria-label": "set task status",
            class: "flex flex-row items-center justify-end gap-1",
            ToDoButton { task_id }
            InProgressButton { task_id }
            DoneButton { task_id }
        }
    }
}

#[component]
fn ToDoButton(task_id: TaskId) -> Element {
    let style = "active:stroke-red-600 sm:hover:stroke-red-600";
    let board_signals = BoardSignals::default();
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "set task status to to do",
                class: "block size-8 {style}",
                onclick: move |_| {
                    spawn_forever(set_task_status(board_signals, task_id, TaskStatus::ToDo));
                },
                ToDoIcon {}
            }
            Tooltip { content: "To Do", position: "", dir: "rtl" }
        }
    }
}

#[component]
fn InProgressButton(task_id: TaskId) -> Element {
    let style = "active:stroke-fuchsia-600 sm:hover:stroke-fuchsia-600";
    let board_signals = BoardSignals::default();
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "set task status to in progress",
                class: "block size-8 {style}",
                onclick: move |_| {
                    spawn_forever(set_task_status(board_signals, task_id, TaskStatus::InProgress));
                },
                InProgressIcon {}
            }
            Tooltip { content: "In Progress", position: "", dir: "rtl" }
        }
    }
}

#[component]
fn DoneButton(task_id: TaskId) -> Element {
    let style = "active:stroke-green-500 sm:hover:stroke-green-500";
    let board_signals = BoardSignals::default();
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "set task status to done",
                class: "block size-8 {style}",
                onclick: move |_| {
                    spawn_forever(set_task_status(board_signals, task_id, TaskStatus::Done));
                },
                DoneIcon {}
            }
            Tooltip { content: "Done", position: "", dir: "rtl" }
        }
    }
}

#[component]
fn Assignees(
    task_id: TaskId,
    assignees: Vec<UserId>,
    select_assignees: Signal<bool>,
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
            ToggleSelector {
                task_id,
                r#for: "task-{task_id}-assignees",
                show_selector: select_assignees,
                aria_label: "toggle assignee selection",
                tooltip: "Assign User",
                size,
                tooltip_position,
                dir,
            }
        }
    }
}

#[component]
fn ToggleSelector(
    task_id: TaskId,
    r#for: String,
    show_selector: Signal<bool>,
    aria_label: String,
    tooltip: String,
    size: &'static str,
    tooltip_position: Option<&'static str>,
    dir: Option<&'static str>,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("rounded border-2 {}", theme.button);
    let mut scroll_target = use_context::<Signal<ScrollTarget>>();
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": aria_label,
                class: "block {size} {style}",
                "aria-pressed": show_selector(),
                onclick: move |_| {
                    let show = show_selector();
                    if show {
                        scroll_target.set(ScrollTarget(Some(format!("task-{task_id}-article"))));
                    } else {
                        scroll_target.set(ScrollTarget(Some(r#for.clone())));
                    }
                    show_selector.set(!show);
                },
                PlusIcon {}
            }
            Tooltip { content: tooltip, position: tooltip_position, dir }
        }
    }
}

#[component]
fn AssigneeSelection(task_id: TaskId, assignees: Vec<UserId>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("rounded-lg border {}", theme.border_color);
    let users = use_context::<Signal<Users>>();
    let users = &users.read().0;
    let mut assignee_data = Vec::with_capacity(assignees.len());
    let mut unassigned = Vec::with_capacity(users.len() - assignees.len());
    for (user_id, user) in users.iter() {
        if assignees.contains(user_id) {
            assignee_data.push((*user_id, user.clone()));
        } else {
            unassigned.push((*user_id, user.clone()));
        }
    }
    unassigned.sort_by_key(|(_, user)| user.name.to_lowercase());
    rsx! {
        section {
            "aria-label": "assignee selection",
            class: "flex flex-col gap-2 p-2 {style}",
            UserBadges { task_id, assignees: assignee_data }
            UserList { task_id, unassigned }
        }
    }
}

#[component]
fn TagSelection(task_id: TaskId, tags: Vec<TagId>) -> Element {
    let tag_data = use_context::<Signal<Tags>>();
    let tag_data = &tag_data.read().0;
    let mut unassigned = Vec::with_capacity(tag_data.len() - tags.len());
    for (user_id, user) in tag_data.iter() {
        if !tags.contains(user_id) {
            unassigned.push((*user_id, user.clone()));
        }
    }
    unassigned.sort_by_key(|(_, tag)| tag.name.to_lowercase());
    rsx! {
        section {
            "aria-label": "tag selection",
            AssignmentList {
                body: rsx! {
                    for (tag_id, tag) in unassigned {
                        TagListItem { key: "{tag_id}", task_id, tag_id, tag }
                    }
                    AddTagListItem { key: "{\"add-tag\"}", task_id, }
                }
            }
        }
    }
}

#[component]
fn UserBadges(task_id: TaskId, assignees: Vec<(UserId, UserData)>) -> Element {
    rsx! {
       div {
            class: "flex flex-row gap-2 flex-wrap group text-colored",
            for (user_id, user_data) in assignees {
                UserBadge { task_id, user_id, user_data }
            }
        }
    }
}

#[component]
fn UserBadge(task_id: TaskId, user_id: UserId, user_data: UserData) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let board_signals = BoardSignals::default();
    let style = "border-2 rounded";
    let button_style = "
        rounded-md
        stroke-white
        sm:hover:border sm:hover:border-white
    ";
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
    let unassign_label = format!("unassign {} from task", user_data.name);
    rsx! {
        div {
            class: "
                flex flex-row items-center gap-2
                text-sm py-1 px-2 {style} {color}
            ",
            {user_data.name}
            button {
                "aria-label": unassign_label,
                class: "size-5 p-0.5 {button_style}",
                onclick: move |_| {
                    spawn_forever(delete_task_assignee(board_signals, task_id, user_id));
                },
                CancelIcon {}
            }
        }
    }
}

#[component]
fn AssignmentList(body: Element) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        rounded-lg shadow
        border
        divide-y
        {} {}
    ",
        theme.border_color, theme.divide_color
    );
    rsx! {
        ul {
            class: "text-sm {style}",
            {body}
        }
    }
}

#[component]
fn AssignmentListItem(
    content: String,
    color: Color,
    aria_label: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = "sm:hover:underline";
    let color = match color {
        Color::Black => theme.color1_text,
        Color::White => theme.color2_text,
        Color::Gray => theme.color3_text,
        Color::Silver => theme.color4_text,
        Color::Maroon => theme.color5_text,
        Color::Red => theme.color6_text,
        Color::Purple => theme.color7_text,
        Color::Fushsia => theme.color8_text,
        Color::Green => theme.color9_text,
        Color::Lime => theme.color10_text,
        Color::Olive => theme.color11_text,
        Color::Yellow => theme.color12_text,
        Color::Navy => theme.color13_text,
        Color::Blue => theme.color14_text,
        Color::Teal => theme.color15_text,
        Color::Aqua => theme.color16_text,
    };
    rsx! {
        li {
            button {
                class: "px-4 py-2 w-full text-left {style} {color}",
                onclick: move |event| onclick.call(event),
                {content}
            }
        }
    }
}

#[component]
fn UserList(task_id: TaskId, unassigned: Vec<(UserId, UserData)>) -> Element {
    rsx! {
        AssignmentList {
            body: rsx! {
                for (user_id, user) in unassigned {
                    UserListItem { key: "{user_id}", task_id, user_id, user }
                }
                AddUserListItem { key: "{\"add-user\"}", task_id, }
            }
        }
    }
}

#[component]
fn UserListItem(task_id: TaskId, user_id: UserId, user: UserData) -> Element {
    let board_signals = BoardSignals::default();
    let label = format!("assign {} to task", user.name);
    rsx! {
        AssignmentListItem {
            content: user.name,
            color: user.color,
            aria_label: label,
            onclick: move |_| {
                spawn_forever(add_task_assignee(board_signals, task_id, user_id));
            },
        }
    }
}

#[component]
fn TagListItem(task_id: TaskId, tag_id: TagId, tag: TagData) -> Element {
    let board_signals = BoardSignals::default();
    let label = format!("assign {} to task", tag.name);
    rsx! {
        AssignmentListItem {
            content: tag.name,
            color: tag.color,
            aria_label: label,
            onclick: move |_| {
                spawn_forever(add_task_tag(board_signals, task_id, tag_id));
            },
        }
    }
}

#[component]
fn AddUserListItem(task_id: TaskId) -> Element {
    let show_form = use_signal(|| false);
    rsx! {
        li {
            if show_form() {
                AddUserListForm { task_id, show_form }
            } else {
                ShowSelectionListFormButton {
                    r#for: "task-{task_id}-new-user-form",
                    content: "Add User",
                    show_form ,
                }
            }
        }
    }
}

#[component]
fn AddTagListItem(task_id: TaskId) -> Element {
    let show_form = use_signal(|| false);
    rsx! {
        li {
            if show_form() {
                AddTagListForm { task_id, show_form }
            } else {
                ShowSelectionListFormButton {
                    r#for: "task-{task_id}-new-tag-form",
                    content: "Add Tag",
                    show_form,
                }
            }
        }
    }
}

#[component]
fn ShowSelectionListFormButton(r#for: String, content: String, show_form: Signal<bool>) -> Element {
    let mut scroll_target = use_context::<Signal<ScrollTarget>>();
    let style = "text-blue-500 sm:hover:underline active:underline";
    rsx! {
        button {
            class: "px-4 py-2 w-full text-left {style}",
            onclick: move |_| {
                scroll_target.set(ScrollTarget(Some(r#for.clone())));
                show_form.set(true)
            },
            {content}
        }
    }
}

#[component]
fn AddUserListForm(task_id: TaskId, show_form: Signal<bool>) -> Element {
    let board_signals = BoardSignals::default();
    rsx! {
        li {
            form {
                id: "task-{task_id}-new-user-form",
                "aria-label": "add user",
                class: "flex flex-col gap-2 p-2",
                onsubmit: move |event| {
                    let name = event.values()["Name"].as_value();
                    let color = serde_json::from_str(
                        &event.values()["color-picker"].as_value()
                    ).unwrap();
                    spawn_forever(create_user(board_signals, task_id, UserData{ name, color }));
                    show_form.set(false);
                },
                TextInput {
                    id: "task-{task_id}-new-user-name-input",
                    label: "Name",
                }
                ColorPicker { }
                div {
                    class: "flex flex-row gap-2 items-center justify-center",
                    ConfirmButton { label: "add user" }
                    CancelButton {
                        label: "cancel adding user",
                        editing: show_form,
                    }
                }
            }
        }
    }
}

#[component]
fn AddTagListForm(task_id: TaskId, show_form: Signal<bool>) -> Element {
    let board_signals = BoardSignals::default();
    rsx! {
        li {
            form {
                id: "task-{task_id}-new-tag-form",
                "aria-label": "add tag",
                class: "flex flex-col gap-2 p-2",
                onsubmit: move |event| {
                    let name = event.values()["Name"].as_value();
                    let color = serde_json::from_str(
                        &event.values()["color-picker"].as_value()
                    ).unwrap();
                    spawn_forever(create_tag(board_signals, task_id, TagData{ name, color }));
                    show_form.set(false);
                },
                TextInput {
                    id: "task-{task_id}-new-tag-name-input",
                    label: "Name",
                }
                ColorPicker { }
                div {
                    class: "flex flex-row gap-2 items-center justify-center",
                    ConfirmButton { label: "add tag" }
                    CancelButton {
                        label: "cancel adding tag",
                        editing: show_form,
                    }
                }
            }
        }
    }
}

#[component]
fn TaskTags(task_id: TaskId, tags: Vec<TagId>, select_tags: Signal<bool>) -> Element {
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
            ToggleSelector {
                task_id,
                r#for: "task-{task_id}-tags",
                show_selector: select_tags,
                aria_label: "toggle tag selection",
                tooltip: "Add Tag",
                size: "size-6",
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
            Tooltip { position: "-top-10 -left-20", content: tooltip }
        }
    }
}

#[component]
fn TaskActions(task_id: TaskId) -> Element {
    let board_signals = BoardSignals::default();
    rsx! {
        section {
            "aria-label": "task actions",
            class: "flex flex-row gap-1",
            // ActionButton {
            //     onclick: move |_| {
            //         spawn_forever(create_quick_add_task(board_signals, task_id));
            //     },
            //     tooltip: "Add to Quick Tasks",
            //     body: rsx!(BoltIcon {}),
            // }
            ActionButton {
                onclick: move |_| {
                    spawn_forever(clone_task(board_signals, task_id)) ;
                },
                tooltip: "Duplicate Task",
                body: rsx!(CopyIcon {})
            }
            ActionButton {
                onclick: move |_| {
                    spawn_forever(archive_task(board_signals, task_id));
                },
                tooltip: "Archive Task",
                body: rsx!(ArchiveIcon {})
            }
        }
    }
}

async fn set_task_status(signals: BoardSignals, task_id: TaskId, status: TaskStatus) {
    if send_set_task_status_request(signals, task_id, status)
        .await
        .is_ok()
    {
        requests::board(signals).await;
    }
}

async fn send_set_task_status_request(
    signals: BoardSignals,
    task_id: TaskId,
    status: TaskStatus,
) -> Result<(), anyhow::Error> {
    let url = {
        let url = &signals.url.read().0;
        let board = signals.board.read();
        url.join(&format!(
            "/api/boards/{}/tasks/{}/status",
            board.board_name, task_id
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

async fn clone_task(signals: BoardSignals, task_id: TaskId) {
    if send_clone_task_request(signals, task_id).await.is_ok() {
        requests::board(signals).await;
    }
}

async fn send_clone_task_request(
    signals: BoardSignals,
    task_id: TaskId,
) -> Result<TaskId, anyhow::Error> {
    let url = {
        let url = &signals.url.read().0;
        let board = signals.board.read();
        url.join(&format!(
            "/api/boards/{}/tasks/{}/clone",
            board.board_name, task_id
        ))?
    };
    Ok(reqwest::Client::new()
        .post(url)
        .send()
        .await?
        .json::<TaskId>()
        .await?)
}

async fn archive_task(signals: BoardSignals, task_id: TaskId) {
    if send_archive_task_request(signals, task_id).await.is_ok() {
        requests::board(signals).await;
    }
}

async fn send_archive_task_request(
    signals: BoardSignals,
    task_id: TaskId,
) -> Result<(), anyhow::Error> {
    let url = {
        let url = &signals.url.read().0;
        let board = signals.board.read();
        url.join(&format!(
            "/api/boards/{}/tasks/{}/archived",
            board.board_name, task_id
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

async fn delete_task_assignee(signals: BoardSignals, task_id: TaskId, assignee: UserId) {
    if send_delete_task_assignee_request(signals, task_id, assignee)
        .await
        .is_ok()
    {
        requests::board(signals).await;
    }
}

async fn send_delete_task_assignee_request(
    signals: BoardSignals,
    task_id: TaskId,
    assignee: UserId,
) -> Result<(), anyhow::Error> {
    let url = {
        let url = &signals.url.read().0;
        let board = signals.board.read();
        url.join(&format!(
            "/api/boards/{}/tasks/{}/assignees/{}",
            board.board_name, task_id, assignee
        ))?
    };
    Ok(Client::new().delete(url).send().await?.json::<()>().await?)
}

async fn add_task_assignee(signals: BoardSignals, task_id: TaskId, assignee: UserId) {
    if send_add_task_assignee_request(signals, task_id, assignee)
        .await
        .is_ok()
    {
        requests::board(signals).await;
    }
}

async fn send_add_task_assignee_request(
    signals: BoardSignals,
    task_id: TaskId,
    assignee: UserId,
) -> Result<(), anyhow::Error> {
    let url = {
        let url = &signals.url.read().0;
        let board = signals.board.read();
        url.join(&format!(
            "/api/boards/{}/tasks/{}/assignees",
            board.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .post(url)
        .json(&assignee)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn create_user(signals: BoardSignals, task_id: TaskId, user_data: UserData) {
    match requests::create_user(signals.url, signals.board, user_data).await {
        Ok((user_id, _)) => {
            add_task_assignee(signals, task_id, user_id).await;
        }
        Err(e) => {
            log::info!("Error creating user: {:?}", e);
        }
    }
}

async fn create_tag(signals: BoardSignals, task_id: TaskId, tag_data: TagData) {
    match requests::create_tag(signals.url, signals.board, tag_data).await {
        Ok((tag_id, _)) => {
            add_task_tag(signals, task_id, tag_id).await;
        }
        Err(e) => {
            log::info!("Error creating tag: {:?}", e);
        }
    }
}

async fn add_task_tag(signals: BoardSignals, task_id: TaskId, tag_id: TagId) {
    if send_add_task_tag_request(signals.url, signals.board, task_id, tag_id)
        .await
        .is_ok()
    {
        requests::board(signals).await;
    }
}

async fn send_add_task_tag_request(
    url: Signal<UnloadUrl>,
    board: Signal<Board>,
    task_id: TaskId,
    tag_id: TagId,
) -> Result<(), anyhow::Error> {
    let url = {
        let url = &url.read().0;
        let board = board.read();
        url.join(&format!(
            "/api/boards/{}/tasks/{}/tags",
            board.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .post(url)
        .json(&tag_id)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn set_task_description(signals: BoardSignals, task_id: TaskId, description: String) {
    if send_set_task_description_request(signals, task_id, description)
        .await
        .is_ok()
    {
        requests::board(signals).await;
    }
}

async fn send_set_task_description_request(
    signals: BoardSignals,
    task_id: TaskId,
    description: String,
) -> Result<(), anyhow::Error> {
    let url = {
        let url = &signals.url.read().0;
        let board = signals.board.read();
        url.join(&format!(
            "/api/boards/{}/tasks/{}/description",
            board.board_name, task_id
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

async fn delete_task(signals: BoardSignals, task_id: TaskId) {
    if send_delete_task_request(signals, task_id).await.is_ok() {
        requests::board(signals).await;
    }
}

async fn send_delete_task_request(
    signals: BoardSignals,
    task_id: TaskId,
) -> Result<(), anyhow::Error> {
    let url = {
        let url = &signals.url.read().0;
        let board = signals.board.read();
        url.join(&format!(
            "/api/boards/{}/tasks/{}",
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
