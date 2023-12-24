use crate::board::Board;
use crate::join_board::JoinBoard;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use shared_models::BoardName;

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    JoinBoard {},
    #[route("/boards/:board_name")]
    Board { board_name: BoardName },
}
