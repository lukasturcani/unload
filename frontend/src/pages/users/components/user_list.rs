use dioxus::prelude::*;
use shared_models::UserEntry;

use crate::{pages::users::model::UserEntries, themes::Theme};

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
            class: style,
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
            class: "px-3 py-1",
            { user.name }
        }
    }
}
