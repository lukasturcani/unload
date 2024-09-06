use dioxus::prelude::*;
use dioxus_sdk::{i18n::*, translate};
use shared_models::{TagData, TagId};

use crate::{
    components::{
        color_picker::ColorPicker,
        form::{CancelButton, ConfirmButton},
        input::TextInput,
    },
    pages::board::{
        components::assignment_list::{
            AssignmentList, AssignmentListItem, ShowSelectionListFormButton,
        },
        model::Tags,
        requests::{self, BoardSignals},
    },
};

#[component]
pub fn TagSelection(
    id: String,
    tags: ReadOnlySignal<Vec<TagId>>,
    on_assign_tag: EventHandler<TagId>,
    on_add_tag: EventHandler<TagId>,
) -> Element {
    let i18 = use_i18();
    let tags = tags.read();
    let tag_data = use_context::<Signal<Tags>>();
    let tag_data = &tag_data.read().0;
    let mut unassigned = Vec::with_capacity(tag_data.len() - tags.len());
    for (user_id, user) in tag_data.iter() {
        if !tags.contains(user_id) {
            unassigned.push((*user_id, user.clone()));
        }
    }
    unassigned.sort_by_key(|(_, tag)| tag.name.to_lowercase());
    rsx! {
        section {
            aria_label: translate!(i18, "tag_selection_section_label"),
            AssignmentList {
                body: rsx! {
                    for (tag_id, tag) in unassigned {
                        TagListItem { key: "{tag_id}", tag_id, tag, on_assign_tag }
                    }
                    AddTagListItem { id, key: "{\"add-tag\"}", on_add_tag }
                }
            }
        }
    }
}

#[component]
fn TagListItem(tag_id: TagId, tag: TagData, on_assign_tag: EventHandler<TagId>) -> Element {
    let label = format!("assign {} to task", tag.name);
    rsx! {
        AssignmentListItem {
            content: tag.name,
            color: tag.color,
            aria_label: label,
            onclick: move |_| on_assign_tag.call(tag_id),
        }
    }
}

#[component]
fn AddTagListItem(id: String, on_add_tag: EventHandler<TagId>) -> Element {
    let i18 = use_i18();
    let show_form = use_signal(|| false);
    rsx! {
        li {
            if show_form() {
                AddTagListForm { id, show_form, on_add_tag }
            } else {
                ShowSelectionListFormButton {
                    r#for: "{id}-form",
                    content: translate!(i18, "add_tag_button_label"),
                    show_form,
                }
            }
        }
    }
}

#[component]
fn AddTagListForm(id: String, show_form: Signal<bool>, on_add_tag: EventHandler<TagId>) -> Element {
    let i18 = use_i18();
    let board_signals = BoardSignals::default();
    let input_label = translate!(i18, "tag_name_input_label");
    rsx! {
        li {
            form {
                id: "{id}-form",
                aria_label: translate!(i18, "add_tag_form_label"),
                class: "flex flex-col gap-2 p-2",
                onsubmit: move |event| {
                    let values = event.values();
                    let name = values[&input_label].as_value();
                    let color = serde_json::from_str(
                        &values["color-picker"].as_value()
                    ).unwrap();
                    spawn_forever(create_tag(board_signals, TagData{ name, color }, on_add_tag));
                    show_form.set(false);
                },
                TextInput {
                    id: "{id}-tag-name-input",
                    label: input_label.clone(),
                }
                ColorPicker { }
                div {
                    class: "flex flex-row gap-2 items-center justify-center",
                    ConfirmButton { label: "add tag" }
                    CancelButton {
                        label: "cancel adding tag",
                        editing: show_form,
                    }
                }
            }
        }
    }
}

async fn create_tag(signals: BoardSignals, tag_data: TagData, on_add_tag: EventHandler<TagId>) {
    match requests::create_tag(signals.url, signals.board, tag_data).await {
        Ok((tag_id, _)) => on_add_tag.call(tag_id),
        Err(e) => log::info!("Error creating tag: {:?}", e),
    }
}
