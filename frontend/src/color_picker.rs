use dioxus::prelude::*;
use shared_models::Color;

const COLORS: [(Color, &str); 16] = [
    (Color::Black, "Black"),
    (Color::White, "White"),
    (Color::Gray, "Gray"),
    (Color::Silver, "Silver"),
    (Color::Maroon, "Maroon"),
    (Color::Red, "Red"),
    (Color::Purple, "Purple"),
    (Color::Fushsia, "Fushsia"),
    (Color::Green, "Green"),
    (Color::Lime, "Lime"),
    (Color::Olive, "Olive"),
    (Color::Yellow, "Yellow"),
    (Color::Navy, "Navy"),
    (Color::Blue, "Blue"),
    (Color::Teal, "Teal"),
    (Color::Aqua, "Aqua"),
];

pub fn bg_class(color: &Color) -> &'static str {
    match color {
        Color::Black => "bg-black",
        Color::White => "bg-white",
        Color::Gray => "bg-gray-500",
        Color::Silver => "bg-slate-500",
        Color::Maroon => "bg-red-950",
        Color::Red => "bg-red-600",
        Color::Purple => "bg-purple-950",
        Color::Fushsia => "bg-fuchsia-600",
        Color::Green => "bg-green-900",
        Color::Lime => "bg-lime-500",
        Color::Olive => "bg-lime-950",
        Color::Yellow => "bg-yellow-400",
        Color::Navy => "bg-blue-950",
        Color::Blue => "bg-blue-700",
        Color::Teal => "bg-teal-400",
        Color::Aqua => "bg-cyan-400",
    }
}

pub fn bg_hover_class(color: &Color) -> &'static str {
    match color {
        Color::Black => "sm:hover:bg-black",
        Color::White => "sm:hover:bg-white",
        Color::Gray => "sm:hover:bg-gray-500",
        Color::Silver => "sm:hover:bg-slate-500",
        Color::Maroon => "sm:hover:bg-red-950",
        Color::Red => "sm:hover:bg-red-600",
        Color::Purple => "sm:hover:bg-purple-950",
        Color::Fushsia => "sm:hover:bg-fuchsia-600",
        Color::Green => "sm:hover:bg-green-900",
        Color::Lime => "sm:hover:bg-lime-500",
        Color::Olive => "sm:hover:bg-lime-950",
        Color::Yellow => "sm:hover:bg-yellow-400",
        Color::Navy => "sm:hover:bg-blue-950",
        Color::Blue => "sm:hover:bg-blue-700",
        Color::Teal => "sm:hover:bg-teal-400",
        Color::Aqua => "sm:hover:bg-cyan-400",
    }
}

pub fn border_class(color: &Color) -> &'static str {
    match color {
        Color::Black => "border-black",
        Color::White => "border-white",
        Color::Gray => "border-gray-500",
        Color::Silver => "border-slate-500",
        Color::Maroon => "border-red-950",
        Color::Red => "border-red-600",
        Color::Purple => "border-purple-950",
        Color::Fushsia => "border-fuchsia-600",
        Color::Green => "border-green-900",
        Color::Lime => "border-lime-500",
        Color::Olive => "border-lime-950",
        Color::Yellow => "border-yellow-400",
        Color::Navy => "border-blue-950",
        Color::Blue => "border-blue-700",
        Color::Teal => "border-teal-400",
        Color::Aqua => "border-cyan-400",
    }
}

pub fn text_class(color: &Color) -> &'static str {
    match color {
        Color::Black => "text-black",
        Color::White => "text-white",
        Color::Gray => "text-gray-500",
        Color::Silver => "text-slate-500",
        Color::Maroon => "text-red-950",
        Color::Red => "text-red-600",
        Color::Purple => "text-purple-950",
        Color::Fushsia => "text-fuchsia-600",
        Color::Green => "text-green-900",
        Color::Lime => "text-lime-500",
        Color::Olive => "text-lime-950",
        Color::Yellow => "text-yellow-400",
        Color::Navy => "text-blue-950",
        Color::Blue => "text-blue-700",
        Color::Teal => "text-teal-400",
        Color::Aqua => "bg-cyan-400",
    }
}

#[component]
pub fn ColorPicker<'a>(
    cx: Scope<'a>,
    default_color: Option<Color>,
    on_pick_color: EventHandler<'a, Color>,
) -> Element<'a> {
    let selected = use_state(cx, || *default_color);
    cx.render(rsx! {
        div {
            class: "flex-1 flex grid grid-cols-4 gap-4 justify-items-center",
            for (color, name, class) in
                COLORS
                .iter()
                .map(|(color, name)| (color, name, bg_class(color)))
            {rsx! {
                if selected.map_or(false, |selected_color| selected_color == *color) {rsx!{
                    div {
                        class: "group/color-tooltip relative",
                        div {
                            class: "
                                w-8 h-8 rounded cursor-pointer {class}
                                ring-blue-600 ring-2",
                            onclick: |_| {
                                selected.set(Some(*color));
                                on_pick_color.call(*color);
                            },
                        },
                        div {
                            class: "
                                pointer-events-none absolute -top-10 left-0 w-max
                                opacity-0 transition-opacity group-hover/color-tooltip:opacity-100
                                z-10 inline-block px-3 py-2 text-sm font-medium text-white
                                rounded-lg shadow-sm opacity-0 tooltip bg-gray-800
                                border border-gray-700",
                            "{name}"
                            div {
                                class: "tooltip-arrow",
                                "data-popper-arrow": "",
                            }
                        }
                    }
                }} else {rsx!{
                    div {
                        class: "group/color-tooltip relative",
                        div {
                            class: "w-8 h-8 rounded cursor-pointer {class}",
                            onclick: |_| {
                                selected.set(Some(*color));
                                on_pick_color.call(*color);
                            },
                        },
                        div {
                            class: "
                                pointer-events-none absolute -top-10 left-0 w-max
                                opacity-0 transition-opacity group-hover/color-tooltip:opacity-100
                                z-10 inline-block px-3 py-2 text-sm font-medium text-white
                                rounded-lg shadow-sm opacity-0 tooltip bg-gray-800
                                border border-gray-700",
                            "{name}"
                            div {
                                class: "tooltip-arrow",
                                "data-popper-arrow": "",
                            }
                        }
                    }

                }}
            }}
        }
    })
}
