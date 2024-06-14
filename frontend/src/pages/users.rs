use dioxus::prelude::*;
use shared_models::BoardName;

#[component]
pub fn Users(board_name: BoardName) -> Element {
    rsx! {
        div {
            "Users"
        }
    }
}
