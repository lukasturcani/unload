use crate::pages::{
    Archive, Board, JoinBoard, LanguageBoard, LanguageTags, LanguageUsers, Tags, Users,
};
use dioxus::prelude::*;
use shared_models::BoardName;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/:language")]
    JoinBoard { language: String },
    #[route("/boards/:board_name")]
    Board { board_name: BoardName },
    #[route("/:language/boards/:board_name")]
    LanguageBoard {
        language: String,
        board_name: BoardName,
    },
    #[route("/boards/:board_name/users")]
    Users { board_name: BoardName },
    #[route("/:language/boards/:board_name/users")]
    LanguageUsers {
        language: String,
        board_name: BoardName,
    },
    #[route("/boards/:board_name/tags")]
    Tags { board_name: BoardName },
    #[route("/:language/boards/:board_name/tags")]
    LanguageTags {
        language: String,
        board_name: BoardName,
    },
    #[route("/boards/:board_name/archive")]
    Archive { board_name: BoardName },
}
