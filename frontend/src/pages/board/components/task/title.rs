use dioxus::prelude::*;
use reqwest::Client;
use shared_models::TaskId;

use crate::{
    components::{
        form::{CancelButton, ConfirmButton},
        icons::EditIcon,
        input::TextInput,
        tooltip::Tooltip,
    },
    pages::board::requests::{self, BoardSignals},
    themes::Theme,
};

#[component]
pub fn Title(task_id: TaskId, title: String) -> Element {
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
pub fn DenseTitle(task_id: TaskId, title: String, is_late: bool, expanded: bool) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            TitleInput { task_id, editing, title }
        } else {
            DenseTitleShow { task_id, editing, title, is_late, expanded }
        }
    }
}

#[component]
fn TitleInput(task_id: TaskId, editing: Signal<bool>, title: String) -> Element {
    let board_signals = BoardSignals::default();
    rsx! {
        form {
            "aria-label": "update title",
            class: "flex flex-col gap-2 justify-center items-center",
            onsubmit: move |event| {
                let title = event.values()["Title"].as_value();
                spawn_forever(set_task_title(board_signals, task_id, title));
                editing.set(false);
            },
            div {
                class: "flex flex-row gap-1 items-center",
                TextInput {
                    id: "task-{task_id}-title-input",
                    label: "Title",
                    value: title,
                }
            }
            div {
                class: "flex flex-row gap-1 items-center",
                ConfirmButton { label: "set title" }
                CancelButton { label: "cancel title update", editing }
            }
        }
    }
}

#[component]
fn DenseTitleShow(
    task_id: TaskId,
    editing: Signal<bool>,
    title: String,
    is_late: bool,
    expanded: bool,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let text_color = if is_late { theme.late_text_color } else { "" };
    rsx! {
        div {
            class: "flex flex-row gap-2 items-center",
            h3 {
                class: if expanded {
                    format!("
                        {text_color} text-lg sm:text-xl
                        font-bold tracking-tight
                    ")
                } else { format!("{text_color} text-sm tracking-tight") },
                {title}
            }
            SmallEditButton { task_id, editing }
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
            class: "group relative",
            button {
                "aria-label": "edit title",
                class: "block size-5",
                onclick: move |_| editing.set(true),
                EditIcon {}
            }
            Tooltip {
                content: "Edit Title",
                position: "",
            }
        }
    }
}

#[component]
fn SmallEditButton(task_id: TaskId, editing: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "edit title",
                class: "block size-4",
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
        let url = &signals.url.read().0;
        let board = signals.board.read();
        url.join(&format!(
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
