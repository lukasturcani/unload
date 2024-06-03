use dioxus::prelude::*;
use itertools::Itertools;
use shared_models::{Color, TagId, TaskSize, UserId};

use crate::{color_picker, model::Model, styles};

#[component]
pub fn FilterBar() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-3 gap-2",
            TagFilter {}
            UserFilter {}
        }
    }
}

fn tag_bg(model: Signal<Model>, tag_id: &TagId, tag_color: &Color) -> String {
    if model.read().tag_filter.contains(tag_id) {
        format!("{} ring ring-blue-500", color_picker::bg_class(tag_color))
    } else {
        "bg-inherit".into()
    }
}

#[component]
pub fn TagFilter() -> Element {
    let mut model = use_context::<Signal<Model>>();
    rsx! {
        div {
            class: "
                flex flex-row flex-wrap gap-2
                justify-center sm:justify-start
            ",
            for (tag_id, tag) in model
                .read()
                .tags
                .iter()
                .sorted_by(|(_, tag1), (_, tag2)| tag1.name.cmp(&tag2.name))
            {
                span {
                    class: "
                        text-sm font-medium px-2.5 py-0.5 rounded
                        {tag_bg(model, tag_id, &tag.color)}
                        {color_picker::bg_hover_class(&tag.color)}
                        text-white cursor-pointer
                        border-2 {color_picker::border_class(&tag.color)}
                        flex flex-row gap-2
                    ",
                    onclick: {
                        let tag_id = *tag_id;
                        move |event| {
                            event.stop_propagation();
                            let mut model = model.write();
                            if model.tag_filter.contains(&tag_id) {
                                model.tag_filter.remove(&tag_id);
                            } else {
                                model.tag_filter.insert(tag_id);
                            }
                        }
                    },
                    "# {tag.name}",
                }
            }

        }
    }
}

fn user_bg(model: Signal<Model>, user_id: &UserId, user_color: &Color) -> String {
    if model.read().user_filter.contains(user_id) {
        format!("{} ring ring-blue-500", color_picker::bg_class(user_color))
    } else {
        "bg-inherit".into()
    }
}

#[component]
pub fn UserFilter() -> Element {
    let mut model = use_context::<Signal<Model>>();
    rsx! {
        div {
            class: "
                flex flex-row flex-wrap gap-2
                justify-center sm:justify-start
            ",
            for (&user_id, user) in model.read().users.iter() {
                div {
                    class: "group relative",
                    onclick: |event| event.stop_propagation(),
                    div {
                        class: "
                            w-6 h-6 rounded cursor-pointer
                            border-2 {color_picker::border_class(&user.color)}
                            {user_bg(model, &user_id, &user.color)}
                            {color_picker::bg_hover_class(&user.color)}
                        ",
                        onclick: move |event| {
                            event.stop_propagation();
                            let mut model = model.write();
                            if model.user_filter.contains(&user_id) {
                                model.user_filter.remove(&user_id);
                            } else {
                                model.user_filter.insert(user_id);
                            }
                        },
                    },
                    div {
                        class: styles::TOOLTIP,
                        "{user.name}"
                        div {
                            class: "tooltip-arrow",
                            "data-popper-arrow": "",
                        }
                    }
                }
            }
        }
    }
}

fn size_bg(model: Signal<Model>, size: &TaskSize) -> &'static str {
    if model
        .read()
        .size_filter
        .map_or(false, |filter| &filter == size)
    {
        match size {
            TaskSize::Small => "bg-emerald-700 ring ring-blue-500",
            TaskSize::Medium => "bg-yellow-900 ring ring-blue-500",
            TaskSize::Large => "bg-red-900 ring ring-blue-500",
        }
    } else {
        "bg-inherit"
    }
}
