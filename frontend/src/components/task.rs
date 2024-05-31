use dioxus::prelude::*;
use reqwest::Client;
use shared_models::{Color, QuickAddData, TagData, TagId, TaskId, TaskStatus, UserData, UserId};

use crate::{
    components::{
        due::{Due, DueOptions},
        form::{CancelButton, ConfirmButton},
        icons::{
            ArchiveIcon, BoltIcon, CancelIcon, CopyIcon, DoneIcon, DownIcon, EditIcon,
            InProgressIcon, PlusIcon, ToDoIcon, UpIcon,
        },
        input::TextInput,
        tooltip::Tooltip,
    },
    model::{Board, TagFilter, Tags, TaskData, UserFilter, Users},
    requests::{self, BoardSignals},
    themes::Theme,
};

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
        border
        rounded-lg
        shadow
        {} {} {}
        ",
        if is_late {
            theme.late_border_color
        } else {
            theme.border_color
        },
        theme.bg_color_2,
        theme.sm_hover_bg_color_3
    );
    let expanded = use_signal(|| false);
    let expanded_ = expanded();
    let select_assignees = use_signal(|| false);
    let select_tags = use_signal(|| false);
    let label = task.title.clone();
    rsx! {
        article {
            "aria-label": label,
            class: "flex flex-col gap-2 px-3 pt-3 {style}",
            div {
                class: "flex flex-row justify-between",
                Title { task_id, title: task.title }
                StatusButtons { task_id }
            }
            div {
                class: "flex flex row justify-between items-center",
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
            if expanded_ || is_late {
                Due {
                    task_id,
                    due: task.due.map(|due| DueOptions {
                        due,
                        show_time_left: status != TaskStatus::Done
                    })
                }
            }
            if expanded() {
                Description { task_id, description: task.description }
            //     SpecialActions { task_id }
            }
            ToggleExpanded { expanded }
        }
    }
}

#[component]
fn Title(task_id: TaskId, title: String) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            TitleInput { task_id, editing, title }
        } else {
            TitleShow { task_id, editing, title }
        }
    }
}

#[component]
fn TitleInput(task_id: TaskId, editing: Signal<bool>, title: String) -> Element {
    let board_signals = BoardSignals::default();
    rsx! {
        form {
            "aria-label": "update title",
            class: "flex flex-row gap-2 items-center",
            onsubmit: move |event| {
                let title = event.values()["Title"].as_value();
                spawn_forever(set_task_title(board_signals, task_id, title));
                editing.set(false);
            },
            TextInput {
                id: "task-{task_id}-title-input",
                label: "Title",
                value: title,
            }
            ConfirmButton { label: "set title" }
            CancelButton { label: "cancel title update", editing }
        }
    }
}

#[component]
fn TitleShow(task_id: TaskId, editing: Signal<bool>, title: String) -> Element {
    rsx! {
        div {
            class: "flex flex-row gap-2 items-center",
            h3 {
                class: "
                    text-lg sm:text-xl
                    font-bold tracking-tight
                    underline underline-offset-8
                ",
                {title}
            }
            EditButton { task_id, editing }
        }
    }
}

