use crate::color_picker::{self, ColorPicker};
use crate::styles;
use crate::{model::Model, requests};
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use itertools::Itertools;
use shared_models::BoardName;
use shared_models::{Color, TagEntry, TagId};
use tokio::join;

enum Column {
    Color,
    Name,
}

#[component]
pub fn Tags(cx: Scope, board_name: BoardName) -> Element {
    let nav = use_navigator(cx);
    let model = use_shared_state::<Model>(cx).unwrap();
    let archived_tags = use_ref(cx, Vec::new);
    if &model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    use_future(cx, (), |_| requests::board_tags(model.clone()));
    use_future(cx, (), |_| {
        get_archived_tags(model.clone(), archived_tags.clone())
    });
    let tags = &model.read().tags;
    let edit_field = use_state(cx, || None::<(usize, Column)>);
    let color = use_state(cx, || Color::Black);
    let name = use_state(cx, String::new);
    cx.render(rsx! {
        div {
            class: "
                w-screen h-screen
                bg-gray-900
                flex flex-col
            ",
            onclick: |_|  edit_field.set(None),
            div {
                class: "grow w-full p-4 overflow-auto",
                div {
                    class: "overflow-scroll border border-gray-900 w-full rounded-lg",
                    table {
                        class: "w-full text-sm text-left text-gray-400",
                        thead {
                            class: "text-xs uppercase bg-gray-700 text-gray-400",
                            tr {
                                th {
                                    scope: "col",
                                    class: "p-3",
                                    "Color"
                                }
                                th {
                                    scope: "col",
                                    class: "p-3",
                                    "Tag"
                                }
                                th {
                                    scope: "col",
                                    class: "p-3",
                                }
                            }
                        }
                        tbody {
                            class: "divide-y divide-gray-700",
                            for (row_index, (tag_id, tag)) in tags.iter().sorted_by_key(|x| x.1.name.to_lowercase()).enumerate() {
                                tr {
                                    key: "{tag_id}",
                                    class: "bg-gray-800 sm:hover:bg-gray-600",
                                    td {
                                        class: "p-3",
                                        match **edit_field {
                                            Some((edit_row, Column::Color)) if edit_row == row_index => rsx!{
                                                ColorPicker {
                                                    on_pick_color: {
                                                        let tag_id = *tag_id;
                                                        move |color| {
                                                            edit_field.set(None);
                                                            cx.spawn(set_tag_color(model.clone(), tag_id, color));
                                                        }
                                                    },
                                                }
                                            },
                                            _ => rsx!{
                                                div {
                                                    class: "flex flex-row gap-1",
                                                    div {
                                                        class: "w-8 h-8 rounded cursor-pointer {color_picker::bg_class(&tag.color)}",
                                                        onclick: {
                                                            let tag_id = *tag_id;
                                                            move |_| {
                                                                color.set(model.read().tags[&tag_id].color);
                                                                edit_field.set(Some((row_index, Column::Color)));
                                                            }
                                                        },
                                                    },
                                                    svg {
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        fill: "none",
                                                        "viewBox": "0 0 24 24",
                                                        "stroke-width": "1.5",
                                                        stroke: "currentColor",
                                                        class: "w-4 h-4",
                                                        onclick: {
                                                            let tag_id = *tag_id;
                                                            move |_| {
                                                                color.set(model.read().tags[&tag_id].color);
                                                                edit_field.set(Some((row_index, Column::Color)));
                                                            }
                                                        },
                                                        path {
                                                            "stroke-linecap": "round",
                                                            "stroke-linejoin": "round",
                                                            d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                                                        }
                                                    }
                                                }
                                            },
                                        }
                                    }
                                    td {
                                        class: "p-3",
                                        match **edit_field {
                                            Some((edit_row, Column::Name)) if edit_row == row_index => rsx! {
                                                input {
                                                    r#type: "text",
                                                    value: "{name}",
                                                    class: "bg-inherit rounded text-sm",
                                                    oninput: |event| name.set(event.data.value.clone()),
                                                    onfocusout: {
                                                        let tag_id = *tag_id;
                                                        move |_| {
                                                            edit_field.set(None);
                                                            set_tag_name(model.clone(), tag_id, name.to_string())
                                                        }
                                                    },
                                                }
                                            },
                                            _ => rsx!{
                                                div {
                                                    class: "flex flex-row gap-1",
                                                    p {
                                                        onclick: {
                                                            let tag_id = *tag_id;
                                                            move |_| {
                                                                name.set(model.read().tags[&tag_id].name.clone());
                                                                edit_field.set(Some((row_index, Column::Name)));
                                                            }
                                                        },
                                                        "{tag.name}"
                                                    }
                                                    svg {
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        fill: "none",
                                                        "viewBox": "0 0 24 24",
                                                        "stroke-width": "1.5",
                                                        stroke: "currentColor",
                                                        class: "w-4 h-4",
                                                        onclick: {
                                                            let tag_id = *tag_id;
                                                            move |_| {
                                                                name.set(model.read().tags[&tag_id].name.clone());
                                                                edit_field.set(Some((row_index, Column::Name)));
                                                            }
                                                        },
                                                        path {
                                                            "stroke-linecap": "round",
                                                            "stroke-linejoin": "round",
                                                            d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                                                        }
                                                    }
                                                }
                                            },
                                        }
                                    }
                                    td {
                                        class: "p-3",
                                        div {
                                            class: "grid grid-rows-1 place-items-end",
                                            div {
                                                class: "flex flex-row gap-2",
                                                svg {
                                                    xmlns: "http://www.w3.org/2000/svg" ,
                                                    fill: "none",
                                                    "viewBox": "0 0 24 24",
                                                    "stroke-width": "1.5",
                                                    stroke: "currentColor",
                                                    class: "
                                                        w-6 h-6 cursor-pointer text-gray-400
                                                        sm:hover:text-blue-500 active:text-blue-500
                                                    ",
                                                    onclick: {
                                                        let tag_id = *tag_id;
                                                        move |event| {
                                                            event.stop_propagation();
                                                            set_tag_archived(
                                                                model.clone(),
                                                                archived_tags.clone(),
                                                                tag_id,
                                                                true,
                                                            )
                                                        }
                                                    },
                                                    path {
                                                        "stroke-linecap": "round",
                                                        "stroke-linejoin": "round",
                                                        d: "m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z",
                                                    }
                                                }
                                                svg {
                                                    xmlns: "http://www.w3.org/2000/svg",
                                                    fill: "none",
                                                    "viewBox": "0 0 24 24",
                                                    "stroke-width": "1.5",
                                                    stroke: "currentColor",
                                                    class: "w-6 h-6 cursor-pointer text-red-600",
                                                    onclick: {
                                                        let tag_id = *tag_id;
                                                        move |_| {
                                                            delete_tag(model.clone(), tag_id)
                                                        }
                                                    },
                                                    path {
                                                        "stroke-linecap": "round",
                                                        "stroke-linejoin": "round",
                                                        d: "m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0",
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "grow w-full p-4 overflow-auto",
                div {
                    class: "overflow-scroll border border-gray-900 w-full rounded-lg",
                    table {
                        class: "w-full text-sm text-left text-gray-400",
                        thead {
                            class: "text-xs uppercase bg-gray-700 text-gray-400",
                            tr {
                                th {
                                    scope: "col",
                                    class: "p-3",
                                    "Color"
                                }
                                th {
                                    scope: "col",
                                    class: "p-3",
                                    "Tag"
                                }
                                th {
                                    scope: "col",
                                    class: "p-3",
                                }
                            }
                        }
                        tbody {
                            class: "divide-y divide-gray-700",
                            for (row_index, tag) in archived_tags
                                .read()
                                .iter()
                                .sorted_by_key(|tag| tag.name.to_lowercase())
                                .enumerate()
                            {
                                tr {
                                    key: "{tag.id}",
                                    class: "bg-gray-800 sm:hover:bg-gray-600",
                                    td {
                                        class: "p-3",
                                        match **edit_field {
                                            Some((edit_row, Column::Color)) if edit_row == row_index => rsx!{
                                                ColorPicker {
                                                    on_pick_color: {
                                                        let tag_id = tag.id;
                                                        move |color| {
                                                            edit_field.set(None);
                                                            cx.spawn(set_tag_color(model.clone(), tag_id, color));
                                                        }
                                                    },
                                                }
                                            },
                                            _ => rsx!{
                                                div {
                                                    class: "flex flex-row gap-1",
                                                    div {
                                                        class: "w-8 h-8 rounded cursor-pointer {color_picker::bg_class(&tag.color)}",
                                                        onclick: {
                                                            let tag_id = tag.id;
                                                            move |_| {
                                                                color.set(model.read().tags[&tag_id].color);
                                                                edit_field.set(Some((row_index, Column::Color)));
                                                            }
                                                        },
                                                    },
                                                    svg {
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        fill: "none",
                                                        "viewBox": "0 0 24 24",
                                                        "stroke-width": "1.5",
                                                        stroke: "currentColor",
                                                        class: "w-4 h-4",
                                                        onclick: {
                                                            let tag_id = tag.id;
                                                            move |_| {
                                                                color.set(model.read().tags[&tag_id].color);
                                                                edit_field.set(Some((row_index, Column::Color)));
                                                            }
                                                        },
                                                        path {
                                                            "stroke-linecap": "round",
                                                            "stroke-linejoin": "round",
                                                            d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                                                        }
                                                    }
                                                }
                                            },
                                        }
                                    }
                                    td {
                                        class: "p-3",
                                        match **edit_field {
                                            Some((edit_row, Column::Name)) if edit_row == row_index => rsx! {
                                                input {
                                                    r#type: "text",
                                                    value: "{name}",
                                                    class: "bg-inherit rounded text-sm",
                                                    oninput: |event| name.set(event.data.value.clone()),
                                                    onfocusout: {
                                                        let tag_id = tag.id;
                                                        move |_| {
                                                            edit_field.set(None);
                                                            set_tag_name(model.clone(), tag_id, name.to_string())
                                                        }
                                                    },
                                                }
                                            },
                                            _ => rsx!{
                                                div {
                                                    class: "flex flex-row gap-1",
                                                    p {
                                                        onclick: {
                                                            let tag_id = tag.id;
                                                            move |_| {
                                                                name.set(model.read().tags[&tag_id].name.clone());
                                                                edit_field.set(Some((row_index, Column::Name)));
                                                            }
                                                        },
                                                        "{tag.name}"
                                                    }
                                                    svg {
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        fill: "none",
                                                        "viewBox": "0 0 24 24",
                                                        "stroke-width": "1.5",
                                                        stroke: "currentColor",
                                                        class: "w-4 h-4",
                                                        onclick: {
                                                            let tag_id = tag.id;
                                                            move |_| {
                                                                name.set(model.read().tags[&tag_id].name.clone());
                                                                edit_field.set(Some((row_index, Column::Name)));
                                                            }
                                                        },
                                                        path {
                                                            "stroke-linecap": "round",
                                                            "stroke-linejoin": "round",
                                                            d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                                                        }
                                                    }
                                                }
                                            },
                                        }
                                    }
                                    td {
                                        class: "p-3",
                                        div {
                                            class: "grid grid-rows-1 place-items-end",
                                            div {
                                                class: "flex flex-row gap-2",
                                                svg {
                                                    xmlns: "http://www.w3.org/2000/svg" ,
                                                    fill: "none",
                                                    "viewBox": "0 0 24 24",
                                                    "stroke-width": "1.5",
                                                    stroke: "currentColor",
                                                    class: "
                                                        w-6 h-6 cursor-pointer text-gray-400
                                                        sm:hover:text-blue-500 active:text-blue-500
                                                    ",
                                                    onclick: {
                                                        let tag_id = tag.id;
                                                        move |event| {
                                                            event.stop_propagation();
                                                            set_tag_archived(
                                                                model.clone(),
                                                                archived_tags.clone(),
                                                                tag_id,
                                                                false,
                                                            )
                                                        }
                                                    },
                                                    path {
                                                        "stroke-linecap": "round",
                                                        "stroke-linejoin": "round",
                                                        d: "m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m6 4.125 2.25 2.25m0 0 2.25 2.25M12 13.875l2.25-2.25M12 13.875l-2.25 2.25M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z"
                                                    }
                                                }
                                                svg {
                                                    xmlns: "http://www.w3.org/2000/svg",
                                                    fill: "none",
                                                    "viewBox": "0 0 24 24",
                                                    "stroke-width": "1.5",
                                                    stroke: "currentColor",
                                                    class: "w-6 h-6 cursor-pointer text-red-600",
                                                    onclick: {
                                                        let tag_id = tag.id;
                                                        move |_| {
                                                            delete_tag(model.clone(), tag_id)
                                                        }
                                                    },
                                                    path {
                                                        "stroke-linecap": "round",
                                                        "stroke-linejoin": "round",
                                                        d: "m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0",
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

            }
            div {
                class: styles::BOTTOM_BAR,
                button {
                    r#type: "button" ,
                    class: styles::BOTTOM_BAR_BUTTON,
                    onclick: |_| {
                        nav.go_back();
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "
                            w-6 h-6 text-gray-400
                            group-active:text-blue-500
                            sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M15.75 19.5 8.25 12l7.5-7.5",
                        }
                    }
                }
            }
        }
    })
}

async fn set_tag_color(model: UseSharedState<Model>, tag_id: TagId, color: Color) {
    if send_set_tag_color_request(model.clone(), tag_id, color)
        .await
        .is_ok()
    {
        requests::board_tags(model).await;
    }
}

async fn send_set_tag_color_request(
    model: UseSharedState<Model>,
    tag_id: TagId,
    color: Color,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tags/{}/color",
            model.board_name, tag_id
        ))?
    };
    Ok(reqwest::Client::new()
        .put(url)
        .json(&color)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn set_tag_name(model: UseSharedState<Model>, tag_id: TagId, name: String) {
    if send_set_tag_name_request(model.clone(), tag_id, name)
        .await
        .is_ok()
    {
        requests::board_tags(model).await;
    }
}

async fn send_set_tag_name_request(
    model: UseSharedState<Model>,
    tag_id: TagId,
    name: String,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tags/{}/name",
            model.board_name, tag_id
        ))?
    };
    Ok(reqwest::Client::new()
        .put(url)
        .json(&name)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn delete_tag(model: UseSharedState<Model>, tag_id: TagId) {
    if send_delete_tag_request(model.clone(), tag_id).await.is_ok() {
        requests::board_tags(model).await;
    }
}

async fn send_delete_tag_request(
    model: UseSharedState<Model>,
    tag_id: TagId,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model
            .url
            .join(&format!("/api/boards/{}/tags/{}", model.board_name, tag_id))?
    };
    Ok(reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn set_tag_archived(
    model: UseSharedState<Model>,
    archived_tags: UseRef<Vec<TagEntry>>,
    tag_id: TagId,
    archived: bool,
) {
    if send_set_tag_archived_request(model.clone(), tag_id, archived)
        .await
        .is_ok()
    {
        join!(
            requests::board_tags(model.clone()),
            get_archived_tags(model, archived_tags),
        );
    }
}

async fn send_set_tag_archived_request(
    model: UseSharedState<Model>,
    tag_id: TagId,
    archived: bool,
) -> Result<(), anyhow::Error> {
    let url = {
        let model = model.read();
        model.url.join(&format!(
            "/api/boards/{}/tags/{}/archived",
            model.board_name, tag_id
        ))?
    };
    Ok(reqwest::Client::new()
        .put(url)
        .json(&archived)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn get_archived_tags(model: UseSharedState<Model>, archived_tags: UseRef<Vec<TagEntry>>) {
    if let Ok(result) = send_get_archived_tags_request(model.clone()).await {
        let mut archived_tags = archived_tags.write();
        archived_tags.clear();
        archived_tags.extend(result);
    }
}

async fn send_get_archived_tags_request(
    model: UseSharedState<Model>,
) -> Result<Vec<TagEntry>, anyhow::Error> {
    let url = {
        let model = model.read();
        model
            .url
            .join(&format!("/api/boards/{}/archive/tags", model.board_name))?
    };
    Ok(reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<TagEntry>>()
        .await?)
}
