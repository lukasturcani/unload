use crate::color_picker::{self, SelectingColorPicker};
use crate::styles;
use dioxus::prelude::*;
use itertools::Itertools;
use reqwest::Url;
use shared_models::BoardName;
use shared_models::{Color, TagEntry, TagId};
use std::str::FromStr;

struct TagsUrl(reqwest::Url);

#[derive(Default)]
struct TagEntries(Vec<TagEntry>);

#[component]
pub fn ArchivedTags(board_name: BoardName) -> Element {
    let url = use_signal(|| {
        #[cfg(debug_assertions)]
        let url = Url::from_str("http://localhost:8080").unwrap();
        #[cfg(not(debug_assertions))]
        let url = Url::from_str("https://unload.fly.dev").unwrap();
        TagsUrl(url.join(&format!("/api/boards/{}/", board_name)).unwrap())
    });
    let tags = use_signal(TagEntries::default);
    let nav = use_navigator();
    use_future(move || async move {
        let url = &url.read().0;
        get_tags(tags, url).await;
    });
    let read_tags = tags.read();
    rsx! {
        div {
            class: "
                w-screen h-dvh
                bg-gray-900
                flex flex-col
            ",
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
                            for tag in read_tags
                                .0
                                .iter()
                                .sorted_by_key(|tag| tag.name.to_lowercase())
                            {
                                TagRow {
                                    key: "{tag.id}",
                                    tag: tag.clone(),
                                    url,
                                    tags,
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
                    onclick: move |_| {
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
    }
}

#[component]
fn TagRow(tag: TagEntry, url: Signal<TagsUrl>, tags: Signal<TagEntries>) -> Element {
    let mut editing_color_signal = use_signal(|| false);
    let editing_color = editing_color_signal();

    let mut name_signal = use_signal(|| None::<String>);
    let name = name_signal();

    rsx! {
        tr {
            class: "bg-gray-800 sm:hover:bg-gray-600",
            td {
                class: "p-3",
                if editing_color {
                    SelectingColorPicker {
                        default_color: tag.color,
                        on_pick_color: move |color| {
                            editing_color_signal.set(false);
                            spawn(
                                set_tag_color(tags, url, tag.id, color)
                            );
                        },
                    }

                } else {
                    div {
                        class: "flex flex-row gap-1",
                        div {
                            class: "w-8 h-8 rounded cursor-pointer {color_picker::bg_class(&tag.color)}",
                            onclick: move |_| editing_color_signal.set(true),
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "w-4 h-4",
                            onclick: move |_| editing_color_signal.set(true),
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                            }
                        }
                    }
                }
            }
            td {
                class: "p-3",
                if let Some(name_value) = name {
                    input {
                        r#type: "text",
                        value: "{name_value}",
                        class: "bg-inherit rounded text-sm",
                        oninput: move |event| name_signal.set(Some(event.data.value())),
                        onfocusout:  move |_| {
                            name_signal.set(None);
                            set_tag_name(tags, url, tag.id, name_value.clone())
                        },
                    }
                } else {
                    div {
                        class: "flex flex-row gap-1",
                        p {
                            onclick: {
                                let tag_name = tag.name.clone();
                                move |_| name_signal.set(Some(tag_name.clone()))
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
                            onclick: move |_| name_signal.set(Some(tag.name.clone())),
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                            }
                        }
                    }
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
                            onclick: move |_| set_tag_archived(tags, url, tag.id),
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m6 4.125 2.25 2.25m0 0 2.25 2.25M12 13.875l2.25-2.25M12 13.875l-2.25 2.25M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z",
                            }
                        }
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "w-6 h-6 cursor-pointer text-red-600",
                            onclick: move |_| delete_tag(tags, url, tag.id),
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

async fn set_tag_color(
    tags: Signal<TagEntries>,
    url: Signal<TagsUrl>,
    tag_id: TagId,
    color: Color,
) {
    let url = &url.read().0;
    let _ = send_set_tag_color_request(url, tag_id, color).await;
    get_tags(tags, url).await;
}

async fn send_set_tag_color_request(
    url: &Url,
    tag_id: TagId,
    color: Color,
) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tags/{}/color", tag_id))?;
    Ok(reqwest::Client::new()
        .put(url)
        .json(&color)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn set_tag_name(tags: Signal<TagEntries>, url: Signal<TagsUrl>, tag_id: TagId, name: String) {
    let url = &url.read().0;
    let _ = send_set_tag_name_request(url, tag_id, name).await;
    get_tags(tags, url).await;
}

async fn send_set_tag_name_request(
    url: &Url,
    tag_id: TagId,
    name: String,
) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tags/{}/name", tag_id))?;
    Ok(reqwest::Client::new()
        .put(url)
        .json(&name)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn delete_tag(tags: Signal<TagEntries>, url: Signal<TagsUrl>, tag_id: TagId) {
    let url = &url.read().0;
    let _ = send_delete_tag_request(url, tag_id).await;
    get_tags(tags, url).await;
}

async fn send_delete_tag_request(url: &Url, tag_id: TagId) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tags/{}", tag_id))?;
    Ok(reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn set_tag_archived(tags: Signal<TagEntries>, url: Signal<TagsUrl>, tag_id: TagId) {
    let url = &url.read().0;
    let _ = send_set_tag_archived_request(url, tag_id).await;
    get_tags(tags, url).await;
}

async fn send_set_tag_archived_request(url: &Url, tag_id: TagId) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tags/{}/archived", tag_id))?;
    Ok(reqwest::Client::new()
        .put(url)
        .json(&false)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn get_tags(mut tags: Signal<TagEntries>, url: &Url) {
    if let Ok(result) = send_get_tags_request(url).await {
        tags.write().0 = result;
    }
}

async fn send_get_tags_request(url: &Url) -> Result<Vec<TagEntry>, anyhow::Error> {
    let url = url.join("archive/tags")?;
    Ok(reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<TagEntry>>()
        .await?)
}
