use dioxus::prelude::*;
use reqwest::Client;
use shared_models::TaskId;

use crate::{
    components::{
        form::{CancelButton, ConfirmButton},
        icons::EditIcon,
        tooltip::Tooltip,
    },
    pages::board::requests::{self, BoardSignals},
    themes::Theme,
};

#[component]
pub fn Description(task_id: TaskId, description: String) -> Element {
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
                onmounted: move |event| async move {
                    let _ = event.set_focus(true).await;
                },
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
    let edit_button_style = format!("rounded border {}", theme.button);
    rsx! {
        section {
            "aria-label": "description",
            class: "flex flex-col gap-1",
            DescriptionContent { description }
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

struct Line {
    index: usize,
    content: String,
}

enum Block {
    Text(String),
    Bullet(Vec<Line>),
    Checkbox(Vec<Line>),
}

fn parse_blocks(description: &str) -> Vec<Block> {
    let mut blocks = Vec::<Block>::new();
    let mut char_index = 0;
    for line in description.lines() {
        if line.starts_with('*') {
            if let Some(Block::Bullet(lines)) = blocks.last_mut() {
                lines.push(Line {
                    index: char_index,
                    content: line.into(),
                });
            } else {
                blocks.push(Block::Bullet(vec![Line {
                    index: char_index,
                    content: line.into(),
                }]));
            };
        } else if line.starts_with("- [ ]") || line.starts_with("- [x]") {
            if let Some(Block::Checkbox(lines)) = blocks.last_mut() {
                lines.push(Line {
                    index: char_index,
                    content: line.into(),
                });
            } else {
                blocks.push(Block::Checkbox(vec![Line {
                    index: char_index,
                    content: line.into(),
                }]));
            };
        } else if let Some(Block::Text(text)) = blocks.last_mut() {
            text.push('\n');
            text.push_str(line);
        } else {
            blocks.push(Block::Text(line.into()));
        };
        char_index += line.len();
    }
    blocks
}

#[component]
fn DescriptionContent(description: String) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "p-4 rounded border whitespace-pre-wrap break-words {} {}",
        theme.bg_color_1, theme.border_color
    );
    rsx! {
        div {
            class: style,
            for block in parse_blocks(&description) {
                match block {
                    Block::Text(text) => rsx!{
                        p { {text} }
                    },
                    Block::Bullet(lines) => rsx!{
                        ul {
                            for line in lines {
                                li { {line.content} }
                            }
                        }
                    },
                    Block::Checkbox(lines) => rsx!{
                        ul {
                            for line in lines {
                                li { {line.content} }
                            }
                        }
                    },
                }
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
