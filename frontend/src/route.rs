use crate::add_task::{AddDoneTask, AddInProgressTask, AddTask, AddToDoTask};
use crate::add_user::AddUser;
use crate::board::Board;
use crate::edit_task::EditTask;
use crate::join_board::JoinBoard;
use crate::tags::Tags;
use crate::users::Users;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use shared_models::{BoardName, TaskId};

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
    #[route("/boards/:board_name/tags")]
    Tags { board_name: BoardName },
    #[route("/boards/:board_name/add-task")]
    AddTask { board_name: BoardName },
    #[route("/boards/:board_name/add-to-do-task")]
    AddToDoTask { board_name: BoardName },
    #[route("/boards/:board_name/add-in-progress-task")]
    AddInProgressTask { board_name: BoardName },
    #[route("/boards/:board_name/add-done-task")]
    AddDoneTask { board_name: BoardName },
    #[route("/boards/:board_name/tasks/:task_id/edit")]
    EditTask {
        board_name: BoardName,
        task_id: TaskId,
    },
}
