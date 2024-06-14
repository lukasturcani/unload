use dioxus::prelude::*;
use reqwest::Url;
use shared_models::{BoardName, UserEntry};

use crate::components::nav::NavBar;
use crate::model::UnloadUrl;
use crate::pages::users::components::UserList;
use crate::pages::users::model::{UserEntries, UsersUrl};
use crate::themes::Theme;

mod components;
mod model;

#[component]
pub fn Users(board_name: BoardName) -> Element {
    let url = use_context::<Signal<UnloadUrl>>();
    eval(&format!(r#"document.title = "{board_name}";"#));
    let users = use_context_provider(|| Signal::new(UserEntries::default()));
    let url = use_context_provider({
        let board_name = board_name.clone();
        move || {
            let url = url
                .read()
                .0
                .join(&format!("/api/boards/{}/users", board_name))
                .unwrap();
            Signal::new(UsersUrl(url))
        }
    });
    use_future(move || get_users(users, url));
    rsx! { UsersPage { board_name } }
}

#[component]
fn UsersPage(board_name: BoardName) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("{} {}", theme.text_color, theme.bg_color_1);
    rsx! {
        div {
            class: "flex flex-col h-dvh w-screen {style}",
            div {
                class: "grow container mx-auto h-full overflow-y-auto py-4",
                UserList {}
            }
            NavBar { board_name }
        }
    }
}

async fn get_users(mut users: Signal<UserEntries>, url: Signal<UsersUrl>) {
    let url = &url.read().0;
    if let Ok(result) = send_get_users_request(url).await {
        users.write().0 = result;
    }
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
