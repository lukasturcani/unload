use dioxus::prelude::*;
use shared_models::TaskStatus;

use crate::{commands::FocusTarget, components::icons::CircledPlusIcon, themes::Theme};

#[component]
pub fn AddTaskButton(status: TaskStatus, adding_task: Signal<bool>) -> Element {
    let mut focus_target = use_context::<Signal<FocusTarget>>();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border-t {}", theme.border_color);
    rsx! {
        button {
            class: "
                h-10 sm:h-12 shrink-0 grow-0
                flex flex-row justify-center items-center
                {style}
            ",
            onclick: move |_| {
                if adding_task() {
                    focus_target.set(
                        FocusTarget(Some(format!("new-{status:#?}-task-title-input")))
                    );
                } else {
                    adding_task.set(true);
                }
            },
            div {
                class: "size-6",
                CircledPlusIcon {}
            }
        }
    }
}
