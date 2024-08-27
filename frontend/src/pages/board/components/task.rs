use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use reqwest::Client;
use shared_models::{TagId, TaskId, TaskStatus, UserId};

use crate::{
    commands::ScrollTarget,
    components::{
        icons::{
            ArchiveIcon, CopyIcon, DoneIcon, DownIcon, InProgressIcon, ToDoIcon, TrashIcon, UpIcon,
        },
        tooltip::Tooltip,
    },
    model::UnloadUrl,
    pages::board::{
        components::{
            assignee_selection::AssigneeSelection,
            assignees::FilteringAssignees,
            tag_selection::TagSelection,
            task::{
                description::Description,
                due::{Due, DueOptions},
                title::{DenseTitle, Title},
            },
            task_tags::FilteringTaskTags,
        },
        model::Board,
        requests::{self, BoardSignals},
    },
    themes::Theme,
};

mod description;
mod due;
mod title;

fn is_late(due: &Option<DateTime<Utc>>) -> bool {
    due.map_or(false, |due| due < chrono::Utc::now())
}

#[component]
pub fn Task(
    task_id: TaskId,
    title: ReadOnlySignal<String>,
    description: ReadOnlySignal<String>,
    status: TaskStatus,
    assignees: ReadOnlySignal<Vec<UserId>>,
    tags: ReadOnlySignal<Vec<TagId>>,
    due: Option<DateTime<Utc>>,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let is_late = is_late(&due);
    let style = format!(
        "
        sm:rounded-lg
        sm:shadow
        {} {} {}
        ",
        if is_late && status != TaskStatus::Done {
            theme.late_border_color
        } else {
            theme.border_color
        },
        if is_late {
            "first:border-t border-b sm:first:border-t-2 sm:border-2 "
        } else {
            "first:border-t border-b sm:border"
        },
        theme.bg_color_2,
    );
    let expanded = use_signal(|| false);
    let expanded_ = expanded();
    let select_assignees = use_signal(|| false);
    let select_tags = use_signal(|| false);

    let board_signals = BoardSignals::default();
    let mut scroll_target = use_context::<Signal<ScrollTarget>>();
    rsx! {
        article {
            id: "task-{task_id}-article",
            aria_label: title,
            class: "flex flex-col gap-2 p-2.5 {style}",
            div {
                class: "flex flex-row justify-between items-center",
                div {
                    class: "flex flex-row items-center gap-1",
                    ToggleExpanded { task_id, expanded, size: "size-7" }
                    Title { task_id, title }
                }
                StatusButtons { task_id, status }
            }
            div {
                class: "flex flex-row justify-between items-center",
                FilteringAssignees {
                    id: "task-{task_id}-assignees",
                    assignees,
                    select_assignees
                    on_toggle_selector: move |show| {
                        if show {
                            scroll_target.set(ScrollTarget(Some(format!("task-{task_id}-article"))));
                        } else {
                            scroll_target.set(ScrollTarget(Some(format!("task-{task_id}-assignees"))));
                        }
                    },
                }
                TaskActions { task_id }
            }
            if select_assignees() {
                AssigneeSelection {
                    id: "task-{task_id}-assignee-selection",
                    assignees,
                    on_assign_user: move |user_id| {
                        spawn_forever(add_task_assignee(board_signals, task_id, user_id));
                    },
                    on_unassign_user: move |user_id| {
                        spawn_forever(delete_task_assignee(board_signals, task_id, user_id));
                    },
                    on_add_user: move |user_id| {
                        spawn_forever(add_task_assignee(board_signals, task_id, user_id));
                    },
                }
            }
            FilteringTaskTags {
                id: "task-{task_id}-tags",
                tags,
                select_tags,
                on_unassign_tag: move |tag_id| {
                    spawn_forever(requests::delete_task_tag(board_signals, task_id, tag_id));
                },
                on_toggle_selector: move |show| {
                    if show {
                        scroll_target.set(ScrollTarget(Some(format!("task-{task_id}-article"))));
                    } else {
                        scroll_target.set(ScrollTarget(Some(format!("task-{task_id}-tags"))));
                    }
                },
            }
            if select_tags() {
                TagSelection {
                    id: "task-{task_id}-tag-selection",
                    tags,
                    on_assign_tag: move |tag_id| {
                        spawn_forever(add_task_tag(board_signals, task_id, tag_id));
                    },
                    on_add_tag: move |tag_id| {
                        spawn_forever(add_task_tag(board_signals, task_id, tag_id));
                    },
                }
            }
            if expanded_ || (is_late && status != TaskStatus::Done) {
                Due {
                    task_id,
                    due: due.map(|due| DueOptions {
                        due,
                        show_time_left: status != TaskStatus::Done,
                        is_late,
                    })
                }
            }
            if expanded() {
                Description { task_id, description }
                SpecialActions { task_id }
            }
        }
    }
}

