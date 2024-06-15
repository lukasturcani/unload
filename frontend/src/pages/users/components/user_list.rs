use dioxus::prelude::*;
use shared_models::{Color, UserEntry, UserId};

use crate::{
    commands::ScrollTarget,
    components::{
        color_picker::ColorPicker,
        form::ConfirmButton,
        icons::{CancelIcon, EditIcon, TrashIcon},
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
            class: "overflow-y-auto w-full max-w-lg {style}",
            for user in users.read().0.iter() {
                UserListItem { user: user.clone() }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum State {
    EditingColor,
    EditingName,
    Show,
}

#[component]
fn UserListItem(user: UserEntry) -> Element {
    let state = use_signal(|| State::Show);
    rsx! {
        match state() {
            State::Show => rsx! {
                li {
                    class: "
                        px-3 py-1
                        flex flex-row justify-between
                    ",
                    div {
                        class: "flex flex-row items-center gap-5",
                        ColorShow { user_id: user.id, color: user.color, state }
                        NameShow { user_id: user.id, name: user.name, state }

                    }
                    div {
                        class: "flex flex-row items-center gap-1",
                        DeleteUserButton { user_id: user.id }
                    }
                }
            },
            State::EditingColor => rsx! {
                li {
                    class: "flex flex-row w-full items-center justify-center",
                    ColorSelect { user_id: user.id, color: user.color, state }
                }
            },
            State::EditingName => rsx! {
                li {
                    class: "flex flex-row w-full items-center justify-center",
                    NameInput { user_id: user.id, name: user.name, state }
                }
            },
        }
    }
}

#[component]
fn ColorSelect(user_id: UserId, color: Color, state: Signal<State>) -> Element {
    let users = use_context::<Signal<UserEntries>>();
    let url = use_context::<Signal<UsersUrl>>();
    rsx! {
        form {
            id: "user-{user_id}-color-form",
            "aria-label": "edit color",
            class: "flex flex-col gap-2 items-center p-2",
            onsubmit: move |event| {
                let color = serde_json::from_str(
                    &event.values()["color-picker"].as_value()
                ).unwrap();
                spawn_forever(requests::set_user_color(users, url, user_id, color));
                state.set(State::Show);
            },
            ColorPicker { selected_color: color }
            div {
                class: "flex flex-row gap-2 items-center justify-center",
                ConfirmButton { label: "set color" }
                CancelButton { label: "cancel color update", state }
            }
        }
    }
}

#[component]
fn CancelButton(label: String, state: Signal<State>) -> Element {
    let style = "
        rounded-md
        border border-red-600
        stroke-red-600
        active:bg-red-600
        sm:hover:bg-red-600 sm:hover:stroke-white
    ";
    rsx! {
        button {
            "aria-label": label,
            class: "size-7 {style}",
            onclick: move |_| {
                state.set(State::Show);
            },
            CancelIcon {}
        }
    }
}

#[component]
fn ColorShow(user_id: UserId, color: Color, state: Signal<State>) -> Element {
    let mut scroll_target = use_context::<Signal<ScrollTarget>>();
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
            class: "flex flex-row items-center gap-1",
            div {
                class: "size-6 {style}",
            }
            button {
                class: "size-4",
                "aria-label": "edit color",
                onclick: move |_| {
                    scroll_target.set(
                        ScrollTarget(Some(format!("user-{user_id}-color-form")))
                    );
                    state.set(State::EditingColor)
                },
                EditIcon {}
            }
        }
    }
}

#[component]
fn NameInput(user_id: UserId, name: String, state: Signal<State>) -> Element {
    let url = use_context::<Signal<UsersUrl>>();
    let users = use_context::<Signal<UserEntries>>();
    rsx! {
        form {
            id: "user-{user_id}-name-form",
            "aria-label": "edit name",
            class: "flex flex-row gap-2 items-center p-2",
            onsubmit: move |event| {
                let name = event.values()["Name"].as_value();
                spawn_forever(requests::set_user_name(users, url, user_id, name));
                state.set(State::Show);
            },
            TextInput {
                id: "user-{user_id}-name-input",
                label: "Name",
                value: name,
            }
            ConfirmButton { label: "set name" }
            CancelButton { label: "cancel name update", state }
        }
    }
}

#[component]
fn NameShow(user_id: UserId, name: String, state: Signal<State>) -> Element {
    let mut scroll_target = use_context::<Signal<ScrollTarget>>();
    rsx! {
        div {
            class: "flex flex-row items-center gap-1",
            {name}
            button {
                class: "size-4",
                "aria-label": "edit name",
                onclick: move |_| {
                    scroll_target.set(
                        ScrollTarget(Some(format!("user-{user_id}-name-form")))
                    );
                    state.set(State::EditingName)
                },
                EditIcon {}
            }
        }
    }
}

#[component]
fn DeleteUserButton(user_id: UserId) -> Element {
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