#[component]
fn EditButton(task_id: TaskId, editing: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "relative",
            button {
                "aria-label": "edit title",
                class: "peer size-5",
                onclick: move |_| editing.set(true),
                EditIcon {}
            }
            Tooltip {
                content: "Edit Title",
                position: ""
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
        "rounded-lg border {} {}",
        theme.bg_color_2, theme.border_color
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
    let edit_button_style = format!(
        "rounded border {} {}",
        theme.border_color, theme.sm_hover_bg_color_2
    );
    rsx! {
        section {
            "aria-label": "description",
            class: "flex flex-col gap-1 {style}",
            p { {description} }
            button {
                "aria-label": "edit description",
                class: "group w-full flex flex-row justify-center items-center {edit_button_style}",
                onclick: move |_| editing.set(true),
                div {
                    class: "relative",
                    div { class: "size-5", EditIcon {} }
                    Tooltip { content: "Edit Description" }
                }
            }
        }
    }
}

#[component]
fn SpecialActions(task_id: TaskId) -> Element {
    todo!()
}

#[component]
fn StatusButtons(task_id: TaskId) -> Element {
    rsx! {
        section {
            "aria-label": "set task status",
            class: "flex flex-row items-center",
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
            class: "relative",
            button {
                "aria-label": "set task status to to do",
                class: "peer size-9 {style}",
                onclick: move |_| {
                    spawn_forever(set_task_status(board_signals, task_id, TaskStatus::ToDo));
                },
                ToDoIcon {}
            }
            Tooltip { content: "To Do", position: "" }
        }
    }
}

#[component]
fn InProgressButton(task_id: TaskId) -> Element {
    let style = "active:stroke-yellow-300 sm:hover:stroke-yellow-300";
    let board_signals = BoardSignals::default();
    rsx! {
        div {
            class: "relative",
            button {
                "aria-label": "set task status to in progress",
                class: "peer size-9 {style}",
                onclick: move |_| {
                    spawn_forever(set_task_status(board_signals, task_id, TaskStatus::InProgress));
                },
                InProgressIcon {}
            }
            Tooltip { content: "In Progress", position: "-left-10" }
        }
    }
}

#[component]
fn DoneButton(task_id: TaskId) -> Element {
    let style = "active:stroke-green-500 sm:hover:stroke-green-500";
    let board_signals = BoardSignals::default();
    rsx! {
        div {
            class: "relative",
            button {
                "aria-label": "set task status to done",
                class: "peer size-9 {style}",
                onclick: move |_| {
                    spawn_forever(set_task_status(board_signals, task_id, TaskStatus::Done));
                },
                DoneIcon {}
            }
            Tooltip { content: "Done", position: "-left-4" }
        }
    }
}

#[component]
fn Assignees(task_id: TaskId, assignees: Vec<UserId>, select_assignees: Signal<bool>) -> Element {
    let users = use_context::<Signal<Users>>();
    let users = &users.read().0;
    rsx! {
        section {
            "aria-label": "assignees",
            class: "flex flex-row flex-wrap items-center gap-2",
            for user_id in assignees {
                UserIcon { user_id, user_data: users[&user_id].clone() }
            }
            ToggleSelector {
                show_selector: select_assignees,
                aria_label: "toggle assignee selection",
                tooltip: "Assign User"
            }
        }
    }
}

#[component]
fn ToggleSelector(show_selector: Signal<bool>, aria_label: String, tooltip: String) -> Element {
    let style = "
        rounded border-2 border-white
        sm:hover:bg-white sm:hover:stroke-black
        aria-pressed:bg-white aria-pressed:stroke-black
    ";
    rsx! {
        div {
            class: "relative",
            button {
                "aria-label": aria_label,
                class: "peer size-6 {style}",
                "aria-pressed": show_selector(),
                onclick: move |_| {
                    show_selector.set(!show_selector());
                },
                PlusIcon {}
            }
            Tooltip { content: tooltip }
        }
    }
}

#[component]
fn UserIcon(user_id: UserId, user_data: UserData) -> Element {
    let mut user_filter = use_context::<Signal<UserFilter>>();
    let color = match user_data.color {
        Color::Black => "border-black aria-pressed:bg-black",
        Color::White => "border-white aria-pressed:bg-white",
        Color::Gray => "border-gray-400 aria-pressed:bg-gray-400",
        Color::Silver => "border-slate-500 aria-pressed:bg-slate-500",
        Color::Maroon => "border-rose-400 aria-pressed:bg-rose-400",
        Color::Red => "border-red-600 aria-pressed:bg-red-600",
        Color::Purple => "border-purple-600 aria-pressed:bg-purple-600",
        Color::Fushsia => "border-fuchsia-400 aria-pressed:bg-fuchsia-400",
        Color::Green => "border-emerald-500 aria-pressed:bg-emerald-500",
        Color::Lime => "border-lime-500 aria-pressed:bg-lime-500",
        Color::Olive => "border-indigo-400 aria-pressed:bg-indigo-400",
        Color::Yellow => "border-yellow-400 aria-pressed:bg-yellow-400",
        Color::Navy => "border-amber-200 aria-pressed:bg-amber-200",
        Color::Blue => "border-blue-400 aria-pressed:bg-blue-400",
        Color::Teal => "border-teal-300 aria-pressed:bg-teal-300",
        Color::Aqua => "border-cyan-500 aria-pressed:bg-cyan-500",
    };
    let style = "
        rounded border-2
        sm:hover:border-4 active:border-4 sm:hover:scale-110 active:scale-110
    ";
    let label = format!("toggle {} filter", user_data.name);
    rsx! {
        div {
            class: "relative",
            button {
                class: "peer size-6 {style} {color}",
                "aria-label": label,
                "aria-pressed": user_filter.read().0.contains(&user_id),
                onclick: move |_| {
                    let mut user_filter = user_filter.write();
                    if user_filter.0.contains(&user_id) {
                        user_filter.0.remove(&user_id);
                    } else {
                        user_filter.0.insert(user_id);
                    }
                },
                div { class: "size-full" }
            }
            Tooltip { content: user_data.name }
        }
    }
}

const ASSIGNMENT_LIST_STYLE: &str = "rounded-lg border";

#[component]
fn AssigneeSelection(task_id: TaskId, assignees: Vec<UserId>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "{} {} {}",
        ASSIGNMENT_LIST_STYLE, theme.bg_color_2, theme.border_color
    );
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
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "{} {} {}",
        ASSIGNMENT_LIST_STYLE, theme.bg_color_2, theme.border_color
    );
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
            class: "p-2 {style}",
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
            class: "flex flex-row gap-2 flex-wrap",
            for (user_id, user_data) in assignees {
                UserBadge { task_id, user_id, user_data }
            }
        }
    }
}

