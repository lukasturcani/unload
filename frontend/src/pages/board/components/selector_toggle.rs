use dioxus::prelude::*;

use crate::{
    components::{icons::PlusIcon, tooltip::Tooltip},
    themes::Theme,
};

#[component]
pub fn SelectorToggle(
    show_selector: Signal<bool>,
    tooltip: ReadOnlySignal<String>,
    size: &'static str,
    tooltip_position: Option<&'static str>,
    dir: Option<&'static str>,
    on_toggle_selector: EventHandler<bool>,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("rounded border-2 {}", theme.button);
    rsx! {
        div {
            class: "group relative",
            button {
                aria_label: tooltip,
                class: "block {size} {style}",
                aria_pressed: show_selector(),
                onclick: move |_| {
                    let show = show_selector();
                    on_toggle_selector.call(show);
                    show_selector.set(!show);
                },
                PlusIcon {}
            }
            Tooltip { content: tooltip, position: tooltip_position, dir }
        }
    }
}
