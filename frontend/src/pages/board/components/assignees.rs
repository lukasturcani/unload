use dioxus::prelude::*;
use shared_models::UserId;

use crate::pages::board::{
    components::{selector_toggle::SelectorToggle, UserIcon},
    model::Users,
};

#[component]
pub fn Assignees(
    id: String,
    assignees: Signal<Vec<UserId>>,
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
