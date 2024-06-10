use dioxus::prelude::*;
use shared_models::BoardName;

use crate::components::nav::NavBar;

#[component]
pub fn Archive(board_name: BoardName) -> Element {
    rsx! {
        div {
            class: "
                w-screen h-dvh
                bg-gray-900
                flex flex-col
                text-white stroke-white
            ",
            NavBar { board_name }
        }
    }
}
