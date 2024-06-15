use dioxus::prelude::*;
use reqwest::Url;
use shared_models::{Color, TagEntry, TagId, TaskEntry, TaskId};

use crate::model::UnloadUrl;

use super::model::{TagEntries, TaskEntries};

pub async fn set_tag_color(
    tags: Signal<TagEntries>,
    url: Signal<UnloadUrl>,
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
    Ok(reqwest::Client::new()
        .put(url)
        .json(&color)
        .send()
        .await?
        .json::<()>()
        .await?)
}

pub async fn set_tag_name(
    tags: Signal<TagEntries>,
    url: Signal<UnloadUrl>,
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
    Ok(reqwest::Client::new()
        .put(url)
        .json(&name)
        .send()
        .await?
        .json::<()>()
        .await?)
}

pub async fn delete_tag(tags: Signal<TagEntries>, url: Signal<UnloadUrl>, tag_id: TagId) {
    let url = &url.read().0;
    let _ = send_delete_tag_request(url, tag_id).await;
    get_tags_(tags, url).await;
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

pub async fn set_task_archived(
    tasks: Signal<TaskEntries>,
    url: Signal<UnloadUrl>,
    task_id: TaskId,
) {
    let url = &url.read().0;
    let _ = send_set_task_archived_request(url, task_id).await;
    get_tasks_(tasks, url).await;
}

async fn send_set_task_archived_request(url: &Url, task_id: TaskId) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tasks/{}/archived", task_id))?;
    Ok(reqwest::Client::new()
        .put(url)
        .json(&false)
        .send()
        .await?
        .json::<()>()
        .await?)
}

pub async fn set_tag_archived(tags: Signal<TagEntries>, url: Signal<UnloadUrl>, tag_id: TagId) {
    let url = &url.read().0;
    let _ = send_set_tag_archived_request(url, tag_id).await;
    get_tags_(tags, url).await;
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

pub async fn get_tags(mut tags: Signal<TagEntries>, url: Signal<UnloadUrl>) {
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
    Ok(reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<TagEntry>>()
        .await?)
}

pub async fn get_tasks(tasks: Signal<TaskEntries>, url: Signal<UnloadUrl>) {
    let url = &url.read().0;
    get_tasks_(tasks, url).await;
}

async fn get_tasks_(mut tasks: Signal<TaskEntries>, url: &Url) {
    if let Ok(result) = send_get_tasks_request(url).await {
        tasks.write().0 = result;
    }
}

async fn send_get_tasks_request(url: &Url) -> Result<Vec<TaskEntry>, anyhow::Error> {
    let url = url.join("archive/tasks")?;
    Ok(reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<TaskEntry>>()
        .await?)
}

pub async fn delete_task(tasks: Signal<TaskEntries>, url: Signal<UnloadUrl>, task_id: TaskId) {
    let url = &url.read().0;
    let _ = send_delete_task_request(url, task_id).await;
    get_tasks_(tasks, url).await;
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
