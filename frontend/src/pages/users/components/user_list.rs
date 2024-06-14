use dioxus::prelude::*;
use shared_models::UserEntry;

use crate::pages::users::model::UserEntries;

#[component]
pub fn UserList() -> Element {
    let users = use_context::<Signal<UserEntries>>();
    rsx! {
        ul {
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
            { user.name }
        }
    }
}
