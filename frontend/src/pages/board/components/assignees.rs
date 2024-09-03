use dioxus::prelude::*;
use dioxus_sdk::{i18n::*, translate};
use shared_models::UserId;

use crate::pages::board::{
    components::{
        selector_toggle::SelectorToggle,
        user_icon::{FilteringUserIcon, UserIcon},
    },
    model::Users,
};

#[component]
pub fn Assignees(
    id: String,
    assignees: ReadOnlySignal<Vec<UserId>>,
    select_assignees: Signal<bool>,
    icon_size: Option<&'static str>,
    tooltip_position: Option<&'static str>,
    dir: Option<&'static str>,
    on_toggle_selector: EventHandler<bool>,
) -> Element {
    let i18 = use_i18();
    let users = use_context::<Signal<Users>>();
    let users = &users.read().0;
    let size = icon_size.unwrap_or("size-6");
    rsx! {
        section {
            id,
            aria_label: translate!(i18, "assignees_section_label"),
            class: "flex flex-row flex-wrap items-center gap-2",
            for &user_id in assignees.read().iter() {
                UserIcon {
                    user_id,
                    user_data: users[&user_id].clone(),
                    size,
                    tooltip_position,
                    dir
                }
            }
            SelectorToggle {
                show_selector: select_assignees,
                aria_label: "toggle assignee selection",
                tooltip: "Assign User",
                size,
                tooltip_position,
                dir,
                on_toggle_selector,
            }
        }
    }
}

#[component]
pub fn FilteringAssignees(
    id: String,
    assignees: ReadOnlySignal<Vec<UserId>>,
    select_assignees: Signal<bool>,
    icon_size: Option<&'static str>,
    tooltip_position: Option<&'static str>,
    dir: Option<&'static str>,
    on_toggle_selector: EventHandler<bool>,
) -> Element {
    let users = use_context::<Signal<Users>>();
    let users = &users.read().0;
    let size = icon_size.unwrap_or("size-6");
    rsx! {
        section {
            id,
            "aria-label": "assignees",
            class: "flex flex-row flex-wrap items-center gap-2",
            for &user_id in assignees.read().iter() {
                FilteringUserIcon {
                    user_id,
                    user_data: users[&user_id].clone(),
                    size,
                    tooltip_position,
                    dir
                }
            }
            SelectorToggle {
                show_selector: select_assignees,
                aria_label: "toggle assignee selection",
                tooltip: "Assign User",
                size,
                tooltip_position,
                dir,
                on_toggle_selector,
            }
        }
    }
}
