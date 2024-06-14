use dioxus::prelude::*;
use shared_models::{Color, UserEntry, UserId};

use crate::{
    components::{
        form::{CancelButton, ConfirmButton},
        icons::{EditIcon, TrashIcon},
        input::TextInput,
    },
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
                class: "flex flex-row items-center gap-1",
                Color { color: user.color }
                Name { user_id: user.id, name: user.name }
            }
            div {
                class: "flex flex-row items-center gap-1",
                DeleteUserBUtton { user_id: user.id }
            }
        }
    }
}

#[component]
fn Color(color: Color) -> Element {
    rsx! {}
}

#[component]
fn Name(user_id: UserId, name: String) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            NameInput { user_id, name, editing }
        } else {
            NameShow { name, editing }
        }
    }
}

#[component]
fn NameInput(user_id: UserId, name: String, editing: Signal<bool>) -> Element {
    rsx! {
        form {
            id: "user-{user_id}-name-form",
            "aria-label": "edit name",
            class: "flex flex-row gap-2 items-center",
            TextInput {
                id: "user-{user_id}-name-input",
                label: "Name",
            }
            ConfirmButton { label: "set name" }
            CancelButton { label: "cancel name update", editing }
        }
    }
}

#[component]
fn NameShow(name: String, editing: Signal<bool>) -> Element {
    rsx! {
        {name}
        button {
            class: "size-4",
            "aria-label": "edit name",
            onclick: move |_| editing.set(true),
            EditIcon {}
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
