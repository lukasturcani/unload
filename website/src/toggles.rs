use dioxus::prelude::*;

#[component]
pub fn Toggle(
    label: ReadOnlySignal<String>,
    on_change: EventHandler<FormEvent>,
    checked: Option<bool>,
) -> Element {
    rsx! {
        label {
            class: "inline-flex items-center cursor-pointer",
            input {
                r#type: "checkbox",
                value: label,
                class: "sr-only peer",
                onchange: move |event| on_change.call(event),
                checked: checked.unwrap_or(false),
            }
            div {
                class: "relative w-11 h-6 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-focus rounded-full \
                    peer bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white \
                    after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full \
                    after:h-5 after:w-5 after:transition-all border-gray-600 peer-checked:bg-blue-600",
            }
            span {
                class: "ms-3 text-sm font-medium text-text-secondary",
                {label}
            }
        }
    }
}
