use std::collections::HashSet;

use dioxus::prelude::*;
use itertools::Itertools;
use reqwest::Client;
use shared_models::{TagData, TagId, TaskId};

use crate::{color_picker::ColorPicker, model::Model, requests, styles};

#[component]
pub fn TagSearch(
    cx: Scope,
    task_id: TaskId,
    ul_style: &'static str,
    hover_style: &'static str,
    text_input_style: &'static str,
) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    if model.read().tag_search_created_tag.is_some() {
        if let Some((tag_id, _)) = model.write().tag_search_created_tag.take() {
            spawn(add_task_tag(model.clone(), *task_id, tag_id));
        }
    }
    let read_model = model.read();
    let tags: HashSet<_> = read_model.tasks[task_id]
        .tags
        .iter()
        .map(|id| *id)
        .collect();
    let show_add_tag_button = use_state(cx, || true);
    let new_tag = use_state(cx, String::new);
    cx.render(rsx! {
        ul {
            class: "text-sm text-gray-200 z-10 rounded-lg shadow {ul_style}",
            rsx!{
                for (tag_id, tag) in read_model
                    .tags
                    .iter()
                    .filter(|(id, _)| !tags.contains(id))
                    .sorted_by_key(|(_, tag)| tag.name.to_lowercase())
                {rsx!{
                    li {
                        key: "{tag_id}",
                        button {
                            r#type: "button",
                            class: "
                                text-left w-full px-4 py-2
                                {hover_style} hover:text-white
                            ",
                            prevent_default: "onmousedown",
                            onmousedown: |_| {},
                            onclick: {
                                let task_id = *task_id;
                                let tag_id = *tag_id;
                                move |event| {
                                    event.stop_propagation();
                                    add_task_tag(model.clone(), task_id, tag_id)
                                }
                            },
                            tag.name.clone(),
                        }
                    },
                }}
            }
            li {
                key: "add tag",
                if **show_add_tag_button {rsx! {
                    button {
                        r#type: "button",
                        class: "
                            text-left w-full px-4 py-2
                            {hover_style}
                            font-medium text-blue-500 hover:underline
                        ",
                        prevent_default: "onmousedown",
                        onmousedown: |_| {},
                        onclick: |_| {
                            show_add_tag_button.set(false);
                        },
                        "Add Tag"
                    }
                }} else {rsx! {
                    div {
                        class: "p-2",
                        div {
                            class: "flex flex-col gap-2 p-2",
                            input {
                                class: "{styles::TEXT_INPUT} {text_input_style}",
                                r#type: "text",
                                placeholder: "Tag",
                                value: "{new_tag}",
                                oninput: |event| {
                                    new_tag.set(event.value.clone())
                                },
                            }
                            ColorPicker {
                                on_pick_color: |color| {
                                    show_add_tag_button.set(true);
                                    if new_tag.trim().is_empty() {
                                        return;
                                    }
                                    cx.spawn(create_tag(
                                        model.clone(),
                                        TagData {
                                            name: new_tag.make_mut().drain(..).collect(),
                                            color
                                        },
                                    ));
                                },
                            }
                        }
                    }
                }}
            }
        }
    })
}

async fn create_tag(model: UseSharedState<Model>, tag_data: TagData) {
    if let Ok(tag_data) = requests::create_tag(model.clone(), tag_data).await {
        requests::board(model.clone()).await;
        model.write().tag_search_created_tag = Some(tag_data);
    }
}

async fn add_task_tag(model: UseSharedState<Model>, task_id: TaskId, tag_id: TagId) {
    if send_add_task_tag_request(model.clone(), task_id, tag_id)
        .await
        .is_ok()
    {
        requests::board(model.clone()).await;
    }
}

async fn send_add_task_tag_request(
    model: UseSharedState<Model>,
    task_id: TaskId,
    tag_id: TagId,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tasks/{}/tags",
            model.board_name, task_id
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
