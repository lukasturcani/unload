use dioxus::prelude::*;

use crate::components::icons::{CancelIcon, ConfirmIcon};

#[component]
pub fn ConfirmButton(label: String) -> Element {
    let style = "
        rounded-md
        border border-green-500
        stroke-green-500
        active:bg-green-500
        sm:hover:bg-green-500 sm:hover:stroke-white
    ";
    rsx! {
        button {
            "aria-label": label,
            class: "size-7 {style}",
            r#type: "submit",
            ConfirmIcon {}
        }
    }
}

#[component]
pub fn CancelButton(label: String, editing: Signal<bool>) -> Element {
    let style = "
        rounded-md
        border border-red-500
        stroke-red-500
        active:bg-red-500
        sm:hover:bg-red-500 sm:hover:stroke-white
    ";
    rsx! {
        button {
            "aria-label": label,
            class: "size-7 {style}",
            onclick: move |_| {
                editing.set(false);
            },
            CancelIcon {}
        }
    }
}
