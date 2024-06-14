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
                class: "flex flex-row items-center gap-5",
                Color { user_id: user.id, color: user.color }
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
fn Color(user_id: UserId, color: Color) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        div {
            class: "flex flex-row items-center gap-1",
            if editing() {
            }
            else {
                ColorShow { color, editing }
            }
        }
    }
}

#[component]
fn ColorShow(color: Color, editing: Signal<bool>) -> Element {
    let color = match color {
        Color::Black => "bg-black",
        Color::White => "bg-white",
        Color::Gray => "bg-gray-400",
        Color::Silver => "bg-slate-500",
        Color::Maroon => "bg-rose-400",
        Color::Red => "bg-red-600",
        Color::Purple => "bg-purple-600",
        Color::Fushsia => "bg-fuchsia-400",
        Color::Green => "bg-emerald-500",
        Color::Lime => "bg-lime-500",
        Color::Olive => "bg-indigo-400",
        Color::Yellow => "bg-yellow-400",
        Color::Navy => "bg-amber-200",
        Color::Blue => "bg-blue-400",
        Color::Teal => "bg-teal-300",
        Color::Aqua => "bg-cyan-500",
    };
    let style = format!("rounded {color}");
    rsx! {
        div {
            class: "size-6 {style}",
        }
        button {
            class: "size-4",
            "aria-label": "edit color",
            onclick: move |_| editing.set(true),
            EditIcon {}
        }
    }
}

#[component]
fn Name(user_id: UserId, name: String) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        div {
            class: "flex flex-row items-center gap-1",
            if editing() {
                NameInput { user_id, name, editing }
            } else {
                NameShow { name, editing }
            }
        }
    }
}

#[component]
fn NameInput(user_id: UserId, name: String, editing: Signal<bool>) -> Element {
    let url = use_context::<Signal<UsersUrl>>();
    let users = use_context::<Signal<UserEntries>>();
    rsx! {
        form {
            id: "user-{user_id}-name-form",
            "aria-label": "edit name",
            class: "flex flex-row gap-2 items-center",
            onsubmit: move |event| {
                let name = event.values()["Name"].as_value();
                spawn_forever(requests::set_user_name(users, url, user_id, name));
                editing.set(false);
            },
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
