use dioxus::prelude::*;
use shared_models::{UserEntry, UserId};

use crate::{
    components::{icons::TrashIcon, tooltip::Tooltip},
    pages::users::{
        model::{UserEntries, UsersUrl},
        requests,
    },
    themes::Theme,
};

#[component]
pub fn UserList() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
            border rounded-lg
            divide-y
            {} {}
        ",
        theme.border_color, theme.divide_color,
    );
    let users = use_context::<Signal<UserEntries>>();
    rsx! {
        ul {
            class: "overflow-y-auto w-full {style}",
            for user in users.read().0.iter() {
                UserListItem { user: user.clone() }
            }
        }
    }
}

#[component]
fn UserListItem(user: UserEntry) -> Element {
    rsx! {
        li {
            class: "
                px-3 py-1
                flex flex-row justify-between
            ",
            div {
                { user.name }
            }
            div {
                class: "flex flex-row items-center gap-1",
                DeleteUserBUtton { user_id: user.id }
            }
        }
    }
}

#[component]
fn DeleteUserBUtton(user_id: UserId) -> Element {
    let url = use_context::<Signal<UsersUrl>>();
    let users = use_context::<Signal<UserEntries>>();
    let style = "stroke-red-600";
    rsx! {
        button {
            "aria-label": "delete user",
            class: "block size-6 {style}",
            onclick: move |_| {
                spawn_forever(requests::delete_user(users, url, user_id));
            },
            TrashIcon {}
        }
    }
}
