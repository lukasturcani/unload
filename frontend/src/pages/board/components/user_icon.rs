use dioxus::prelude::*;
use dioxus_sdk::{i18n::use_i18, translate};
use shared_models::{Color, UserData, UserId};

use crate::{components::tooltip::Tooltip, pages::board::model::UserFilter, themes::Theme};

#[component]
pub fn UserIcon(
    user_id: UserId,
    user_data: UserData,
    size: &'static str,
    tooltip_position: Option<&'static str>,
    dir: Option<&'static str>,
) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let color = match user_data.color {
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
    let style = "
        rounded border-2
        sm:hover:border-4 active:border-4 sm:hover:scale-110 active:scale-110
    ";
    let aria_label = format!(
        "{}: {}",
        translate!(i18, "toggle_user_filter_button_label"),
        user_data.name,
    );
    rsx! {
        div {
            class: "group relative",
            div {
                class: "block {size} {style} {color}",
                aria_label,
                div { class: "size-full" }
            }
            Tooltip { content: user_data.name, position: tooltip_position, dir }
        }
    }
}

#[component]
pub fn FilteringUserIcon(
    user_id: UserId,
    user_data: UserData,
    size: &'static str,
    tooltip_position: Option<&'static str>,
    dir: Option<&'static str>,
) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let mut user_filter = use_context::<Signal<UserFilter>>();
    let color = match user_data.color {
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
    let style = "
        rounded border-2
        sm:hover:border-4 active:border-4 sm:hover:scale-110 active:scale-110
    ";
    let aria_label = format!(
        "{}: {}",
        translate!(i18, "toggle_user_filter_button_label"),
        user_data.name,
    );
    rsx! {
        div {
            class: "group relative",
            button {
                class: "block {size} {style} {color}",
                aria_label,
                aria_pressed: user_filter.read().0.contains(&user_id),
                onclick: move |_| {
                    let mut user_filter = user_filter.write();
                    if user_filter.0.contains(&user_id) {
                        user_filter.0.remove(&user_id);
                    } else {
                        user_filter.0.insert(user_id);
                    }
                },
                div { class: "size-full" }
            }
            Tooltip { content: user_data.name, position: tooltip_position, dir }
        }
    }
}
