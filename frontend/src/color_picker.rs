use dioxus::prelude::*;
use shared_models::Color;

const COLORS: [(Color, &str, &str); 16] = [
    (Color::Black, "Black", "bg-black"),
    (Color::White, "White", "bg-white"),
    (Color::Gray, "Gray", "bg-gray-500"),
    (Color::Silver, "Silver", "bg-slate-500"),
    (Color::Maroon, "Maroon", "bg-red-950"),
    (Color::Red, "Red", "bg-red-600"),
    (Color::Purple, "Purple", "bg-purple-950"),
    (Color::Fushsia, "Fushsia", "bg-fuchsia-600"),
    (Color::Green, "Green", "bg-green-900"),
    (Color::Lime, "Lime", "bg-lime-500"),
    (Color::Olive, "Olive", "bg-lime-950"),
    (Color::Yellow, "Yellow", "bg-yellow-400"),
    (Color::Navy, "Navy", "bg-blue-950"),
    (Color::Blue, "Blue", "bg-blue-700"),
    (Color::Teal, "Teal", "bg-teal-400"),
    (Color::Aqua, "Aqua", "bg-cyan-400"),
];

#[component]
pub fn ColorPicker(cx: Scope) -> Element {
    let selected = use_state(cx, || Color::Black);
    cx.render(rsx! {
        div {
            class: "flex-1 flex grid grid-cols-4 gap-4 justify-items-center",
            for (color, name, class) in COLORS.iter() {rsx! {
                if color == &**selected {rsx!{
                    div {
                        class: "group relative",
                        div {
                            class: "
                                w-8 h-8 rounded cursor-pointer {class}
                                ring-blue-500 dark:ring-blue-600 ring-2",
                            onclick: |_| selected.set(*color),
                        },
                        div {
                            class: "
                                pointer-events-none absolute -top-10 left-0 w-max
                                opacity-0 transition-opacity group-hover:opacity-100
                                z-10 inline-block px-3 py-2 text-sm font-medium text-white
                                bg-gray-900 rounded-lg shadow-sm opacity-0 tooltip dark:bg-gray-800",
                            "{name}"
                            div {
                                class: "tooltip-arrow",
                                "data-popper-arrow": "",
                            }
                        }
                    }
                }} else {rsx!{
                    div {
                        class: "group relative",
                        div {
                            class: "w-8 h-8 rounded cursor-pointer {class}",
                            onclick: |_| selected.set(*color),
                        },
                        div {
                            class: "
                                pointer-events-none absolute -top-10 left-0 w-max
                                opacity-0 transition-opacity group-hover:opacity-100
                                z-10 inline-block px-3 py-2 text-sm font-medium text-white
                                bg-gray-900 rounded-lg shadow-sm opacity-0 tooltip dark:bg-gray-800",
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
