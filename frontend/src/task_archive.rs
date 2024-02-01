use crate::styles;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use reqwest::Url;
use shared_models::{BoardName, TaskEntry};
use std::str::FromStr;

#[component]
pub fn TaskArchive(cx: Scope, board_name: BoardName) -> Element {
    let nav = use_navigator(cx);
    let tasks = use_state(cx, Vec::new);
    use_future(cx, (), |_| get_tasks(board_name.clone(), tasks.clone()));
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
                    .map(|task| {
                        rsx! {
                            li {
                                key: "{task.id}",
                                class: "
                                    text-white
                                    p-2.5
                                    sm:hover:bg-gray-600
                                ",
                                task.title.clone()
                            }
                        }
                    })
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

async fn get_tasks(board_name: BoardName, tasks: UseState<Vec<TaskEntry>>) {
    if let Ok(result) = send_get_tasks_request(board_name).await {
        tasks.set(result);
    }
}

async fn send_get_tasks_request(board_name: BoardName) -> Result<Vec<TaskEntry>, anyhow::Error> {
    #[cfg(debug_assertions)]
    let url = Url::from_str("http://localhost:8080").unwrap();
    #[cfg(not(debug_assertions))]
    let url = Url::from_str("https://unload.fly.dev").unwrap();
    Ok(reqwest::get(
        url.join(&format!("/api/boards/{}/archive/tasks", board_name))
            .unwrap(),
    )
    .await?
    .json::<Vec<TaskEntry>>()
    .await?)
}
