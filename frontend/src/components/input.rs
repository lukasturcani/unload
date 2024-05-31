use dioxus::prelude::*;

use crate::themes::Theme;

#[component]
pub fn TextInput(id: String, label: String, value: Option<String>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        text-base
        rounded-lg
        border
        focus:ring-blue-500 focus:border-blue-500
        {} {}
    ",
        theme.border_color, theme.bg_color_2
    );
    let name = label.clone();
    rsx! {
        label {
            class: "text-sm",
            r#for: "{id}",
            {label}
        }
        input {
            id,
            class: "p-2.5 {style}",
            name,
            required: true,
            value,
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
        focus:ring-blue-500 focus:border-blue-500
        {} {}
    ",
        theme.border_color, theme.bg_color_2
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
