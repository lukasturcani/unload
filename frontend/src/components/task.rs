use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use reqwest::Client;
use shared_models::{Color, QuickAddData, TagId, TaskId, TaskStatus, UserData, UserId};

use crate::{
    components::icons::{
        ArchiveIcon, BoltIcon, CancelIcon, ConfirmIcon, CopyIcon, DoneIcon, EditIcon,
        InProgressIcon, PlusIcon, ToDoIcon,
    },
    model::{TaskData, UserFilter, Users},
    requests::{self, BoardSignals},
};

#[component]
pub fn Task(task_id: TaskId, task: TaskData) -> Element {
    let style = "
        border border-gray-700
        rounded-lg
        shadow
        bg-gray-800 sm:hover:bg-gray-700
    ";
    let expanded = use_signal(|| false);
    let user_search = use_signal(|| false);
    rsx! {
        div {
            class: "flex flex-col gap-2 p-3 {style}",
            div {
                class: "flex flex-row justify-between",
                Title { task_id, title: task.title }
                StatusButtons { task_id }
            }
            div {
                class: "flex flex row justify-between",
                Assignees { task_id, assignees: task.assignees.clone(), user_search }
                TaskActions { task_id }
            }
            if user_search() {
                UserSearch { task_id, assignees: task.assignees }
            }
            // Tags { task_id, tags: task.tags }
            // if expanded() {
            //     Due { task_id, due: task.due }
            //     Description { task_id, description: task.description }
            //     SpecialActions { task_id }
            // }
            // ToggleExpanded {
            //     expanded
            // }
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
            TitleShow { editing, title }
        }
    }
}

#[component]
fn TextInput(
    required: Option<bool>,
    placeholder: Option<String>,
    value: Option<Signal<String>>,
    oninput: EventHandler<FormEvent>,
) -> Element {
    let style = "
        text-base
        rounded-lg
        border border-gray-600
        bg-gray-700
        focus:ring-blue-500 focus:border-blue-500
    ";
    rsx! {
        input {
            class: "p-2.5 {style}",
            required,
            placeholder,
            value,
            oninput: move |event| oninput.call(event),
        }
    }
}

#[component]
fn TitleInput(task_id: TaskId, editing: Signal<bool>, title: String) -> Element {
    let mut title = use_signal(|| title);
    let board_signals = BoardSignals::default();
    rsx! {
        div {
            form {
                class: "flex flex-row gap-2 items-center",
                onsubmit: move |_| {
                    spawn_forever(set_task_title(board_signals, task_id, title()));
                    editing.set(false);
                },
                TextInput {
                    required: true,
                    value: title,
                    oninput: move |event: FormEvent| title.set(event.value()),
                }
                ConfirmButton {}
                CancelButton { editing }
            }
        }
    }
}

#[component]
fn ConfirmButton() -> Element {
    let style = "
        rounded-md
        border border-green-500
        stroke-green-500
        active:bg-green-500
        sm:hover:bg-green-500 sm:hover:stroke-white
    ";
    rsx! {
        button {
            class: "size-7 {style}",
            r#type: "submit",
            ConfirmIcon {}
        }
    }
}

#[component]
fn CancelButton(editing: Signal<bool>) -> Element {
    let style = "
        rounded-md
        border border-red-500
        stroke-red-500
        active:bg-red-500
        sm:hover:bg-red-500 sm:hover:stroke-white
    ";
    rsx! {
        button {
            class: "size-7 {style}",
            onclick: move |_| {
                editing.set(false);
            },
            CancelIcon {}
        }
    }
}

#[component]
fn TitleShow(editing: Signal<bool>, title: String) -> Element {
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
            EditButton { editing }
        }
    }
}

#[component]
fn EditButton(editing: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "relative",
            button {
                class: "peer size-5",
                onclick: move |_| editing.set(true),
                EditIcon {}
            }
            Tooltip { content: "Edit Title", position: "" }
        }
    }
}

#[component]
fn Description(task_id: TaskId, description: String) -> Element {
    todo!()
}

#[component]
fn SpecialActions(task_id: TaskId) -> Element {
    todo!()
}

