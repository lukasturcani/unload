use std::collections::HashSet;

use dioxus::prelude::*;
use itertools::Itertools;
use reqwest::Client;
use shared_models::{TagData, TagId, TaskId};

use crate::{color_picker::ColorPicker, model::Model, requests, styles};

#[component]
pub fn TagSearch<'a>(
    cx: Scope<'a>,
    id: &'a str,
    on_select_tag: EventHandler<'a, TagId>,
    on_remove_tag: EventHandler<'a, TagId>,
    initial_tags: Option<Vec<TagId>>,
    always_show_suggestions: Option<bool>,
    on_search_focus_in: Option<EventHandler<'a>>,
    on_search_focus_out: Option<EventHandler<'a>>,
) -> Element<'a> {
    let always_show_suggestions = always_show_suggestions.unwrap_or(false);
    let model = use_shared_state::<Model>(cx).unwrap();
    let show_color_picker = use_state(cx, || false);
    let has_input_focus = use_state(cx, || false);
    let search_input = use_state(cx, String::default);
    let selected = {
        let model = model.read();
        use_ref(cx, || {
            if let Some(tags) = initial_tags {
                tags.iter()
                    .map(|id| (*id, model.tags[id].name.clone()))
                    .collect()
            } else {
                Vec::new()
            }
        })
    };
    if model.read().tag_search_created_tag.is_some() {
        if let Some(tag) = model.write().tag_search_created_tag.take() {
            on_select_tag.call(tag.0);
            selected.write().push(tag);
        }
    }
    let tag_data = if (**has_input_focus || always_show_suggestions) && !**show_color_picker {
        let model = model.read();
        let selected = selected.read();
        let tags: Vec<_> = model
            .tags
            .iter()
            .filter(|(id, tag)| {
                tag.name.contains(&**search_input)
                    && selected.iter().all(|(selected_id, _)| selected_id != *id)
            })
            .map(|(id, tag)| (*id, tag.name.clone()))
            .collect();
        let show_add_tag_button = !search_input.is_empty()
            && model
                .tags
                .iter()
                .all(|(_, tag)| tag.name != search_input.trim());
        Some((tags, show_add_tag_button))
    } else {
        None
    };
    cx.render(rsx! {
        label {
            r#for: *id,
            class: styles::TEXT_INPUT_LABEL,
            "Tags"
        },
        div {
            class: "relative",
            div {
                class: "absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none",
                svg {
                    class: "w-4 h-4 text-gray-400",
                    "aria-hidden": "true",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none" ,
                    "viewBox": "0 0 20 20",
                    path {
                        d: "m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z",
                        stroke: "currentColor",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        "stroke-width", "2",
                    },
                },
            },
            input {
                r#type: "search",
                id: *id,
                class: "{styles::TEXT_INPUT} ps-10",
                placeholder: "Search",
                autocomplete: "off",
                value: "{search_input}",
                onfocusin: move |_| {
                    if let Some(handler) = on_search_focus_in {
                        handler.call(());
                    }
                    has_input_focus.set(true);
                },
                onfocusout: move |_| {
                    if let Some(handler) = on_search_focus_out {
                        handler.call(());
                    }
                    has_input_focus.set(false)
                },
                oninput: |event| search_input.set(event.data.value.clone())
            },
        },
        if **show_color_picker {rsx!{
            div {
                class: "mt-2 z-10 divide-y divide-gray-100 rounded-lg shadow bg-gray-700 p-4",
                ColorPicker {
                    on_pick_color: |color| {
                        show_color_picker.set(false);
                        cx.spawn(create_tag(
                            model.clone(),
                            TagData {
                                name: search_input.make_mut().drain(..).collect(),
                                color
                            },
                        ));
                    },
                }
            }
        }}
        if let Some((tags, show_add_tag_button)) = tag_data {rsx!{
            if !tags.is_empty() || show_add_tag_button {rsx!{
                div {
                    class: "mt-2 z-10 divide-y divide-gray-100 rounded-lg shadow bg-gray-700",
                    ul {
                        class: "py-2 text-sm text-gray-200",
                        rsx!{
                            for tag in tags {rsx!{
                                li {
                                    key: "{tag.0}",
                                    button {
                                        r#type: "button",
                                        class: "block text-left w-full px-4 py-2 hover:bg-gray-600 hover:text-white focus:border-blue-500",
                                        prevent_default: "onmousedown",
                                        onmousedown: |_| {},
                                        onclick: move |_| {
                                            search_input.set(String::new());
                                            selected.write().push(tag.clone());
                                            on_select_tag.call(tag.0);
                                        },
                                        tag.1.clone(),
                                    }
                                },
                            }}
                        }
                        if show_add_tag_button {rsx!{
                            li {
                                key: "add tag",
                                button {
                                    r#type: "button",
                                    class: "block text-left w-full px-4 py-2
                                        hover:bg-gray-600
                                        font-medium text-blue-500 hover:underline",
                                    prevent_default: "onmousedown",
                                    onmousedown: |_| {},
                                    onclick: |_| show_color_picker.set(true),
                                    "Add Tag"
                                }
                            },
                        }}
                    }
                }
            }}
            else {rsx!{
                div {
                    class: "mt-2 z-10 divide-y divide-gray-100 rounded-lg shadow bg-gray-700 focus:border-blue-500",
                    ul {
                        class: "py-2 text-sm text-gray-200 focus:border-blue-500",
                        li {
                            class: "italic text-gray-400 block text-left w-full px-4 py-2",
                            prevent_default: "onmousedown",
                            onmousedown: |_| {},
                            "No matches"
                        },
                    }
                }
            }}
        }}
        div {
            class: "mt-2",
            for tag in selected.read().iter().map(|x| x.clone()) {rsx!{
                span {
                    class: "inline-flex items-center px-2 py-1 me-2 mt-2 text-sm font-medium rounded bg-gray-700 text-gray-300",
                    tag.1.clone(),
                    button {
                        r#type: "button",
                        class: "inline-flex items-center p-1 ms-2 text-sm text-gray-400 bg-transparent rounded-sm hover:bg-gray-600 hover:text-gray-300",
                        "aria-label": "Remove",
                        onclick: move |_| {
                            selected.write().retain(|this| this.0 != tag.0);
                            on_remove_tag.call(tag.0);
                        },
                        svg {
                            class: "w-2 h-2",
                            "aria-hidden": "true",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 14 14",
                            path {
                                stroke: "currentColor",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                "stroke-width": "2",
                                d: "m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6",
                            },
                        },
                        span {
                            class: "sr-only",
                            "Remove badge",
                        },
                    },
                },
            }},
        },
    })
}

#[component]
pub fn CompactTagSearch(
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
