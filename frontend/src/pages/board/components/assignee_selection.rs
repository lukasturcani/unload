use dioxus::prelude::*;
use dioxus_sdk::{i18n::use_i18, translate};
use shared_models::{Color, UserData, UserId};

use crate::{
    components::{
        color_picker::ColorPicker,
        form::{CancelButton, ConfirmButton},
        icons::CancelIcon,
        input::TextInput,
    },
    pages::board::{
        components::assignment_list::{
            AssignmentList, AssignmentListItem, ShowSelectionListFormButton,
        },
        model::Users,
        requests::{self, BoardSignals},
    },
    themes::Theme,
};

#[component]
pub fn AssigneeSelection(
    id: String,
    assignees: ReadOnlySignal<Vec<UserId>>,
    on_assign_user: EventHandler<UserId>,
    on_unassign_user: EventHandler<UserId>,
    on_add_user: EventHandler<UserId>,
) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("rounded-lg border {}", theme.border_color);
    let users = use_context::<Signal<Users>>();
    let users = &users.read().0;
    let assignees = assignees.read();
    let mut assignee_data = Vec::with_capacity(assignees.len());
    let mut unassigned = Vec::with_capacity(users.len() - assignees.len());
    for (user_id, user) in users.iter() {
        if assignees.contains(user_id) {
            assignee_data.push((*user_id, user.clone()));
        } else {
            unassigned.push((*user_id, user.clone()));
        }
    }
    unassigned.sort_by_key(|(_, user)| user.name.to_lowercase());
    rsx! {
        section {
            aria_label: translate!(i18, "assignee_selection_section_label"),
            class: "flex flex-col gap-2 p-2 {style}",
            UserBadges { assignees: assignee_data, on_unassign_user }
            UserList { id, unassigned, on_assign_user, on_add_user }
        }
    }
}

#[component]
fn UserBadges(
    assignees: Vec<(UserId, UserData)>,
    on_unassign_user: EventHandler<UserId>,
) -> Element {
    rsx! {
       div {
            class: "flex flex-row gap-2 flex-wrap group text-colored",
            for (user_id, user_data) in assignees {
                UserBadge { user_id, user_data, on_unassign_user }
            }
        }
    }
}

#[component]
fn UserBadge(
    user_id: UserId,
    user_data: UserData,
    on_unassign_user: EventHandler<UserId>,
) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = "border-2 rounded";
    let button_style = format!("rounded sm:hover:border {}", theme.border_color);
    let color = match user_data.color {
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
    let aria_label = format!(
        "{}: {}",
        translate!(i18, "remove_user_from_task_button_label"),
        user_data.name,
    );
    rsx! {
        div {
            class: "
                flex flex-row items-center gap-2
                text-sm py-1 px-2 {style} {color}
            ",
            {user_data.name}
            button {
                aria_label,
                class: "size-5 p-0.5 {button_style}",
                onclick: move |_| on_unassign_user.call(user_id),
                CancelIcon {}
            }
        }
    }
}

#[component]
fn UserList(
    id: String,
    unassigned: Vec<(UserId, UserData)>,
    on_assign_user: EventHandler<UserId>,
    on_add_user: EventHandler<UserId>,
) -> Element {
    rsx! {
        AssignmentList {
            body: rsx! {
                for (user_id, user) in unassigned {
                    UserListItem { key: "{user_id}", user_id, user, on_assign_user }
                }
                AddUserListItem { key: "{\"add-user\"}", id, on_add_user }
            }
        }
    }
}

#[component]
fn UserListItem(user_id: UserId, user: UserData, on_assign_user: EventHandler<UserId>) -> Element {
    rsx! {
        AssignmentListItem {
            content: user.name,
            color: user.color,
            onclick: move |_| on_assign_user.call(user_id),
        }
    }
}

#[component]
fn AddUserListItem(id: String, on_add_user: EventHandler<UserId>) -> Element {
    let i18 = use_i18();
    let show_form = use_signal(|| false);
    rsx! {
        li {
            if show_form() {
                AddUserListForm { id, show_form, on_add_user }
            } else {
                ShowSelectionListFormButton {
                    r#for: "{id}-form",
                    content: translate!(i18, "add_user_button_label"),
                    show_form ,
                }
            }
        }
    }
}

#[component]
fn AddUserListForm(
    id: String,
    show_form: Signal<bool>,
    on_add_user: EventHandler<UserId>,
) -> Element {
    let i18 = use_i18();
    let board_signals = BoardSignals::default();
    let input_label = translate!(i18, "user_name_input_label");
    rsx! {
        li {
            form {
                id: "{id}-form",
                aria_label: translate!(i18, "add_user_form_label"),
                class: "flex flex-col gap-2 p-2",
                onsubmit: move |event| {
                    let values = event.values();
                    let name = values[&input_label].as_value();
                    let color = serde_json::from_str(
                        &values["color-picker"].as_value()
                    ).unwrap();
                    spawn_forever(create_user(board_signals, UserData{ name, color }, on_add_user));
                    show_form.set(false);
                },
                TextInput {
                    id: "{id}-new-user-name-input",
                    label: input_label.clone(),
                }
                ColorPicker { }
                div {
                    class: "flex flex-row gap-2 items-center justify-center",
                    ConfirmButton { label: translate!(i18, "add_user_button_label") }
                    CancelButton {
                        label: translate!(i18, "cancel_adding_new_user_button_label"),
                        editing: show_form,
                    }
                }
            }
        }
    }
}

async fn create_user(
    signals: BoardSignals,
    user_data: UserData,
    on_add_user: EventHandler<UserId>,
) {
    match requests::create_user(signals.url, signals.board, user_data).await {
        Ok((user_id, _)) => on_add_user.call(user_id),
        Err(e) => log::info!("Error creating user: {:?}", e),
    }
}
