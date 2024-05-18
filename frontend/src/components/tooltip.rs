use dioxus::prelude::*;

use crate::themes::Theme;

#[component]
pub fn Tooltip(content: String, position: Option<String>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "border rounded-lg shadow-sm {} {}",
        theme.border_color, theme.bg_color_2
    );
    let position = position.unwrap_or("-top-10 -left-2".to_string());
    rsx! {
        div {
            role: "tooltip",
            class: "
                pointer-events-none
                absolute {position} z-10
                w-max px-3 py-2 text-sm
                invisible peer-hover:visible
                {style}
            ",
            p { {content} }
        }

    }
}
