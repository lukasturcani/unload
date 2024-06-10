use crate::components::archive::Archive;
use crate::components::board::Board;
use crate::join_board::JoinBoard;
use crate::tags::Tags;
use crate::users::Users;
use dioxus::prelude::*;
use shared_models::BoardName;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    JoinBoard {},
    #[route("/boards/:board_name")]
    Board { board_name: BoardName },
    #[route("/boards/:board_name/users")]
    Users { board_name: BoardName },
    #[route("/boards/:board_name/tags")]
    Tags { board_name: BoardName },
    #[route("/boards/:board_name/archive")]
    Archive { board_name: BoardName },
}
