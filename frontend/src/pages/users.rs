use dioxus::prelude::*;
use shared_models::BoardName;

use crate::components::nav::NavBar;
use crate::model::UnloadUrl;
use crate::pages::users::components::UserList;
use crate::pages::users::model::{UserEntries, UsersUrl};
use crate::themes::Theme;

mod components;
mod model;
mod requests;

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
    use_future(move || requests::get_users(users, url));
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
                class: "
                    grow container mx-auto py-4 h-full overflow-y-auto
                    flex flex-col items-center justify-center
                ",
                UserList {}
            }
            NavBar { board_name }
        }
    }
}
