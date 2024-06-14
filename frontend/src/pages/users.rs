use dioxus::prelude::*;
use shared_models::BoardName;

use crate::components::nav::NavBar;
use crate::model::UnloadUrl;
use crate::pages::users::components::UserList;
use crate::pages::users::model::UsersUrl;
use crate::themes::Theme;

mod components;
mod model;

#[component]
pub fn Users(board_name: BoardName) -> Element {
    let url = use_context::<Signal<UnloadUrl>>();
    use_context_provider(move || {
        let url = url
            .read()
            .0
            .join(&format!("/api/boards/{}/users", board_name))
            .unwrap();
        Signal::new(UsersUrl(url))
    });
    rsx! { UserList {} }
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
                UserList {}
            }
            NavBar { board_name }
        }
    }
}