#[component]
fn StatusButtons(task_id: TaskId) -> Element {
    rsx! {
        div {
            class: "flex flex-row",
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
fn Assignees(task_id: TaskId, assignees: Vec<UserId>, user_search: Signal<bool>) -> Element {
    let users = use_context::<Signal<Users>>();
    let users = &users.read().0;
    rsx! {
        div {
            class: "flex flex-row flex-wrap gap-2",
            for user_id in assignees {
                UserIcon { user_id, user_data: users[&user_id].clone() }
            }
            ToggleUserSearchButton { user_search }
        }
    }
}

#[component]
fn ToggleUserSearchButton(user_search: Signal<bool>) -> Element {
    let style = "
        rounded border-2 border-white
        sm:hover:bg-white sm:hover:stroke-black
        aria-pressed:bg-white aria-pressed:stroke-black
    ";
    rsx! {
        div {
            class: "relative",
            button {
                class: "peer size-6 {style}",
                "aria-pressed": user_search(),
                onclick: move |_| {
                    user_search.set(!user_search());
                },
                PlusIcon {}
            }
            Tooltip { content: "Assign User" }
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
    rsx! {
        div {
            class: "relative",
            button {
                class: "peer size-6 {style} {color}",
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

#[component]
fn Tooltip(content: String, position: Option<String>) -> Element {
    let style = "bg-gray-800 rounded-lg shadow-sm";
    let position = position.unwrap_or("-top-10 -left-2".to_string());
    rsx! {
        div {
            class: "
                pointer-events-none
                absolute {position} z-10
                w-max px-3 py-2 text-sm
                opacity-0 transition-opacity peer-hover:opacity-100
                {style}
            ",
            {content}
        }

    }
}

#[component]
fn UserSearch(task_id: TaskId, assignees: Vec<UserId>) -> Element {
    let style = "rounded-lg bg-gray-800 border border-gray-700";
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
        div {
            class: "flex flex-col gap-2 p-2 {style}",
            UserBadges { task_id, assignees: assignee_data }
            UserList { task_id, unassigned }
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
    rsx! {
        div {
            class: "
                flex flex-row items-center gap-2
                text-sm py-1 px-2 {style} {color}
            ",
            {user_data.name}
            button {
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
fn UserList(task_id: TaskId, unassigned: Vec<(UserId, UserData)>) -> Element {
    let style = "
        rounded-lg shadow
        bg-gray-800
        border border-gray-700
        divide-y divide-gray-700
    ";
    rsx! {
        ul {
            class: "text-sm {style}",
            for (user_id, user) in unassigned {
                UserListItem { key: "{user_id}", task_id, user_id, user }
            }
            AddUserListItem { key: "{\"add-user\"}" }
        }
    }
}

#[component]
fn UserListItem(task_id: TaskId, user_id: UserId, user: UserData) -> Element {
    let style = "active:bg-gray-700 sm:hover:bg-gray-700";
    let color = match user.color {
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
    let board_signals = BoardSignals::default();
    rsx! {
        li {
            button {
                class: "px-4 py-2 w-full text-left {style} {color}",
                onclick: move |_| {
                    spawn_forever(add_task_assignee(board_signals, task_id, user_id));
                },
                {user.name}
            }
        }
    }
}

#[component]
fn AddUserListItem() -> Element {
    let adding_user = use_signal(|| false);
    rsx! {
        li {
            if adding_user() {
                AddUserListForm { adding_user }
            } else {
                AddUserListButtom { adding_user }
            }
        }
    }
}

#[component]
fn AddUserListButtom(adding_user: Signal<bool>) -> Element {
    let style = "text-blue-500 sm:hover:underline active:underline";
    rsx! {
        button {
            class: "px-4 py-2 w-full text-left {style}",
            onclick: move |_| adding_user.set(true),
            "Add User"
        }
    }
}

#[component]
fn AddUserListForm(adding_user: Signal<bool>) -> Element {
    let mut name = use_signal(String::new);
    let mut color = use_signal(|| None::<Color>);
    rsx! {
        li {
            form {
                class: "flex flex-col gap-2 p-2",
                onsubmit: move |_| {
                    adding_user.set(false);
                },
                TextInput {
                    placeholder: "Name",
                    value: name,
                    required: true,
                    oninput: move |event: FormEvent| name.set(event.value()),
                }
                ColorPicker { color }
            }
        }
    }
}

#[component]
fn ColorPicker(color: Signal<Option<Color>>) -> Element {
    let fieldset_style = "rounded-lg border border-gray-700";
    let legend_style = "text-gray-400";
    let button_style = "
        border-2 border-blue-500 rounded-lg
        text-blue-500
        sm:hover:underline active:underline
    ";
    rsx! {
        fieldset {
            class: fieldset_style,
            legend {
                class: legend_style,
                "Color"
            }
            input {
                "aria-label": "color1",
                required: true,
                r#type: "radio",
                name: "color-picker",
                div {
                    class: "size-6 bg-rose-400",
                }
            }
            input {
                "aria-label": "color2",
                r#type: "radio",
                name: "color-picker",
                div {
                    class: "size-6 bg-emerald-500",
                }
            }
        }
        button {
            class: "px-4 py-2 w-full {button_style}",
            r#type: "submit",
            "Add"
        }
    }
}

#[component]
fn Tags(task_id: TaskId, tags: Vec<TagId>) -> Element {
    todo!()
}

#[component]
fn ActionButton(tooltip: String, body: Element, onclick: EventHandler<MouseEvent>) -> Element {
    let style = "sm:hover:stroke-blue-500 active:stroke-blue-500";
    rsx! {
        div {
            class: "relative",
            button {
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
        div {
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
fn Due(task_id: TaskId, due: Option<DateTime<Utc>>) -> Element {
    todo!()
}

#[component]
fn ToggleExpanded(expanded: Signal<bool>) -> Element {
    todo!()
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
