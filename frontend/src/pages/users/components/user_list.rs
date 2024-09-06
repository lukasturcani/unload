use dioxus::prelude::*;
use dioxus_sdk::{i18n::use_i18, translate};
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
            border-y sm:border-x sm:rounded-lg
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
                        class: "flex flex-row items-center gap-5 group filled",
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
    let i18 = use_i18();
    let users = use_context::<Signal<UserEntries>>();
    let url = use_context::<Signal<UsersUrl>>();
    rsx! {
        form {
            id: "user-{user_id}-color-form",
            aria_label: translate!(i18, "edit_user_color_form_label"),
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
                ConfirmButton { label: translate!(i18, "set_user_color_button_label") }
                CancelButton {
                    aria_label: translate!(i18, "cancel_user_color_update_button_label"),
                    state,
                }
            }
        }
    }
}

#[component]
fn CancelButton(aria_label: String, state: Signal<State>) -> Element {
    let style = "
        rounded-md
        border border-red-600
        stroke-red-600
        active:bg-red-600
        sm:hover:bg-red-600 sm:hover:stroke-white
    ";
    rsx! {
        button {
            aria_label,
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
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let color = match color {
        Color::Black => theme.color1_button,
        Color::White => theme.color2_button,
        Color::Gray => theme.color3_button,
        Color::Silver => theme.color4_button,
        Color::Maroon => theme.color5_button,
        Color::Red => theme.color6_button,
        Color::Purple => theme.color7_button,
        Color::Fushsia => theme.color8_button,
        Color::Green => theme.color9_button,
        Color::Lime => theme.color10_button,
        Color::Olive => theme.color11_button,
        Color::Yellow => theme.color12_button,
        Color::Navy => theme.color13_button,
        Color::Blue => theme.color14_button,
        Color::Teal => theme.color15_button,
        Color::Aqua => theme.color16_button,
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
fn NameInput(user_id: UserId, name: ReadOnlySignal<String>, state: Signal<State>) -> Element {
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
            CancelButton { aria_label: "cancel name update", state }
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
