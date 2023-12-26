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
    cx.render(rsx! {
        div {
            class: "flex-1 flex grid grid-cols-4 gap-4 justify-items-center",
            for (color, name, class) in COLORS.iter() {rsx! {
                div {
                    "data-tooltip-target": "{name}-tooltip",
                    class: "w-8 h-8 rounded cursor-pointer {class}",
                },
                div {
                    id: "{name}-tooltip",
                    role: "tooltip",
                    class: "absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white transition-opacity duration-300 bg-gray-900 rounded-lg shadow-sm opacity-0 tooltip dark:bg-gray-700",
                    "{name}"
                    div {
                        class: "tooltip-arrow",
                        "data-popper-arrow": "",
                    }
                }
            }}
        }
    })
}
