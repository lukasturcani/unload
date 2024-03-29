use crate::board::Board;
use crate::join_board::JoinBoard;
use dioxus::prelude::*;
use shared_models::BoardName;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    JoinBoard {},
    #[route("/boards/:board_name")]
    Board { board_name: BoardName },
}
