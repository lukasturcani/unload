use dioxus::prelude::*;

use crate::themes::Theme;

#[component]
pub fn Tooltip(
    content: ReadOnlySignal<String>,
    position: Option<&'static str>,
    dir: Option<&'static str>,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "border rounded-lg shadow-sm {} {}",
        theme.border_color, theme.bg_color_2
    );
    let position = position.unwrap_or("-top-10 -left-2");
    rsx! {
        div {
            dir,
            div {
                role: "tooltip",
                class: "
                    pointer-events-none
                    absolute {position} z-10
                    w-max px-3 py-2 text-sm
                    invisible group-hover:visible
                    {style}
                ",
                p { {content} }
            }
        }
    }
}