#[component]
fn UserBadge(task_id: TaskId, user_id: UserId, user_data: UserData) -> Element {
    let board_signals = BoardSignals::default();
    let style = "border-2 rounded";
    let button_style = "
        rounded-md
        stroke-white
        sm:hover:border sm:hover:border-white
    ";
    let color = match user_data.color {
        Color::Black => "border-black text-black",
        Color::White => "border-white text-white",
        Color::Gray => "border-gray-400 text-gray-400",
        Color::Silver => "border-slate-500 text-slate-500",
        Color::Maroon => "border-rose-400 text-rose-400",
        Color::Red => "border-red-600 text-red-600",
        Color::Purple => "border-purple-600 text-purple-600",
        Color::Fushsia => "border-fuchsia-400 text-fuchsia-400",
        Color::Green => "border-emerald-500 text-emerald-500",
        Color::Lime => "border-lime-500 text-lime-500",
        Color::Olive => "border-indigo-400 text-indigo-400",
        Color::Yellow => "border-yellow-400 text-yellow-400",
        Color::Navy => "border-amber-200 text-amber-200",
        Color::Blue => "border-blue-400 text-blue-400",
        Color::Teal => "border-teal-300 text-teal-300",
        Color::Aqua => "border-cyan-500 text-cyan-500",
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
        {} {} {}
    ",
        theme.border_color, theme.bg_color_2, theme.divide_color
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
    let style = format!("{} {}", theme.sm_hover_bg_color_3, theme.active_bg_color_3);
    let color = match color {
        Color::Black => "text-black",
        Color::White => "text-white",
        Color::Gray => "text-gray-400",
        Color::Silver => "text-slate-500",
        Color::Maroon => "text-rose-400",
        Color::Red => "text-red-600",
        Color::Purple => "text-purple-600",
        Color::Fushsia => "text-fuchsia-400",
        Color::Green => "text-emerald-500",
        Color::Lime => "text-lime-500",
        Color::Olive => "text-indigo-400",
        Color::Yellow => "text-yellow-400",
        Color::Navy => "text-amber-200",
        Color::Blue => "text-blue-400",
        Color::Teal => "text-teal-300",
        Color::Aqua => "text-cyan-500",
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
                ShowSelectionListFormButton { content: "Add User", show_form }
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
                ShowSelectionListFormButton { content: "Add Tag", show_form }
            }
        }
    }
}

