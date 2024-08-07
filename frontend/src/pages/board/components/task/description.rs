use dioxus::prelude::*;
use reqwest::Client;
use shared_models::TaskId;

use crate::{
    components::{
        form::{CancelButton, ConfirmButton},
        icons::{BulletsIcon, CheckboxIcon, EditIcon},
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
    let mut enter_pressed = use_signal(|| false);
    rsx! {
        form {
            "aria-label": "update description",
            class: "flex flex-col gap-2",
            onsubmit: move |event| {
                let description = event.values()["Description"].as_value();
                spawn_forever(set_task_description(board_signals, task_id, description));
                editing.set(false);
            },
            div {
                class: format!(
                    "flex flex-col gap-2 border p-2 rounded-lg {}",
                    theme.border_color,
                ),
                div {
                    class: "flex flex-row justify-center items-center gap-2",
                    button {
                        class: "group",
                        prevent_default: "onclick",
                        onclick: move |_| insert_string(task_id, "\n* "),
                        div {
                            class: "relative",
                            div { class: "size-6", BulletsIcon {} }
                            Tooltip { content: "Bullet Points" }
                        }
                    }
                    button {
                        class: "group",
                        prevent_default: "onclick",
                        onclick: move |_| insert_string(task_id, "\n- [ ] "),
                        div {
                            class: "relative",
                            div { class: "size-6", CheckboxIcon {} }
                            Tooltip { content: "Task List" }
                        }
                    }
                }
                textarea {
                    id: "task-{task_id}-description-input",
                    onmounted: move |event| async move {
                        let _ = event.set_focus(true).await;
                    },
                    onkeydown: move |event| enter_pressed.set(event.data().key() == Key::Enter),
                    oninput: move |_| {
                        if enter_pressed() {
                            spawn(edit_description(task_id, enter_pressed));
                        }
                    },
                    rows: 8.max(description.lines().count() as i64),
                    class: "p-2.5 {style}",
                    name: "Description",
                    required: false,
                    value: description,
                }
            }
            div {
                class: "flex flex-row gap-2 items-center justify-center",
                ConfirmButton { label: "set description" }
                CancelButton { label: "cancel description update", editing }
            }
        }
    }
}

async fn insert_string(task_id: TaskId, string: impl AsRef<str>) {
    let mut text_data = eval(&format!(
        r#"
            let element = document.getElementById("task-{task_id}-description-input");
            dioxus.send([element.value, element.selectionStart]);
        "#,
    ));
    let text_data = &text_data.recv().await.unwrap();
    let [content, position] = &text_data.as_array().unwrap()[..] else {
        panic!("impossible");
    };
    let mut content = String::from(content.as_str().unwrap());
    content.insert_str(position.as_u64().unwrap() as usize, string.as_ref());
    let edit = eval(&format!(
        r#"
            let element = document.getElementById("task-{task_id}-description-input");
            let selectionStart = element.selectionStart + {};
            let content = await dioxus.recv();
            element.value = content;
            element.selectionStart = selectionStart;
            element.selectionEnd = selectionStart;
            element.focus();
        "#,
        string.as_ref().len(),
    ));
    edit.send(content.into()).unwrap();
}

async fn edit_description(task_id: TaskId, mut enter_pressed: Signal<bool>) {
    let mut text_data = eval(&format!(
        r#"
            let element = document.getElementById("task-{task_id}-description-input");
            dioxus.send([element.value, element.selectionStart]);
        "#,
    ));
    let text_data = &text_data.recv().await.unwrap();
    let [content, position] = &text_data.as_array().unwrap()[..] else {
        panic!("impossible");
    };
    let content = content.as_str().unwrap();
    let position = position.as_u64().unwrap() as usize;
    let start = content[..position - 1].rfind('\n').map_or(0, |i| i + 1);
    let line = &content[start..position - 1];
    if line.starts_with("- [ ]") || line.starts_with("- [x]") {
        let mut content = String::from(content);
        if line == "- [ ] " {
            let edit = eval(&format!(
                r#"
                    let element = document.getElementById("task-{task_id}-description-input");
                    let selectionStart = element.selectionStart - 7;
                    let content = await dioxus.recv();
                    element.value = content;
                    element.selectionStart = selectionStart;
                    element.selectionEnd = selectionStart;
                "#,
            ));
            content.drain(start..position);
            edit.send(content.into()).unwrap();
        } else {
            let edit = eval(&format!(
                r#"
                    let element = document.getElementById("task-{task_id}-description-input");
                    let selectionStart = element.selectionStart + 6;
                    let content = await dioxus.recv();
                    element.value = content;
                    element.selectionStart = selectionStart;
                    element.selectionEnd = selectionStart;
                "#,
            ));
            content.insert_str(position, "- [ ] ");
            edit.send(content.into()).unwrap();
        }
    } else if line.starts_with('*') {
        let mut content = String::from(content);
        if line == "* " {
            let edit = eval(&format!(
                r#"
                    let element = document.getElementById("task-{task_id}-description-input");
                    let selectionStart = element.selectionStart - 3;
                    let content = await dioxus.recv();
                    element.value = content;
                    element.selectionStart = selectionStart;
                    element.selectionEnd = selectionStart;
                "#,
            ));
            content.drain(start..position);
            edit.send(content.into()).unwrap();
        } else {
            let edit = eval(&format!(
                r#"
                    let element = document.getElementById("task-{task_id}-description-input");
                    let selectionStart = element.selectionStart + 2;
                    let content = await dioxus.recv();
                    element.value = content;
                    element.selectionStart = selectionStart;
                    element.selectionEnd = selectionStart;
                "#,
            ));
            content.insert_str(position, "* ");
            edit.send(content.into()).unwrap();
        }
    }
    enter_pressed.set(false);
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
            DescriptionContent { task_id, description }
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

#[derive(Clone, Eq, PartialEq)]
struct Line {
    index: usize,
    content: String,
}

enum Block {
    Text(String),
    Bullet(Vec<String>),
    Checkbox(Vec<Line>),
}

fn parse_blocks(description: &str) -> Vec<Block> {
    let mut blocks = Vec::<Block>::new();
    let mut line_index = 0;
    for line in description.lines() {
        if line.starts_with("* ") {
            if let Some(Block::Bullet(lines)) = blocks.last_mut() {
                lines.push(line.into());
            } else {
                blocks.push(Block::Bullet(vec![line.into()]));
            };
        } else if line.starts_with("- [ ]") || line.starts_with("- [x]") {
            if let Some(Block::Checkbox(lines)) = blocks.last_mut() {
                lines.push(Line {
                    index: line_index,
                    content: line.into(),
                });
            } else {
                blocks.push(Block::Checkbox(vec![Line {
                    index: line_index,
                    content: line.into(),
                }]));
            };
        } else if let Some(Block::Text(text)) = blocks.last_mut() {
            text.push('\n');
            text.push_str(line);
        } else {
            blocks.push(Block::Text(line.into()));
        };
        line_index += line.len() + 1;
    }
    blocks
}

#[component]
fn DescriptionContent(task_id: TaskId, description: String) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "p-4 rounded border whitespace-pre-wrap break-words {} {}",
        theme.bg_color_1, theme.border_color
    );
    let description_ = Signal::new(description.clone());
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
                            class:" list-disc list-inside",
                            for line in lines {
                                Bullet { line }
                            }
                        }
                    },
                    Block::Checkbox(lines) => rsx!{
                        ul {
                            for line in lines {
                                Checkbox { task_id, line, description: description_ }
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
fn Checkbox(task_id: TaskId, description: Signal<String>, line: Line) -> Element {
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
