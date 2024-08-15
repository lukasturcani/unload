use dioxus::prelude::*;
use reqwest::Client;
use shared_models::{TagData, TagId, TaskId};

use crate::{
    components::{
        color_picker::ColorPicker,
        form::{CancelButton, ConfirmButton},
        input::TextInput,
    },
    model::UnloadUrl,
    pages::board::{
        components::assignment_list::{
            AssignmentList, AssignmentListItem, ShowSelectionListFormButton,
        },
        model::{Board, Tags},
        requests::{self, BoardSignals},
    },
};

#[component]
pub fn TagSelection(task_id: TaskId, tags: Signal<Vec<TagId>>) -> Element {
    let tag_data = use_context::<Signal<Tags>>();
    let tag_data = &tag_data.read().0;
    let mut unassigned = Vec::with_capacity(tag_data.len() - tags.len());
    let tags = tags.read();
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
fn AddTagListForm(task_id: TaskId, show_form: Signal<bool>) -> Element {
    let board_signals = BoardSignals::default();
    rsx! {
        li {
            form {
                id: "task-{task_id}-new-tag-form",
                "aria-label": "add tag",
                class: "flex flex-col gap-2 p-2",
                onsubmit: move |event| {
                    let values = event.values();
                    let name = values["Name"].as_value();
                    let color = serde_json::from_str(
                        &values["color-picker"].as_value()
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
