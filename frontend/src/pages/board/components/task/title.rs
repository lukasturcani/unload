use dioxus::prelude::*;
use dioxus_sdk::{i18n::*, translate};
use reqwest::Client;
use shared_models::TaskId;

use crate::{
    commands::ScrollTarget,
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
pub fn Title(task_id: TaskId, title: ReadOnlySignal<String>) -> Element {
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
pub fn DenseTitle(
    task_id: TaskId,
    title: ReadOnlySignal<String>,
    is_late: bool,
    expanded: bool,
) -> Element {
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
fn TitleInput(task_id: TaskId, editing: Signal<bool>, title: ReadOnlySignal<String>) -> Element {
    let i18 = use_i18();
    let board_signals = BoardSignals::default();
    let input_label = translate!(i18, "task_title_input_label");
    rsx! {
        form {
            aria_label: translate!(i18, "task_title_update_form_label"),
            class: "flex flex-col gap-2 justify-center items-center",
            onsubmit: move |event| {
                let title = event.values()[&input_label].as_value();
                spawn_forever(set_task_title(board_signals, task_id, title));
                editing.set(false);
            },
            div {
                class: "flex flex-row gap-1 items-center",
                TextInput {
                    id: "task-{task_id}-title-input",
                    label: input_label.clone(),
                    value: title,
                }
            }
            div {
                class: "flex flex-row gap-1 items-center",
                ConfirmButton { label: translate!(i18, "set_task_title_button_label") }
                CancelButton {
                    label: translate!(i18, "cancel_task_title_update_button_label"),
                    editing,
                }
            }
        }
    }
}

#[component]
fn DenseTitleShow(
    task_id: TaskId,
    editing: Signal<bool>,
    title: ReadOnlySignal<String>,
    is_late: bool,
    expanded: bool,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let text_color = if is_late { theme.late_text_color } else { "" };
    rsx! {
        div {
            class: "flex flex-row gap-2 pr-2 items-center",
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
fn TitleShow(task_id: TaskId, editing: Signal<bool>, title: ReadOnlySignal<String>) -> Element {
    rsx! {
        div {
            class: "flex flex-row gap-2 pr-2 items-center",
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
    let i18 = use_i18();
    let mut scroll_target = use_context::<Signal<ScrollTarget>>();
    rsx! {
        div {
            class: "group relative",
            button {
                aria_label: translate!(i18, "edit_task_title_tooltip"),
                class: "block size-5",
                onclick: move |_| {
                    scroll_target.set(
                        ScrollTarget(Some(format!("task-{task_id}-title-input")))
                    );
                    editing.set(true);
                },
                EditIcon {}
            }
            Tooltip {
                content: translate!(i18, "edit_task_title_tooltip"),
                position: "",
            }
        }
    }
}

#[component]
fn SmallEditButton(task_id: TaskId, editing: Signal<bool>) -> Element {
    let i18 = use_i18();
    rsx! {
        div {
            class: "group relative",
            button {
                aria_label: translate!(i18, "edit_task_title_tooltip"),
                class: "block size-4",
                onclick: move |_| editing.set(true),
                EditIcon {}
            }
            Tooltip {
                content: translate!(i18, "edit_task_title_tooltip"),
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
