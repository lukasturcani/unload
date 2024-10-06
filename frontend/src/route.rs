use crate::pages::{Archive, Board, JoinBoard, LanguageBoard, LanguageJoinBoard, Tags, Users};
use dioxus::prelude::*;
use shared_models::BoardName;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    JoinBoard {},
    #[route("/:language")]
    LanguageJoinBoard { language: String },
    #[route("/boards/:board_name")]
    Board { board_name: BoardName },
    #[route("/:language/boards/:board_name")]
    LanguageBoard {
        language: String,
        board_name: BoardName,
    },
    #[route("/boards/:board_name/users")]
    Users { board_name: BoardName },
    #[route("/boards/:board_name/tags")]
    Tags { board_name: BoardName },
    #[route("/boards/:board_name/archive")]
    Archive { board_name: BoardName },
}
