use dioxus::prelude::*;
use shared_models::Color;

const COLORS: [Color; 16] = [
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
];

pub fn bg_class(color: &Color) -> &'static str {
    match color {
        Color::Black => "bg-orange-500",
        Color::White => "bg-teal-600",
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

pub fn bg_hover_class(color: &Color) -> &'static str {
    match color {
        Color::Black => "sm:hover:bg-orange-500",
        Color::White => "sm:hover:bg-teal-600",
        Color::Gray => "sm:hover:bg-gray-400",
        Color::Silver => "sm:hover:bg-slate-500",
        Color::Maroon => "sm:hover:bg-rose-400",
        Color::Red => "sm:hover:bg-red-600",
        Color::Purple => "sm:hover:bg-purple-600",
        Color::Fushsia => "sm:hover:bg-fuchsia-400",
        Color::Green => "sm:hover:bg-emerald-500",
        Color::Lime => "sm:hover:bg-lime-500",
        Color::Olive => "sm:hover:bg-indigo-400",
        Color::Yellow => "sm:hover:bg-yellow-400",
        Color::Navy => "sm:hover:bg-amber-200",
        Color::Blue => "sm:hover:bg-blue-400",
        Color::Teal => "sm:hover:bg-teal-300",
        Color::Aqua => "sm:hover:bg-cyan-500",
    }
}

pub fn border_class(color: &Color) -> &'static str {
    match color {
        Color::Black => "border-orange-500",
        Color::White => "border-teal-600",
        Color::Gray => "border-gray-400",
        Color::Silver => "border-slate-500",
        Color::Maroon => "border-rose-400",
        Color::Red => "border-red-600",
        Color::Purple => "border-purple-600",
        Color::Fushsia => "border-fuchsia-400",
        Color::Green => "border-emerald-500",
        Color::Lime => "border-lime-500",
        Color::Olive => "border-indigo-400",
        Color::Yellow => "border-yellow-400",
        Color::Navy => "border-amber-200",
        Color::Blue => "border-blue-400",
        Color::Teal => "border-teal-300",
        Color::Aqua => "border-cyan-500",
    }
}

#[component]
pub fn SelectingColorPicker(
    default_color: Option<Color>,
    on_pick_color: EventHandler<Color>,
) -> Element {
    let mut selected_signal = use_signal(|| default_color);
    let selected = selected_signal();
    rsx! {
        div {
            class: "flex-1 flex grid grid-cols-4 gap-4 justify-items-center",
            for (color, class) in
                COLORS
                .iter()
                .map(|color| (color, bg_class(color)))
            {
                if selected.map_or(false, |selected_color| selected_color == *color) {
                    div {
                        class: "
                            w-8 h-8 rounded cursor-pointer {class}
                            ring-blue-600 ring-2
                        ",
                        onclick: move |_| {
                            selected_signal.set(Some(*color));
                            on_pick_color.call(*color);
                        },
                    }
                } else {
                    div {
                        class: "w-8 h-8 rounded cursor-pointer {class}",
                        onclick: move |_| {
                            selected_signal.set(Some(*color));
                            on_pick_color.call(*color);
                        },
                    }
                }
            }
        }
    }
}

#[component]
pub fn ColorPicker(on_pick_color: EventHandler<Color>) -> Element {
    rsx! {
        div {
            class: "flex-1 flex grid grid-cols-4 gap-4 justify-items-center",
            onclick: |event| {
                event.stop_propagation();
            },
            for (&color, class) in
                COLORS
                .iter()
                .map(|color| (color, bg_class(color)))
            {
                div {
                    class: "w-8 h-8 rounded cursor-pointer {class}",
                    onclick: move |_| {
                        on_pick_color.call(color);
                    },
                }
            }
        }
    }
}
