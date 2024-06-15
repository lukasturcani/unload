use dioxus::prelude::*;

use reqwest::Url;
use shared_models::{Color, UserEntry, UserId};

use crate::pages::users::model::{UserEntries, UsersUrl};

pub async fn get_users(users: Signal<UserEntries>, url: Signal<UsersUrl>) {
    let url = &url.read().0;
    get_users_(users, url).await;
}

async fn send_get_users_request(url: &Url) -> Result<Vec<UserEntry>, anyhow::Error> {
    let url = url.join("users")?;
    Ok(reqwest::Client::new()
        .get(url.clone())
        .send()
        .await?
        .json::<Vec<UserEntry>>()
        .await?)
}

pub async fn set_user_color(
    users: Signal<UserEntries>,
    url: Signal<UsersUrl>,
    user_id: UserId,
    color: Color,
) {
    let url = &url.read().0;
    let _ = send_set_user_color_request(url, user_id, color).await;
    get_users_(users, url).await;
}

async fn send_set_user_color_request(
    url: &Url,
    user_id: UserId,
    color: Color,
) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("users/{}/color", user_id))?;
    Ok(reqwest::Client::new()
        .put(url)
        .json(&color)
        .send()
        .await?
        .json::<()>()
        .await?)
}

pub async fn set_user_name(
    users: Signal<UserEntries>,
    url: Signal<UsersUrl>,
    user_id: UserId,
    name: String,
) {
    let url = &url.read().0;
    let _ = send_set_user_name_request(url, user_id, name).await;
    get_users_(users, url).await;
}

async fn send_set_user_name_request(
    url: &Url,
    user_id: UserId,
    name: String,
) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("users/{}/name", user_id))?;
    Ok(reqwest::Client::new()
        .put(url)
        .json(&name)
        .send()
        .await?
        .json::<()>()
        .await?)
}

pub async fn get_users_(mut users: Signal<UserEntries>, url: &Url) {
    if let Ok(result) = send_get_users_request(url).await {
        users.write().0 = result;
    }
}

pub async fn delete_user(users: Signal<UserEntries>, url: Signal<UsersUrl>, user_id: UserId) {
    let url = &url.read().0;
    let _ = send_delete_user_request(url, user_id).await;
    get_users_(users, url).await;
}

async fn send_delete_user_request(url: &Url, user_id: UserId) -> Result<(), anyhow::Error> {
    let url = url.join(&format!("users/{}", user_id))?;
    Ok(reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await?)
}
