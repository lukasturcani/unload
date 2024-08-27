use dioxus::prelude::*;

use crate::themes::Theme;

#[component]
pub fn TextInput(
    id: String,
    label: &'static str,
    value: Option<ReadOnlySignal<String>>,
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
    rsx! {
        label {
            class: "text-sm",
            r#for: "{id}",
            {label}
        }
        input {
            id: id.clone(),
            class: "p-2.5 {style}",
            name: label,
            required: true,
            value,
            onmounted: move |event| async move {
                let _ = event.set_focus(true).await;
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
