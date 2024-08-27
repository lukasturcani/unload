use dioxus::prelude::*;
use shared_models::Color;

use crate::{commands::ScrollTarget, themes::Theme};

#[component]
pub fn AssignmentList(body: Element) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        rounded-lg shadow
        border
        divide-y
        {} {}
    ",
        theme.border_color, theme.divide_color
    );
    rsx! {
        ul {
            class: "text-sm {style}",
            {body}
        }
    }
}

#[component]
pub fn AssignmentListItem(
    content: String,
    color: Color,
    aria_label: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = "sm:hover:underline";
    let color = match color {
        Color::Black => theme.color1_text,
        Color::White => theme.color2_text,
        Color::Gray => theme.color3_text,
        Color::Silver => theme.color4_text,
        Color::Maroon => theme.color5_text,
        Color::Red => theme.color6_text,
        Color::Purple => theme.color7_text,
        Color::Fushsia => theme.color8_text,
        Color::Green => theme.color9_text,
        Color::Lime => theme.color10_text,
        Color::Olive => theme.color11_text,
        Color::Yellow => theme.color12_text,
        Color::Navy => theme.color13_text,
        Color::Blue => theme.color14_text,
        Color::Teal => theme.color15_text,
        Color::Aqua => theme.color16_text,
    };
    rsx! {
        li {
            button {
                class: "px-4 py-2 w-full text-left {style} {color}",
                onclick: move |event| onclick.call(event),
                {content}
            }
        }
    }
}

#[component]
pub fn ShowSelectionListFormButton(
    r#for: String,
    content: String,
    show_form: Signal<bool>,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let mut scroll_target = use_context::<Signal<ScrollTarget>>();
    let style = format!(
        "sm:hover:underline active:underline {}",
        theme.action_text_color
    );
    rsx! {
        button {
            class: "px-4 py-2 w-full text-left {style}",
            onclick: move |_| {
                scroll_target.set(ScrollTarget(Some(r#for.clone())));
                show_form.set(true)
            },
            {content}
        }
    }
}
