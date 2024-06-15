use std::collections::HashMap;

use dioxus::prelude::*;
use reqwest::{Client, Url};
use shared_models::{
    Color, TagData, TagEntry, TagId, TaskEntry, TaskId, UserData, UserEntry, UserId,
};
use tokio::join;

use super::model::{BoardUrl, TagEntries, Tags, TaskEntries, Users};

pub async fn set_tag_color(
    tags: Signal<TagEntries>,
    url: Signal<BoardUrl>,
    tag_id: TagId,
    color: Color,
) {
    let url = &url.read().0;
    let _ = send_set_tag_color_request(url, tag_id, color).await;
    get_tags_(tags, url).await;
}

async fn send_set_tag_color_request(
    url: &Url,
    tag_id: TagId,
    color: Color,
) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tags/{}/color", tag_id))?;
    Ok(Client::new()
        .put(url)
        .json(&color)
        .send()
        .await?
        .json::<()>()
        .await?)
}

pub async fn set_tag_name(
    tags: Signal<TagEntries>,
    url: Signal<BoardUrl>,
    tag_id: TagId,
    name: String,
) {
    let url = &url.read().0;
    let _ = send_set_tag_name_request(url, tag_id, name).await;
    get_tags_(tags, url).await;
}

async fn send_set_tag_name_request(
    url: &Url,
    tag_id: TagId,
    name: String,
) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tags/{}/name", tag_id))?;
    Ok(Client::new()
        .put(url)
        .json(&name)
        .send()
        .await?
        .json::<()>()
        .await?)
}

pub async fn delete_tag(tags: Signal<TagEntries>, url: Signal<BoardUrl>, tag_id: TagId) {
    let url = &url.read().0;
    let _ = send_delete_tag_request(url, tag_id).await;
    get_tags_(tags, url).await;
}

async fn send_delete_tag_request(url: &Url, tag_id: TagId) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tags/{}", tag_id))?;
    Ok(Client::new().delete(url).send().await?.json::<()>().await?)
}

pub async fn set_task_archived(tasks: Signal<TaskEntries>, url: Signal<BoardUrl>, task_id: TaskId) {
    let url = &url.read().0;
    let _ = send_set_task_archived_request(url, task_id).await;
    get_tasks_(tasks, url).await;
}

async fn send_set_task_archived_request(url: &Url, task_id: TaskId) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tasks/{}/archived", task_id))?;
    Ok(Client::new()
        .put(url)
        .json(&false)
        .send()
        .await?
        .json::<()>()
        .await?)
}

pub async fn set_tag_archived(tags: Signal<TagEntries>, url: Signal<BoardUrl>, tag_id: TagId) {
    let url = &url.read().0;
    let _ = send_set_tag_archived_request(url, tag_id).await;
    get_tags_(tags, url).await;
}

async fn send_set_tag_archived_request(url: &Url, tag_id: TagId) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tags/{}/archived", tag_id))?;
    Ok(Client::new()
        .put(url)
        .json(&false)
        .send()
        .await?
        .json::<()>()
        .await?)
}

pub async fn get_tag_entries(mut tags: Signal<TagEntries>, url: Signal<BoardUrl>) {
    let url = &url.read().0;
    if let Ok(result) = send_get_tags_request(url).await {
        tags.write().0 = result;
    }
}

async fn get_tags_(mut tags: Signal<TagEntries>, url: &Url) {
    if let Ok(result) = send_get_tags_request(url).await {
        tags.write().0 = result;
    }
}

async fn send_get_tags_request(url: &Url) -> Result<Vec<TagEntry>, anyhow::Error> {
    let url = url.join("archive/tags")?;
    Ok(Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<TagEntry>>()
        .await?)
}

pub async fn get_task_archive(
    mut tasks: Signal<TaskEntries>,
    mut tags: Signal<Tags>,
    mut users: Signal<Users>,
    url: Signal<BoardUrl>,
) {
    let url = &url.read().0;
    if let (Ok(new_tasks), Ok(mut new_tags), Ok(new_unarchived_tags), Ok(new_users)) = join!(
        send_get_tasks_request(url),
        get_tags(url),
        get_unarchived_tags(url),
        get_users(url)
    ) {
        new_tags.extend(new_unarchived_tags);
        tasks.write().0 = new_tasks;
        tags.write().0 = new_tags;
        users.write().0 = new_users;
    }
}

async fn get_tags(url: &Url) -> Result<HashMap<TagId, TagData>, anyhow::Error> {
    let url = url.join("archive/tags")?;
    Ok(Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<TagEntry>>()
        .await?
        .into_iter()
        .fold(HashMap::new(), |mut tags, tag| {
            tags.insert(
                tag.id,
                TagData {
                    name: tag.name,
                    color: tag.color,
                },
            );
            tags
        }))
}

async fn get_unarchived_tags(url: &Url) -> Result<HashMap<TagId, TagData>, anyhow::Error> {
    let url = url.join("tags")?;
    Ok(Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<TagEntry>>()
        .await?
        .into_iter()
        .fold(HashMap::new(), |mut tags, tag| {
            tags.insert(
                tag.id,
                TagData {
                    name: tag.name,
                    color: tag.color,
                },
            );
            tags
        }))
}

async fn get_users(url: &Url) -> Result<HashMap<UserId, UserData>, anyhow::Error> {
    let url = url.join("users")?;
    Ok(Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<UserEntry>>()
        .await?
        .into_iter()
        .fold(HashMap::new(), |mut users, user| {
            users.insert(
                user.id,
                UserData {
                    name: user.name,
                    color: user.color,
                },
            );
            users
        }))
}

async fn get_tasks_(mut tasks: Signal<TaskEntries>, url: &Url) {
    if let Ok(result) = send_get_tasks_request(url).await {
        tasks.write().0 = result;
    }
}

async fn send_get_tasks_request(url: &Url) -> Result<Vec<TaskEntry>, anyhow::Error> {
    let url = url.join("archive/tasks")?;
    Ok(Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<TaskEntry>>()
        .await?)
}

pub async fn delete_task(tasks: Signal<TaskEntries>, url: Signal<BoardUrl>, task_id: TaskId) {
    let url = &url.read().0;
    let _ = send_delete_task_request(url, task_id).await;
    get_tasks_(tasks, url).await;
}

async fn send_delete_task_request(url: &Url, task_id: TaskId) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tasks/{}", task_id))?;
    Ok(Client::new().delete(url).send().await?.json::<()>().await?)
}
