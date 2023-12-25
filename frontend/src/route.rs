use crate::add_task::AddTask;
use crate::add_user::AddUser;
use crate::board::Board;
use crate::join_board::JoinBoard;
use crate::users::Users;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use shared_models::BoardName;

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    JoinBoard {},
    #[route("/boards/:board_name")]
    Board { board_name: BoardName },
    #[route("/boards/:board_name/add-user")]
    AddUser { board_name: BoardName },
    #[route("/boards/:board_name/users")]
    Users { board_name: BoardName },
    #[route("/boards/:board_name/add-task")]
    AddTask { board_name: BoardName },
}
