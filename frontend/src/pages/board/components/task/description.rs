use dioxus::prelude::*;
use dioxus_sdk::{i18n::use_i18, translate};
use reqwest::Client;
use shared_models::TaskId;

use crate::{
    components::{
        form::{CancelButton, ConfirmButton},
        icons::EditIcon,
        tooltip::Tooltip,
    },
    description_parser::{parse_blocks, Block, Line},
    pages::board::{
        components::description_input::DescriptionInput,
        requests::{self, BoardSignals},
    },
    themes::Theme,
};

#[component]
pub fn Description(task_id: TaskId, description: ReadOnlySignal<String>) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            DescriptionForm { task_id, editing, description }
        } else {
            DescriptionShow { task_id, editing, description }
        }
    }
}

#[component]
fn DescriptionForm(
    task_id: TaskId,
    editing: Signal<bool>,
    description: ReadOnlySignal<String>,
) -> Element {
    let i18 = use_i18();
    let board_signals = BoardSignals::default();
    rsx! {
        form {
            aria_label: translate!(i18, "description_update_form_label"),
            class: "flex flex-col gap-2",
            onsubmit: move |event| {
                let description = event.values()["Description"].as_value();
                spawn_forever(set_task_description(board_signals, task_id, description));
                editing.set(false);
            },
            DescriptionInput  {
                id: "task-{task_id}-description-input",
                editing,
                description,
            },
            div {
                class: "flex flex-row gap-2 items-center justify-center",
                ConfirmButton { label: translate!(i18, "set_description_button_label") }
                CancelButton {
                    label: translate!(i18, "cancel_description_update_button_label"),
                    editing,
                }
            }
        }
    }
}

#[component]
fn DescriptionShow(
    task_id: TaskId,
    description: ReadOnlySignal<String>,
    editing: Signal<bool>,
) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let edit_button_style = format!("rounded border {}", theme.button);
    let aria_label = translate!(i18, "edit_description_tooltip");
    rsx! {
        section {
            aria_label: translate!(i18, "description_section_label"),
            class: "flex flex-col gap-1",
            DescriptionContent { task_id, description }
            div {
                class: "flex flex-row justify-center",
                button {
                    aria_label,
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
                        Tooltip {
                            content: aria_label.clone(),
                            position: "-top-12 -left-10",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DescriptionContent(task_id: TaskId, description: ReadOnlySignal<String>) -> Element {
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
                                Checkbox { task_id, line, description }
                            }
                        }
                    },
                }
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
fn Checkbox(task_id: TaskId, description: ReadOnlySignal<String>, line: Line) -> Element {
    let board_signals = BoardSignals::default();
    let (head, tail) = line.content.split_once(']').unwrap();
    rsx! {
        li {
            label {
                input {
                    onchange: move |event| {
                        let mut description = description.read().clone();
                        if event.checked() {
                            description.replace_range(line.index+3..line.index+4, "x");
                        } else {
                            description.replace_range(line.index+3..line.index+4, " ");
                        };
                        spawn_forever(set_task_description(board_signals, task_id, description));
                    },
                    checked: head.ends_with('x'),
                    r#type: "checkbox",
                }
                {tail}
            }
        }
    }
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
