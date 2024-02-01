use dioxus::prelude::*;
use reqwest::Url;
use shared_models::{BoardName, TaskEntry};
use std::str::FromStr;

#[component]
pub fn TaskArchive(cx: Scope, board_name: BoardName) -> Element {
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
               tasks
                    .iter()
                    .map(|task| {
                        rsx! {
                            li {
                                key: "{task.id}",
                                class: "
                                    text-white
                                    p-2.5
                                    border-b border-gray-700
                                ",
                                task.title.clone()
                            }
                        }
                    })
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
