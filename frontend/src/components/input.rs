use dioxus::prelude::*;

use crate::{commands::ScrollTarget, themes::Theme};

#[component]
pub fn TextInput(id: String, label: String, value: Option<String>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        text-base
        rounded-lg
        border
        {} {} {}
    ",
        theme.border_color, theme.bg_color_2, theme.focus_color
    );
    let mut scroll = use_context::<Signal<ScrollTarget>>();
    let name = label.clone();
    rsx! {
        label {
            class: "text-sm",
            r#for: "{id}",
            {label}
        }
        input {
            id: id.clone(),
            class: "p-2.5 {style}",
            name,
            required: true,
            value,
            onmounted: move |event| {
                let id = id.clone();
                async move {
                    let _ = event.set_focus(true).await;
                    scroll.set(ScrollTarget(Some(id.clone())));
                }
            },
        }
    }
}

#[component]
pub fn DateInput(
    id: String,
    label: String,
    value: Option<String>,
    oninput: EventHandler<FormEvent>,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        text-base
        rounded-lg
        border
        {} {} {}
    ",
        theme.border_color, theme.bg_color_2, theme.focus_color
    );
    let name = label.clone();
    rsx! {
        label {
            class: "text-sm",
            r#for: "{id}",
            {label}
        }
        input {
            oninput: move |event| oninput.call(event),
            id,
            class: "p-2.5 {style}",
            r#type: "date",
            name,
            value,
        }
    }
}
