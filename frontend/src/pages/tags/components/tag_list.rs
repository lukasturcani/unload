use dioxus::prelude::*;
use shared_models::{Color, TagEntry, TagId};

use crate::{
    commands::ScrollTarget,
    components::{
        color_picker::ColorPicker,
        form::ConfirmButton,
        icons::{ArchiveIcon, CancelIcon, EditIcon, TrashIcon},
        input::TextInput,
    },
    pages::tags::{
        model::{TagEntries, TagsUrl},
        requests,
    },
    themes::Theme,
};

#[component]
pub fn TagList() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
            border-y sm:border-x sm:rounded-lg
            divide-y
            {} {}
        ",
        theme.border_color, theme.divide_color,
    );
    let tags = use_context::<Signal<TagEntries>>();
    rsx! {
        ul {
            class: "overflow-y-auto w-full max-w-lg {style}",
            for tag in tags.read().0.iter() {
                TagListItem { tag: tag.clone() }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum State {
    EditingColor,
    EditingName,
    Show,
}

#[component]
fn TagListItem(tag: TagEntry) -> Element {
    let state = use_signal(|| State::Show);
    rsx! {
        match state() {
            State::Show => rsx! {
                li {
                    class: "
                        px-3 py-1
                        flex flex-row justify-between
                    ",
                    div {
                        class: "flex flex-row items-center gap-5 group filled",
                        ColorShow { tag_id: tag.id, color: tag.color, state }
                        NameShow { tag_id: tag.id, name: tag.name, state }

                    }
                    div {
                        class: "flex flex-row items-center gap-1",
                        ArchiveTagButton { tag_id: tag.id }
                        DeleteTagButton { tag_id: tag.id }
                    }
                }
            },
            State::EditingColor => rsx! {
                li {
                    class: "flex flex-row w-full items-center justify-center",
                    ColorSelect { tag_id: tag.id, color: tag.color, state }
                }
            },
            State::EditingName => rsx! {
                li {
                    class: "flex flex-row w-full items-center justify-center",
                    NameInput { tag_id: tag.id, name: tag.name, state }
                }
            },
        }
    }
}

#[component]
fn ColorSelect(tag_id: TagId, color: Color, state: Signal<State>) -> Element {
    let tags = use_context::<Signal<TagEntries>>();
    let url = use_context::<Signal<TagsUrl>>();
    rsx! {
        form {
            id: "tag-{tag_id}-color-form",
            "aria-label": "edit color",
            class: "flex flex-col gap-2 items-center p-2",
            onsubmit: move |event| {
                let color = serde_json::from_str(
                    &event.values()["color-picker"].as_value()
                ).unwrap();
                spawn_forever(requests::set_tag_color(tags, url, tag_id, color));
                state.set(State::Show);
            },
            ColorPicker { selected_color: color }
            div {
                class: "flex flex-row gap-2 items-center justify-center",
                ConfirmButton { label: "set color" }
                CancelButton { label: "cancel color update", state }
            }
        }
    }
}

#[component]
fn CancelButton(label: String, state: Signal<State>) -> Element {
    let style = "
        rounded-md
        border border-red-600
        stroke-red-600
        active:bg-red-600
        sm:hover:bg-red-600 sm:hover:stroke-white
    ";
    rsx! {
        button {
            "aria-label": label,
            class: "size-7 {style}",
            onclick: move |_| {
                state.set(State::Show);
            },
            CancelIcon {}
        }
    }
}

#[component]
fn ColorShow(tag_id: TagId, color: Color, state: Signal<State>) -> Element {
    let mut scroll_target = use_context::<Signal<ScrollTarget>>();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let color = match color {
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
    let style = format!("rounded {color}");
    rsx! {
        div {
            class: "flex flex-row items-center gap-1",
            div {
                class: "size-6 {style}",
            }
            button {
                class: "size-4",
                "aria-label": "edit color",
                onclick: move |_| {
                    scroll_target.set(
                        ScrollTarget(Some(format!("tag-{tag_id}-color-form")))
                    );
                    state.set(State::EditingColor)
                },
                EditIcon {}
            }
        }
    }
}

#[component]
fn NameInput(tag_id: TagId, name: String, state: Signal<State>) -> Element {
    let url = use_context::<Signal<TagsUrl>>();
    let tags = use_context::<Signal<TagEntries>>();
    rsx! {
        form {
            id: "tag-{tag_id}-name-form",
            "aria-label": "edit name",
            class: "flex flex-row gap-2 items-center p-2",
            onsubmit: move |event| {
                let name = event.values()["Name"].as_value();
                spawn_forever(requests::set_tag_name(tags, url, tag_id, name));
                state.set(State::Show);
            },
            TextInput {
                id: "tag-{tag_id}-name-input",
                label: "Name",
                value: name,
            }
            ConfirmButton { label: "set name" }
            CancelButton { label: "cancel name update", state }
        }
    }
}

#[component]
fn NameShow(tag_id: TagId, name: String, state: Signal<State>) -> Element {
    let mut scroll_target = use_context::<Signal<ScrollTarget>>();
    rsx! {
        div {
            class: "flex flex-row items-center gap-1",
            {name}
            button {
                class: "size-4",
                "aria-label": "edit name",
                onclick: move |_| {
                    scroll_target.set(
                        ScrollTarget(Some(format!("tag-{tag_id}-name-form")))
                    );
                    state.set(State::EditingName)
                },
                EditIcon {}
            }
        }
    }
}

#[component]
fn DeleteTagButton(tag_id: TagId) -> Element {
    let url = use_context::<Signal<TagsUrl>>();
    let tags = use_context::<Signal<TagEntries>>();
    let style = "stroke-red-600";
    rsx! {
        button {
            "aria-label": "delete tag",
            class: "block size-6 {style}",
            onclick: move |_| {
                spawn_forever(requests::delete_tag(tags, url, tag_id));
            },
            TrashIcon {}
        }
    }
}

#[component]
fn ArchiveTagButton(tag_id: TagId) -> Element {
    let url = use_context::<Signal<TagsUrl>>();
    let tags = use_context::<Signal<TagEntries>>();
    rsx! {
        button {
            "aria-label": "archive tag",
            class: "block size-6",
            onclick: move |_| {
                spawn_forever(requests::set_tag_archived(tags, url, tag_id));
            },
            ArchiveIcon {}
        }
    }
}
