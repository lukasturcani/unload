use crate::styles;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use reqwest::Url;
use shared_models::{BoardName, TaskEntry};
use std::str::FromStr;

struct TagsUrl(Url);

#[component]
pub fn TaskArchive(cx: Scope, board_name: BoardName) -> Element {
    use_shared_state_provider(cx, || {
        #[cfg(debug_assertions)]
        let url = Url::from_str("http://localhost:8080").unwrap();
        #[cfg(not(debug_assertions))]
        let url = Url::from_str("https://unload.fly.dev").unwrap();
        TagsUrl(url.join(&format!("/api/boards/{}/", board_name)).unwrap())
    });

    let nav = use_navigator(cx);
    let tasks = use_state(cx, Vec::new);
    let url = use_shared_state::<TagsUrl>(cx).unwrap();
    use_future(cx, (), {
        let url = url.clone();
        let tasks = tasks.clone();
        |_| async move {
            let url = url.read();
            get_tasks(&url.0, tasks).await
        }
    });
    cx.render(rsx! {
        div {
            class: "
                w-screen h-screen
                bg-gray-900
                flex flex-col
            ",
            ul {
                class: "
                    grow w-full p-4 overflow-auto
                    divide-y divide-gray-700
                ",
               tasks
                    .iter()
                    .map(|task| rsx!(Task{ key: "{task.id}", task: task.clone() }))
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

#[component]
fn Task(cx: Scope, task: TaskEntry) -> Element {
    let expanded = use_state(cx, || false);
    cx.render(rsx! {
        li {
            class: "p-2.5 active:bg-gray-600 sm:hover:bg-gray-600",
            onclick: |_| expanded.set(!**expanded),
            p {
                class: "text-white",
                task.title.clone()
            }
            if **expanded {rsx! {
                p {
                    class: "text-gray-400",
                    task.description.clone()
                }
            }}
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
                onclick: |_| unarchive_task(tags.clone(), url.clone(), tag.id),
                path {
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    d: "m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m6 4.125 2.25 2.25m0 0 2.25 2.25M12 13.875l2.25-2.25M12 13.875l-2.25 2.25M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z",
                }
            }
        }
    })
}

async fn get_tasks(url: &Url, tasks: UseState<Vec<TaskEntry>>) {
    if let Ok(result) = send_get_tasks_request(url).await {
        tasks.set(result);
    }
}

async fn send_get_tasks_request(url: &Url) -> Result<Vec<TaskEntry>, anyhow::Error> {
    Ok(reqwest::get(url.join("archive/tasks").unwrap())
        .await?
        .json::<Vec<TaskEntry>>()
        .await?)
}

async fn unarchive_task(
    tags: UseSharedState<TagEntries>,
    url: UseSharedState<TagsUrl>,
    tag_id: TagId,
) {
    let url = &url.read().0;
    let _ = send_unarchive_task_request(url, tag_id).await;
    get_tags(tags, url).await;
}

async fn send_unarchive_task_request(url: &Url, tag_id: TagId) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tasks/{}/archived", tag_id))?;
    Ok(reqwest::Client::new()
        .put(url)
        .json(&false)
        .send()
        .await?
        .json::<()>()
        .await?)
}