#[component]
fn ShowSelectionListFormButton(content: String, show_form: Signal<bool>) -> Element {
    let style = "text-blue-500 sm:hover:underline active:underline";
    rsx! {
        button {
            class: "px-4 py-2 w-full text-left {style}",
            onclick: move |_| show_form.set(true),
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
                "aria-label": "add user",
                class: "flex flex-col gap-2 p-2",
                onsubmit: move |event| {
                    let name = event.values()["Name"].as_value();
                    let color = color_from_string(&event.values()["color-picker"].as_value());
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
                "aria-label": "add tag",
                class: "flex flex-col gap-2 p-2",
                onsubmit: move |event| {
                    let name = event.values()["Name"].as_value();
                    let color = color_from_string(&event.values()["color-picker"].as_value());
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

fn color_to_string(color: Color) -> &'static str {
    match color {
        Color::Black => "black",
        Color::White => "white",
        Color::Gray => "gray",
        Color::Silver => "silver",
        Color::Maroon => "maroon",
        Color::Red => "red",
        Color::Purple => "purple",
        Color::Fushsia => "fushsia",
        Color::Green => "green",
        Color::Lime => "lime",
        Color::Olive => "olive",
        Color::Yellow => "yellow",
        Color::Navy => "navy",
        Color::Blue => "blue",
        Color::Teal => "teal",
        Color::Aqua => "aqua",
    }
}

fn color_from_string(color: &str) -> Color {
    match color {
        "black" => Color::Black,
        "white" => Color::White,
        "gray" => Color::Gray,
        "silver" => Color::Silver,
        "maroon" => Color::Maroon,
        "red" => Color::Red,
        "purple" => Color::Purple,
        "fushsia" => Color::Fushsia,
        "green" => Color::Green,
        "lime" => Color::Lime,
        "olive" => Color::Olive,
        "yellow" => Color::Yellow,
        "navy" => Color::Navy,
        "blue" => Color::Blue,
        "teal" => Color::Teal,
        "aqua" => Color::Aqua,
        _ => panic!("invalid color"),
    }
}

fn color_to_bg(color: Color) -> &'static str {
    match color {
        Color::Black => "bg-black",
        Color::White => "bg-white",
        Color::Gray => "bg-gray-400",
        Color::Silver => "bg-slate-500",
        Color::Maroon => "bg-rose-400",
        Color::Red => "bg-red-600",
        Color::Purple => "bg-purple-600",
        Color::Fushsia => "bg-fuchsia-400",
        Color::Green => "bg-emerald-500",
        Color::Lime => "bg-lime-500",
        Color::Olive => "bg-indigo-400",
        Color::Yellow => "bg-yellow-400",
        Color::Navy => "bg-amber-200",
        Color::Blue => "bg-blue-400",
        Color::Teal => "bg-teal-300",
        Color::Aqua => "bg-cyan-500",
    }
}

#[component]
fn ColorPicker() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let fieldset_style = format!("rounded-lg border {}", theme.border_color);
    let legend_style = "text-sm";
    let radio_style = "
        rounded-md
        ease-in-out duration-100
        hover:scale-125 peer-checked:scale-125
    ";
    rsx! {
        fieldset {
            class: "flex flex-row items-center justify-center py-2 {fieldset_style}",
            legend {
                class: legend_style,
                "Color"
            }
            div {
                class: "grid grid-cols-4 gap-4",
                for color in [
                    Color::Black,
                    Color::White,
                    Color::Gray,
                    Color::Silver,
                    Color::Maroon,
                    Color::Red,
                    Color::Purple,
                    Color::Fushsia,
                    Color::Green,
                    Color::Lime,
                    Color::Olive,
                    Color::Yellow,
                    Color::Navy,
                    Color::Blue,
                    Color::Teal,
                    Color::Aqua,
                ] {
                    label {
                        class: "flex flex-row items-center gap-2",
                        input {
                            value: color_to_string(color),
                            class: "peer",
                            "aria-label": "color1",
                            required: true,
                            r#type: "radio",
                            name: "color-picker",
                        }
                        div { class: "inline-block size-6 {radio_style} {color_to_bg(color)}" }
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
            "aria-label": "tags",
            class: "flex flex-row flex-wrap gap-2 items-center",
            for tag_id in tags {
                TagChip { task_id, tag_id, tag_data: tag_data[&tag_id].clone() }
            }
            ToggleSelector {
                show_selector: select_tags,
                aria_label: "toggle tag selection",
                tooltip: "Add Tag"
            }
        }
    }
}

#[component]
fn TagChip(task_id: TaskId, tag_id: TagId, tag_data: TagData) -> Element {
    let board_signals = BoardSignals::default();
    let mut tag_filter = use_context::<Signal<TagFilter>>();
    let color = match tag_data.color {
        Color::Black => "border-black aria-pressed:bg-black",
        Color::White => "border-white aria-pressed:bg-white",
        Color::Gray => "border-gray-400 aria-pressed:bg-gray-400",
        Color::Silver => "border-slate-500 aria-pressed:bg-slate-500",
        Color::Maroon => "border-rose-400 aria-pressed:bg-rose-400",
        Color::Red => "border-red-600 aria-pressed:bg-red-600",
        Color::Purple => "border-purple-600 aria-pressed:bg-purple-600",
        Color::Fushsia => "border-fuchsia-400 aria-pressed:bg-fuchsia-400",
        Color::Green => "border-emerald-500 aria-pressed:bg-emerald-500",
        Color::Lime => "border-lime-500 aria-pressed:bg-lime-500",
        Color::Olive => "border-indigo-400 aria-pressed:bg-indigo-400",
        Color::Yellow => "border-yellow-400 aria-pressed:bg-yellow-400",
        Color::Navy => "border-amber-200 aria-pressed:bg-amber-200",
        Color::Blue => "border-blue-400 aria-pressed:bg-blue-400",
        Color::Teal => "border-teal-300 aria-pressed:bg-teal-300",
        Color::Aqua => "border-cyan-500 aria-pressed:bg-cyan-500",
    };
    let style = "rounded border-2 ease-in-out duration-100 sm:hover:scale-110";
    let delete_tag_button_style = "rounded active:border sm:hover:border";
    let pressed = tag_filter.read().0.contains(&tag_id);
    rsx! {
        div {
            class: "
                group
                flex flex-row items-center
                px-1.5 py-0.5
                {style} {color}
            ",
            "aria-pressed": pressed,
            button {
                class: "text-sm pr-1",
                "aria-label": "toggle {tag_data.name} filter",
                "aria-pressed": pressed,
                onclick: move |_| {
                    let mut tag_filter = tag_filter.write();
                    if tag_filter.0.contains(&tag_id) {
                        tag_filter.0.remove(&tag_id);
                    } else {
                        tag_filter.0.insert(tag_id);
                    }
                },
                "# {tag_data.name}"
            }
            button {
                "aria-label": "remove tag {tag_data.name} from task",
                class: "size-5 p-0.5 {delete_tag_button_style}",
                onclick: move |_| {
                    spawn_forever(delete_task_tag(board_signals, task_id, tag_id));
                },
                CancelIcon {}
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
            class: "relative",
            button {
                "aria-label": aria_label,
                class: "peer size-7 {style}",
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
            class: "flex flex-row",
            ActionButton {
                onclick: move |_| {
                    spawn_forever(create_quick_add_task(board_signals, task_id));
                },
                tooltip: "Add to Quick Tasks",
                body: rsx!(BoltIcon {}),
            }
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

#[component]
fn ToggleExpanded(expanded: Signal<bool>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "rounded {} {}",
        theme.active_bg_color_2, theme.sm_hover_bg_color_2
    );
    let expanded_ = expanded();
    rsx! {
        button {
            "aria-label": "toggle expand task",
            "aria-pressed": expanded_,
            class: "
                h-8 w-full
                flex flex-row items-center justify-center
                {style}
            ",
            onclick: move |_| expanded.set(!expanded()),
            div {
                class: "size-6",
                if expanded_ {
                    UpIcon {}
                } else {
                    DownIcon {}
                }
            }
        }
    }
}

async fn set_task_title(signals: BoardSignals, task_id: TaskId, title: String) {
    if send_set_task_title_request(signals, task_id, title)
        .await
        .is_ok()
    {
        requests::board(signals).await;
    }
}

async fn send_set_task_title_request(
    signals: BoardSignals,
    task_id: TaskId,
    title: String,
) -> Result<(), anyhow::Error> {
    let url = {
        let board = signals.board.read();
        board.url.join(&format!(
            "/api/boards/{}/tasks/{}/title",
            board.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .put(url)
        .json(&title)
        .send()
        .await?
        .json::<()>()
        .await?)
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
        let board = signals.board.read();
        board.url.join(&format!(
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

async fn create_quick_add_task(signals: BoardSignals, task_id: TaskId) {
    if send_create_quick_add_task_request(signals, task_id)
        .await
        .is_ok()
    {
        requests::board(signals).await;
    }
}

async fn send_create_quick_add_task_request(
    signals: BoardSignals,
    task_id: TaskId,
) -> Result<TaskId, anyhow::Error> {
    let (url, task_data) = {
        let board = signals.board.read();
        let task = &signals.tasks.read().0[&task_id];
        let url = board
            .url
            .join(&format!("/api/boards/{}/quick-add", board.board_name))?;
        (
            url,
            QuickAddData {
                title: task.title.clone(),
                description: task.description.clone(),
                size: task.size,
                tags: task.tags.clone(),
                assignees: task.assignees.clone(),
            },
        )
    };
    Ok(reqwest::Client::new()
        .post(url)
        .json(&task_data)
        .send()
        .await?
        .json::<TaskId>()
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
        let board = signals.board.read();
        board.url.join(&format!(
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
        let board = signals.board.read();
        board.url.join(&format!(
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
        let board = signals.board.read();
        board.url.join(&format!(
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
        let board = signals.board.read();
        board.url.join(&format!(
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
    match requests::create_user(signals.board, user_data).await {
        Ok((user_id, _)) => {
            add_task_assignee(signals, task_id, user_id).await;
        }
        Err(e) => {
            log::info!("Error creating user: {:?}", e);
        }
    }
}

async fn create_tag(signals: BoardSignals, task_id: TaskId, tag_data: TagData) {
    match requests::create_tag(signals.board, tag_data).await {
        Ok((tag_id, _)) => {
            add_task_tag(signals, task_id, tag_id).await;
        }
        Err(e) => {
            log::info!("Error creating tag: {:?}", e);
        }
    }
}

async fn delete_task_tag(signals: BoardSignals, task_id: TaskId, tag_id: TagId) {
    if send_delete_task_tag_request(signals, task_id, tag_id)
        .await
        .is_ok()
    {
        requests::board(signals).await;
    }
}

async fn send_delete_task_tag_request(
    signals: BoardSignals,
    task_id: TaskId,
    tag_id: TagId,
) -> Result<(), anyhow::Error> {
    let url = {
        let board = signals.board.read();
        board.url.join(&format!(
            "/api/boards/{}/tasks/{}/tags/{}",
            board.board_name, task_id, tag_id
        ))?
    };
    Ok(reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn add_task_tag(signals: BoardSignals, task_id: TaskId, tag_id: TagId) {
    if send_add_task_tag_request(signals.board, task_id, tag_id)
        .await
        .is_ok()
    {
        requests::board(signals).await;
    }
}

async fn send_add_task_tag_request(
    board: Signal<Board>,
    task_id: TaskId,
    tag_id: TagId,
) -> Result<(), anyhow::Error> {
    let url = {
        let board = board.read();
        board.url.join(&format!(
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
        let board = signals.board.read();
        board.url.join(&format!(
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
