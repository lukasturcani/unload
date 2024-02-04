use dioxus::prelude::*;
use shared_models::BoardName;

#[component]
pub fn RepeatingTasks(cx: Scope, board_name: BoardName) -> Element {
    cx.render(rsx! {
        div {}
    })
}
