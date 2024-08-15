use dioxus::prelude::*;
use shared_models::{Color, TagData, TagId};

use crate::{components::icons::CancelIcon, pages::board::model::TagFilter, themes::Theme};

#[component]
pub fn FilterBarTagIcon(tag_id: TagId, tag_data: TagData) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let color = match tag_data.color {
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
    let style = "rounded border-2";
    rsx! {
        div {
            class: "
                group
                flex flex-row items-center
                px-1.5 py-0.5
                {style} {color}
            ",
            FilterButton { tag_id, tag_data }
        }
    }
}

#[component]
fn FilterButton(arial_label: String, content: String, tag_id: TagId) -> Element {
    let mut tag_filter = use_context::<Signal<TagFilter>>();
    let pressed = tag_filter.read().0.contains(&tag_id);
    rsx! {
        button {
            class: "text-sm pr-1",
            "aria-label": "toggle {tag_data.name} filter",
            aria_label,
            "aria-pressed": pressed,
            onclick: move |_| {
                let mut tag_filter = tag_filter.write();
                if tag_filter.0.contains(&tag_id) {
                    tag_filter.0.remove(&tag_id);
                } else {
                    tag_filter.0.insert(tag_id);
                }
            },
            "# {tag_data.name}"
        }
    }
}

#[component]
pub fn TagIcon(
    tag_id: TagId,
    remove_tag_label: String,
    on_unassign_tag: EventHandler<TagId>,
    body: Element,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let color = match tag_data.color {
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
    let style = "rounded border-2";
    let delete_tag_button_style = "rounded active:border sm:hover:border";
    rsx! {
        div {
            class: "
                group
                flex flex-row items-center
                px-1.5 py-0.5
                {style} {color}
            ",
            {body}
            button {
                aria_label: remove_tag_label,
                class: "size-5 p-0.5 {delete_tag_button_style}",
                onclick: move |_| on_unassign_tag.call(tag_id),
                CancelIcon {}
            }
        }
    }
}

#[component]
pub fn FilteringTaskTagIcon(
    tag_id: TagId,
    tag_data: TagData,
    on_unassign_tag: EventHandler<TagId>,
) -> Element {
    rsx! {
        TagIcon {
            tag_id,
            tag_data,
            on_unassign_tag,
            body: rsx! {
                FilterButton { tag_id, tag_data }
            }
        }
    }
}
