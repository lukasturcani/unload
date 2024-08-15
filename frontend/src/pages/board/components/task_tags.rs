use dioxus::prelude::*;
use shared_models::TagId;

use crate::pages::board::{
    components::{selector_toggle::SelectorToggle, TaskTagIcon},
    model::Tags,
};

#[component]
pub fn TaskTags(
    id: String,
    tags: Signal<Vec<TagId>>,
    select_tags: Signal<bool>,
    on_delete_tag: EventHandler<TagId>,
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
                    on_delete_tag,
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
