use dioxus::prelude::*;

use reqwest::Url;
use shared_models::{Color, TagEntry, TagId};

use crate::pages::tags::model::{TagEntries, TagsUrl};

pub async fn get_tags(tags: Signal<TagEntries>, url: Signal<TagsUrl>) {
    let url = &url.read().0;
    get_tags_(tags, url).await;
}

async fn send_get_tags_request(url: &Url) -> Result<Vec<TagEntry>, anyhow::Error> {
    let url = url.join("tags")?;
    Ok(reqwest::Client::new()
        .get(url.clone())
        .send()
        .await?
        .json::<Vec<TagEntry>>()
        .await?)
}

pub async fn set_tag_color(
    tags: Signal<TagEntries>,
    url: Signal<TagsUrl>,
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
    url: Signal<TagsUrl>,
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

pub async fn get_tags_(mut tags: Signal<TagEntries>, url: &Url) {
    if let Ok(result) = send_get_tags_request(url).await {
        tags.write().0 = result;
    }
}

pub async fn delete_tag(tags: Signal<TagEntries>, url: Signal<TagsUrl>, tag_id: TagId) {
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

pub async fn set_tag_archived(tags: Signal<TagEntries>, url: Signal<TagsUrl>, tag_id: TagId) {
    let url = &url.read().0;
    let _ = send_set_tag_archived_request(url, tag_id).await;
    get_tags_(tags, url).await;
}

async fn send_set_tag_archived_request(url: &Url, tag_id: TagId) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("tags/{}/archived", tag_id))?;
    Ok(reqwest::Client::new()
        .put(url)
        .json(&true)
        .send()
        .await?
        .json::<()>()
        .await?)
}
