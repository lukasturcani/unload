use crate::board::Board;
use crate::join_board::JoinBoard;
use crate::model::Model;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use shared_models::BoardName;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    JoinBoard {},
    #[route("/boards/:board_name")]
    Board { board_name: BoardName },
}

#[component]
pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, Model::default);
    cx.render(rsx! { Router::<Route>{} })
}
