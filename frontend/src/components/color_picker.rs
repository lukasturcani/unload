use dioxus::prelude::*;
use shared_models::Color;

use crate::themes::Theme;

#[component]
pub fn ColorPicker(selected_color: Option<Color>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let fieldset_style = format!("rounded-lg border {}", theme.border_color);
    let legend_style = "text-sm";
    let radio_style = "
        rounded-md
        ease-in-out duration-100
        hover:scale-125 peer-checked:scale-125
    ";
    rsx! {
        fieldset {
            class: "flex flex-row items-center justify-center p-2 {fieldset_style}",
            legend {
                class: legend_style,
                "Color"
            }
            div {
                class: "grid grid-cols-4 gap-4 group filled",
                for color in [
                    Color::Black,
                    Color::White,
                    Color::Gray,
                    Color::Silver,
                    Color::Maroon,
                    Color::Red,
                    Color::Purple,
                    Color::Fushsia,
                    Color::Green,
                    Color::Lime,
                    Color::Olive,
                    Color::Yellow,
                    Color::Navy,
                    Color::Blue,
                    Color::Teal,
                    Color::Aqua,
                ] {
                    label {
                        class: "flex flex-row items-center gap-2",
                        input {
                            value: serde_json::to_string(&color).unwrap(),
                            class: format!("peer {}", theme.focus_color),
                            required: true,
                            r#type: "radio",
                            name: "color-picker",
                            checked: selected_color.map_or(false, |c| color == c),
                        }
                        div { class: "inline-block size-6 {radio_style} {color_to_bg(&theme, color)}" }
                    }
                }
            }
        }
    }
}

fn color_to_bg(theme: &Theme, color: Color) -> &'static str {
    match color {
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
    }
}
