use dioxus::prelude::*;
use dioxus_sdk::{i18n::*, translate};
use shared_models::{TagData, TagId};

use crate::pages::board::{
    components::{
        selector_toggle::SelectorToggle,
        tag_icon::{FilteringTaskTagIcon, TaskNewTagIcon, TaskTagIcon},
    },
    model::Tags,
};

#[component]
pub fn FilteringTaskTags(
    id: String,
    tags: ReadOnlySignal<Vec<TagId>>,
    select_tags: Signal<bool>,
    on_unassign_tag: EventHandler<TagId>,
    on_toggle_selector: EventHandler<bool>,
) -> Element {
    let i18 = use_i18();
    let tag_data = use_context::<Signal<Tags>>();
    let tag_data = &tag_data.read().0;
    rsx! {
        section {
            id,
            aria_label: translate!(i18, "tags_section_label"),
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
                tooltip: translate!(i18, "add_tag_toggle_button_tooltip"),
                size: "size-6",
                on_toggle_selector,
            }
        }
    }
}

#[component]
pub fn TaskTags(
    id: String,
    tags: ReadOnlySignal<Vec<TagId>>,
    new_tags: Signal<Vec<TagData>>,
    select_tags: Signal<bool>,
    on_unassign_tag: EventHandler<TagId>,
    on_toggle_selector: EventHandler<bool>,
) -> Element {
    let i18 = use_i18();
    let tag_data = use_context::<Signal<Tags>>();
    let tag_data = &tag_data.read().0;
    rsx! {
        section {
            id,
            aria_label: translate!(i18, "tags_section_label"),
            class: "flex flex-row flex-wrap gap-2 items-center",
            for &tag_id in tags.read().iter() {
                TaskTagIcon {
                    tag_id,
                    tag_data: tag_data[&tag_id].clone(),
                    on_unassign_tag,
                }
            }
            for (tag_id, tag_data) in new_tags.read().iter().enumerate() {
                TaskNewTagIcon {
                    tag_id,
                    tag_data: tag_data.clone(),
                    on_unassign_tag: move |tag_id| {
                        new_tags.write().swap_remove(tag_id);
                    },
                }
            }
            SelectorToggle {
                show_selector: select_tags,
                tooltip: translate!(i18, "add_tag_toggle_button_tooltip"),
                size: "size-6",
                on_toggle_selector,
            }
        }
    }
}
