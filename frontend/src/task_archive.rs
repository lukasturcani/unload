use crate::color_picker;
use crate::styles;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use reqwest::Url;
use shared_models::{
    BoardName, TagData, TagEntry, TagId, TaskData, TaskEntry, TaskId, UserData, UserEntry, UserId,
};
use std::{collections::HashMap, str::FromStr};
use tokio::join;

#[derive(Default)]
struct TaskArchive {
    tasks: HashMap<TaskId, TaskData>,
    users: HashMap<UserId, UserData>,
    tags: HashMap<TagId, TagData>,
}

struct TasksUrl(Url);

#[component]
pub fn TaskArchive(cx: Scope, board_name: BoardName) -> Element {
    use_shared_state_provider(cx, || {
        #[cfg(debug_assertions)]
        let url = Url::from_str("http://localhost:8080").unwrap();
        #[cfg(not(debug_assertions))]
        let url = Url::from_str("https://unload.fly.dev").unwrap();
        TasksUrl(url.join(&format!("/api/boards/{}/", board_name)).unwrap())
    });
    use_shared_state_provider(cx, TaskArchive::default);

    let nav = use_navigator(cx);
    let archive = use_shared_state::<TaskArchive>(cx).unwrap();
    let url = use_shared_state::<TasksUrl>(cx).unwrap();
    use_future(cx, (), {
        let url = url.clone();
        let tasks = archive.clone();
        |_| async move {
            let url = url.read();
            let tasks = tasks.clone();
            get_archive(tasks, &url.0).await
        }
    });
    cx.render(rsx! {
        div {
            class: "
                w-screen h-dvh
                bg-gray-900
                flex flex-col
            ",
            ul {
                class: "
                    grow w-full p-4 overflow-auto
                    divide-y divide-gray-700
                ",
               archive
                    .read()
                    .tasks
                    .keys()
                    .map(|task_id| rsx!(Task{ key: "{task_id}", task_id: *task_id }))
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
fn Task(cx: Scope, task_id: TaskId) -> Element {
    let url = use_shared_state::<TasksUrl>(cx).unwrap();
    let archive = use_shared_state::<TaskArchive>(cx).unwrap();
    let archive_read = archive.read();
    let task = &archive_read.tasks[&task_id];
    let users = &archive_read.users;
    let tags = &archive_read.tags;
    let expanded = use_state(cx, || false);
    cx.render(rsx! {
        li {
            class: "
                p-2.5 sm:hover:bg-gray-600
                flex flex-col gap-2
            ",
            onclick: |_| expanded.set(!**expanded),
            div {
                class: "flex flex-row justify-between",
                p {
                    class: "text-white",
                    task.title.clone()
                }
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
                        onclick: |_| unarchive_task(archive.clone(), url.clone(), *task_id),
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
                        onclick: |_| delete_task(archive.clone(), url.clone(), *task_id),
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0",
                        }
                    }
                }
            }
            if **expanded {rsx! {
                div {
                    class: "flex flex-row gap-2 flex-wrap",
                    for user in task
                        .assignees
                        .iter()
                        .map(|user_id| &users[user_id])
                    {rsx!{
                        span {
                            class: "
                                text-sm font-medium text-white
                                px-2.5 py-0.5 rounded
                                border-2
                                {color_picker::border_class(&user.color)}
                            ",
                            user.name.clone()
                        }
                    }}
                }
                p {
                    class: "text-gray-400",
                    task.description.clone()
                }
                div {
                    class: "flex flex-row gap-2 flex-wrap",
                    for tag in task
                        .tags
                        .iter()
                        .map(|tag_id| &tags[tag_id])
                    {rsx!{
                        span {
                            class: "
                                text-sm font-medium text-white
                                px-2.5 py-0.5 rounded
                                border-2
                                {color_picker::border_class(&tag.color)}
                            ",
                            "# {&tag.name}"
                        }
                    }}
                }
            }}
        }
    })
}

async fn get_archive(archive: UseSharedState<TaskArchive>, url: &Url) {
    let (tasks, users, tags) = join!(
        send_get_tasks_request(url),
        send_get_users_request(url),
        send_get_tags_request(url),
    );
    let mut archive = archive.write();
    if let Ok(result) = tasks {
        archive.tasks = result;
    }
    if let Ok(result) = users {
        archive.users = result;
    }
    if let Ok(result) = tags {
        archive.tags = result;
    }
}

async fn send_get_tasks_request(url: &Url) -> Result<HashMap<TaskId, TaskData>, anyhow::Error> {
    let tasks = reqwest::get(url.join("archive/tasks").unwrap())
        .await?
        .json::<Vec<TaskEntry>>()
        .await?;
    Ok(tasks
        .into_iter()
        .map(|task| {
            (
                task.id,
                TaskData {
                    title: task.title,
                    description: task.description,
                    due: task.due,
                    size: task.size,
                    status: task.status,
                    assignees: task.assignees,
                    tags: task.tags,
                },
            )
        })
        .collect())
}

async fn send_get_users_request(url: &Url) -> Result<HashMap<UserId, UserData>, anyhow::Error> {
    let users = reqwest::get(url.join("users").unwrap())
        .await?
        .json::<Vec<UserEntry>>()
        .await?;
    Ok(users
        .into_iter()
        .map(|user| {
            (
                user.id,
                UserData {
                    name: user.name,
                    color: user.color,
                },
            )
        })
        .collect())
}

async fn send_get_tags_request(url: &Url) -> Result<HashMap<TagId, TagData>, anyhow::Error> {
    match join!(reqwest_tags(url), reqwest_archived_tags(url)) {
        (Ok(tags), Ok(archived_tags)) => Ok(tags
            .into_iter()
            .chain(archived_tags.into_iter())
            .map(|tag| {
                (
                    tag.id,
                    TagData {
                        name: tag.name,
                        color: tag.color,
                    },
                )
            })
            .collect()),
        (Err(e), _) => Err(e.into()),
        (_, Err(e)) => Err(e.into()),
    }
}

async fn reqwest_tags(url: &Url) -> Result<Vec<TagEntry>, reqwest::Error> {
    reqwest::get(url.join("tags").unwrap())
        .await?
        .json::<Vec<TagEntry>>()
        .await
}

async fn reqwest_archived_tags(url: &Url) -> Result<Vec<TagEntry>, reqwest::Error> {
    reqwest::get(url.join("archive/tags").unwrap())
        .await?
        .json::<Vec<TagEntry>>()
        .await
}

async fn unarchive_task(
    archive: UseSharedState<TaskArchive>,
    url: UseSharedState<TasksUrl>,
    task_id: TaskId,
) {
    let url = &url.read().0;
    let _ = send_unarchive_task_request(url, task_id).await;
    get_archive(archive, url).await;
}

async fn send_unarchive_task_request(url: &Url, task_id: TaskId) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tasks/{}/archived", task_id))?;
    Ok(reqwest::Client::new()
        .put(url)
        .json(&false)
        .send()
        .await?
        .json::<()>()
        .await?)
}

async fn delete_task(
    archive: UseSharedState<TaskArchive>,
    url: UseSharedState<TasksUrl>,
    task_id: TaskId,
) {
    let url = &url.read().0;
    let _ = send_delete_task_request(url, task_id).await;
    get_archive(archive, url).await;
}

async fn send_delete_task_request(url: &Url, task_id: TaskId) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tasks/{}", task_id))?;
    Ok(reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await?)
}
