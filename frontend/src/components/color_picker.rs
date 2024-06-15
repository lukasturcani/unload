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
                class: "grid grid-cols-4 gap-4",
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
                            class: "peer",
                            required: true,
                            r#type: "radio",
                            name: "color-picker",
                            checked: selected_color.map_or(false, |c| color == c),
                        }
                        div { class: "inline-block size-6 {radio_style} {color_to_bg(color)}" }
                    }
                }
            }
        }
    }
}

fn color_to_bg(color: Color) -> &'static str {
    match color {
        Color::Black => "bg-black",
        Color::White => "bg-white",
        Color::Gray => "bg-gray-400",
        Color::Silver => "bg-slate-500",
        Color::Maroon => "bg-rose-400",
        Color::Red => "bg-red-600",
        Color::Purple => "bg-purple-600",
        Color::Fushsia => "bg-fuchsia-400",
        Color::Green => "bg-emerald-500",
        Color::Lime => "bg-lime-500",
        Color::Olive => "bg-indigo-400",
        Color::Yellow => "bg-yellow-400",
        Color::Navy => "bg-amber-200",
        Color::Blue => "bg-blue-400",
        Color::Teal => "bg-teal-300",
        Color::Aqua => "bg-cyan-500",
    }
}
