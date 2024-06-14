use dioxus::prelude::*;
use shared_models::BoardName;

mod components;
mod model;

#[component]
pub fn Users(board_name: BoardName) -> Element {
    rsx! {
        div {
            "Users"
        }
    }
}
