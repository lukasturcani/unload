use dioxus::prelude::*;
use shared_models::TagId;

use crate::pages::board::{
    components::{
        selector_toggle::SelectorToggle,
        tag_icon::{FilteringTaskTagIcon, TaskTagIcon},
    },
    model::Tags,
};

#[component]
pub fn FilteringTaskTags(
    id: String,
    tags: Signal<Vec<TagId>>,
    select_tags: Signal<bool>,
    on_unassign_tag: EventHandler<TagId>,
    on_toggle_selector: EventHandler<bool>,
) -> Element {
    let tag_data = use_context::<Signal<Tags>>();
    let tag_data = &tag_data.read().0;
    rsx! {
        section {
            id,
            "aria-label": "tags",
            class: "flex flex-row flex-wrap gap-2 items-center",
            for &tag_id in tags.read().iter() {
                FilteringTaskTagIcon {
                    tag_id,
                    tag_data: tag_data[&tag_id].clone(),
                    on_unassign_tag,
                }
            }
            SelectorToggle {
                show_selector: select_tags,
                aria_label: "toggle tag selection",
                tooltip: "Add Tag",
                size: "size-6",
                on_toggle_selector,
            }
        }
    }
}

#[component]
pub fn TaskTags(
    id: String,
    tags: Signal<Vec<TagId>>,
    select_tags: Signal<bool>,
    on_unassign_tag: EventHandler<TagId>,
    on_toggle_selector: EventHandler<bool>,
) -> Element {
    let tag_data = use_context::<Signal<Tags>>();
    let tag_data = &tag_data.read().0;
    rsx! {
        section {
            id,
            "aria-label": "tags",
            class: "flex flex-row flex-wrap gap-2 items-center",
            for &tag_id in tags.read().iter() {
                TaskTagIcon {
                    tag_id,
                    tag_data: tag_data[&tag_id].clone(),
                    on_unassign_tag,
                }
            }
            SelectorToggle {
                show_selector: select_tags,
                aria_label: "toggle tag selection",
                tooltip: "Add Tag",
                size: "size-6",
                on_toggle_selector,
            }
        }
    }
}
