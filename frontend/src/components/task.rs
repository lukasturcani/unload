use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use reqwest::Client;
use shared_models::{Color, QuickAddData, TagId, TaskId, TaskStatus, UserData, UserId};

use crate::{
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
fn TitleInput(task_id: TaskId, editing: Signal<bool>, title: String) -> Element {
    let style = "
        text-base
        rounded-lg
        border border-gray-600
        bg-gray-700
        focus:ring-blue-500 focus:border-blue-500
    ";
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
                input {
                    required: true,
                    class: "p-2.5 {style}",
                    oninput: move |event| title.set(event.value()),
                    value: title
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
            button {
                class: "size-5",
                onclick: move |_| {
                    editing.set(true);
                },
                EditIcon {}
            }
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
fn ToDoIcon() -> Element {
    rsx! {
        Icon {
            d: "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
        }
    }
}

#[component]
fn InProgressIcon() -> Element {
    rsx! {
        Icon {
            d: "M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
        }
    }
}

#[component]
fn DoneIcon() -> Element {
    rsx! {
        Icon {
            d: "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
        }
    }
}

#[component]
fn BoltIcon() -> Element {
    rsx! {
        Icon {
            d: "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z",
        }
    }
}

#[component]
fn CopyIcon() -> Element {
    rsx! {
        Icon {
            d: "M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 0 1-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 0 1 1.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 0 0-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 0 1-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 0 0-3.375-3.375h-1.5a1.125 1.125 0 0 1-1.125-1.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H9.75",
        }
    }
}

#[component]
fn ArchiveIcon() -> Element {
    rsx! {
        Icon {
            d: "m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z",
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
        Color::Black => "border-black aria-pressed:bg-black sm:hover:bg-black",
        Color::White => "border-white aria-pressed:bg-white sm:hover:bg-white",
        Color::Gray => "border-gray-400 aria-pressed:bg-gray-400 sm:hover:bg-gray-400",
        Color::Silver => "border-slate-500 aria-pressed:bg-slate-500 sm:hover:bg-slate-500",
        Color::Maroon => "border-rose-400 aria-pressed:bg-rose-400 sm:hover:bg-rose-400",
        Color::Red => "border-red-600 aria-pressed:bg-red-600 sm:hover:bg-red-600",
        Color::Purple => "border-purple-600 aria-pressed:bg-purple-600 sm:hover:bg-purple-600",
        Color::Fushsia => "border-fuchsia-400 aria-pressed:bg-fuchsia-400 sm:hover:bg-fuchsia-400",
        Color::Green => "border-emerald-500 aria-pressed:bg-emerald-500 sm:hover:bg-emerald-500",
        Color::Lime => "border-lime-500 aria-pressed:bg-lime-500 sm:hover:bg-lime-500",
        Color::Olive => "border-indigo-400 aria-pressed:bg-indigo-400 sm:hover:bg-indigo-400",
        Color::Yellow => "border-yellow-400 aria-pressed:bg-yellow-400 sm:hover:bg-yellow-400",
        Color::Navy => "border-amber-200 aria-pressed:bg-amber-200 sm:hover:bg-amber-200",
        Color::Blue => "border-blue-400 aria-pressed:bg-blue-400 sm:hover:bg-blue-400",
        Color::Teal => "border-teal-300 aria-pressed:bg-teal-300 sm:hover:bg-teal-300",
        Color::Aqua => "border-cyan-500 aria-pressed:bg-cyan-500 sm:hover:bg-cyan-500",
    };
    let style = "
        rounded border-2
        aria-pressed:ring aria-pressed:ring-blue-500
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

#[component]
fn EditIcon() -> Element {
    rsx! {
        Icon {
            d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10",
        }
    }
}

#[component]
fn ConfirmIcon() -> Element {
    rsx! {
        Icon {
            d: "m4.5 12.75 6 6 9-13.5",
        }
    }
}

#[component]
fn CancelIcon() -> Element {
    rsx! {
        Icon {
            d: "M6 18 18 6M6 6l12 12",
        }
    }
}

#[component]
fn PlusIcon() -> Element {
    rsx! {
        Icon {
            d: "M12 4.5v15m7.5-7.5h-15",
            stroke_width: "2",
        }
    }
}

#[component]
fn Icon(d: &'static str, stroke_width: Option<&'static str>) -> Element {
    let stroke_width = stroke_width.unwrap_or("1.5");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            "viewBox": "0 0 24 24",
            "stroke-width": stroke_width,
            path {
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                d,
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