#[component]
pub fn DenseTask(
    task_id: TaskId,
    title: ReadOnlySignal<String>,
    description: ReadOnlySignal<String>,
    status: TaskStatus,
    assignees: ReadOnlySignal<Vec<UserId>>,
    tags: ReadOnlySignal<Vec<TagId>>,
    due: Option<DateTime<Utc>>,
) -> Element {
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
    let is_late = is_late(&due);
    let board_signals = BoardSignals::default();
    let mut scroll_target = use_context::<Signal<ScrollTarget>>();
    rsx! {
        article {
            id: "task-{task_id}-article",
            aria_label: title,
            class: "flex flex-col gap-2 p-1 {style}",
            div {
                class: "flex flex-row justify-between",
                div {
                    class: "flex flex-row items-center gap-1",
                    ToggleExpanded { task_id, expanded, size: "size-5" }
                    DenseTitle { task_id, title, is_late, expanded: expanded_ }
                }
                FilteringAssignees {
                    id: "task-{task_id}-assignees",
                    assignees,
                    select_assignees,
                    icon_size: if expanded_ { "size-6" } else { "size-5" },
                    tooltip_position: "",
                    dir: "rtl",
                    on_toggle_selector: move |show| {
                        if show {
                            scroll_target.set(ScrollTarget(Some(format!("task-{task_id}-article"))));
                        } else {
                            scroll_target.set(ScrollTarget(Some(format!("task-{task_id}-assignees"))));
                        }
                    },
                }
            }
            if select_assignees() {
                AssigneeSelection {
                    id: "task-{task_id}-assignee-selection",
                    assignees,
                    on_assign_user: move |user_id| {
                        spawn_forever(add_task_assignee(board_signals, task_id, user_id));
                    },
                    on_unassign_user: move |user_id| {
                        spawn_forever(delete_task_assignee(board_signals, task_id, user_id));
                    },
                    on_add_user: move |user_id| {
                        spawn_forever(add_task_assignee(board_signals, task_id, user_id));
                    },
                }
            }
            if expanded_ {
                div {
                    class: "flex flex-row justify-center items-center",
                    StatusButtons { task_id, status }
                }
                Description { task_id, description }
                Due {
                    task_id,
                    due: due.map(|due| DueOptions {
                        due,
                        show_time_left: status != TaskStatus::Done,
                        is_late,
                    })
                }
                div {
                    class: "flex flex row justify-center items-center",
                    TaskActions { task_id }
                }
                FilteringTaskTags {
                    id: "task-{task_id}-tags",
                    tags,
                    select_tags,
                    on_unassign_tag: move |tag_id| {
                        spawn_forever(requests::delete_task_tag(board_signals, task_id, tag_id));
                    },
                    on_toggle_selector: move |show| {
                        if show {
                            scroll_target.set(ScrollTarget(Some(format!("task-{task_id}-article"))));
                        } else {
                            scroll_target.set(ScrollTarget(Some(format!("task-{task_id}-tags"))));
                        }
                    },
                }
                if select_tags() {
                    TagSelection {
                        id: "task-{task_id}-tag-selection",
                        tags,
                        on_assign_tag: move |tag_id| {
                            spawn_forever(add_task_tag(board_signals, task_id, tag_id));
                        },
                        on_add_tag: move |tag_id| {
                            spawn_forever(add_task_tag(board_signals, task_id, tag_id));
                        },
                    }
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
fn StatusButtons(task_id: TaskId, status: TaskStatus) -> Element {
    rsx! {
        section {
            "aria-label": "set task status",
            class: "flex flex-row items-center justify-end gap-1",
            ToDoButton { task_id, status }
            InProgressButton { task_id, status }
            DoneButton { task_id, status }
        }
    }
}

#[component]
fn ToDoButton(task_id: TaskId, status: TaskStatus) -> Element {
    let style = format!(
        "active:stroke-red-600 sm:hover:stroke-red-600 {}",
        if status == TaskStatus::ToDo {
            "stroke-red-600"
        } else {
            ""
        }
    );
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
fn InProgressButton(task_id: TaskId, status: TaskStatus) -> Element {
    let style = format!(
        "active:stroke-fuchsia-600 sm:hover:stroke-fuchsia-600 {}",
        if status == TaskStatus::InProgress {
            "stroke-fuchsia-600"
        } else {
            ""
        }
    );
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
fn DoneButton(task_id: TaskId, status: TaskStatus) -> Element {
    let style = format!(
        "active:stroke-green-500 sm:hover:stroke-green-500 {}",
        if status == TaskStatus::Done {
            "stroke-green-500"
        } else {
            ""
        }
    );
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
