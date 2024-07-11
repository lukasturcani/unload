use crate::model::SavedBoards;
use crate::model::UnloadUrl;
use crate::pages::board::model::Board;
use crate::pages::board::model::Tags;
use crate::pages::board::model::TaskData;
use crate::pages::board::model::Tasks;
use crate::pages::board::model::Users;
use dioxus::prelude::*;
use reqwest::Client;
use shared_models::BoardData;
use shared_models::SavedBoard;
use shared_models::TagData;
use shared_models::TagId;
use shared_models::{TaskEntry, TaskId, TaskStatus, UserData, UserId};
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct BoardSignals {
    pub board: Signal<Board>,
    pub url: Signal<UnloadUrl>,
    pub tasks: Signal<Tasks>,
    pub users: Signal<Users>,
    pub tags: Signal<Tags>,
    pub saved_boards: Signal<SavedBoards>,
}

impl Default for BoardSignals {
    fn default() -> Self {
        Self {
            board: use_context::<Signal<Board>>(),
            url: use_context::<Signal<UnloadUrl>>(),
            tasks: use_context::<Signal<Tasks>>(),
            users: use_context::<Signal<Users>>(),
            tags: use_context::<Signal<Tags>>(),
            saved_boards: use_context::<Signal<SavedBoards>>(),
        }
    }
}

pub async fn board(mut signals: BoardSignals) {
    log::info!("sending board data request");
    if let Ok(board_data) = send_board_data_request(signals).await {
        log::info!("got board data");
        let mut board = signals.board.write();
        let mut tasks = signals.tasks.write();
        let mut users = signals.users.write();
        let mut tags = signals.tags.write();
        let mut saved_boards = signals.saved_boards.write();

        let task_response = TasksResponse::from(board_data.tasks);

        board.title = board_data.title;
        board.to_do = task_response.to_do;
        board.in_progress = task_response.in_progress;
        board.done = task_response.done;
        users.0 = board_data
            .users
            .into_iter()
            .fold(HashMap::new(), |mut users, user| {
                users.insert(
                    user.id,
                    UserData {
                        name: user.name,
                        color: user.color,
                    },
                );
                users
            });
        tasks.0 = task_response.tasks;
        tags.0 = board_data
            .tags
            .into_iter()
            .fold(HashMap::new(), |mut tags, tag| {
                tags.insert(
                    tag.id,
                    TagData {
                        name: tag.name,
                        color: tag.color,
                    },
                );
                tags
            });
        saved_boards.0 = board_data.saved_boards;
        if saved_boards.0.iter().all(|b| b.name != board.board_name) {
            saved_boards.0.push(SavedBoard {
                name: board.board_name.clone(),
                title: board.title.clone(),
            });
        }
    } else {
        log::info!("failed to get board data")
    }
}

async fn send_board_data_request(signals: BoardSignals) -> Result<BoardData, anyhow::Error> {
    let url = {
        let url = &signals.url.read().0;
        let board = signals.board.read();
        url.join(&format!("/api/boards/{}/read", board.board_name))?
    };
    Ok(Client::new()
        .post(url)
        .json(
            &signals
                .saved_boards
                .read()
                .0
                .iter()
                .map(|SavedBoard { name, .. }| name)
                .collect::<Vec<_>>(),
        )
        .send()
        .await?
        .json::<BoardData>()
        .await?)
}

#[derive(Default, Debug)]
struct TasksResponse {
    tasks: HashMap<TaskId, TaskData>,
    to_do: Vec<TaskId>,
    in_progress: Vec<TaskId>,
    done: Vec<TaskId>,
}

impl From<Vec<TaskEntry>> for TasksResponse {
    fn from(value: Vec<TaskEntry>) -> Self {
        let mut to_do = Vec::new();
        let mut in_progress = Vec::new();
        let mut done = Vec::new();
        let mut tasks = HashMap::with_capacity(value.len());
        for task in value {
            tasks.insert(
                task.id,
                TaskData {
                    title: task.title,
                    description: task.description,
                    created: task.created,
                    updated: task.updated,
                    due: task.due,
                    assignees: task.assignees,
                    tags: task.tags,
                },
            );
            match task.status {
                TaskStatus::ToDo => to_do.push(task.id),
                TaskStatus::InProgress => in_progress.push(task.id),
                TaskStatus::Done => done.push(task.id),
            }
        }
        Self {
            tasks,
            to_do,
            in_progress,
            done,
        }
    }
}

pub async fn create_user(
    url: Signal<UnloadUrl>,
    board: Signal<Board>,
    mut user_data: UserData,
) -> Result<(UserId, String), anyhow::Error> {
    let url = {
        let url = &url.read().0;
        let board = board.read();
        url.join(&format!("/api/boards/{}/users", board.board_name))?
    };
    user_data.name = user_data.name.trim().to_string();
    Ok((
        Client::new()
            .post(url)
            .json(&user_data)
            .send()
            .await?
            .json::<UserId>()
            .await?,
        user_data.name,
    ))
}

pub async fn create_tag(
    url: Signal<UnloadUrl>,
    board: Signal<Board>,
    mut tag_data: TagData,
) -> Result<(TagId, String), anyhow::Error> {
    let url = {
        let url = &url.read().0;
        let board = board.read();
        url.join(&format!("/api/boards/{}/tags", board.board_name))?
    };
    tag_data.name = tag_data.name.trim().to_string();
    Ok((
        Client::new()
            .post(url)
            .json(&tag_data)
            .send()
            .await?
            .json::<TagId>()
            .await?,
        tag_data.name,
    ))
}

pub async fn create_task(
    url: Signal<UnloadUrl>,
    board: Signal<Board>,
    task_data: &shared_models::TaskData,
) -> Result<TaskId, anyhow::Error> {
    let url = {
        let url = &url.read().0;
        let board = board.read();
        url.join(&format!("/api/boards/{}/tasks", board.board_name))?
    };
    Ok(Client::new()
        .post(url)
        .json(task_data)
        .send()
        .await?
        .json::<TaskId>()
        .await?)
}

pub async fn set_board_title(signals: BoardSignals, title: String) {
    if send_set_board_title_request(signals, title).await.is_ok() {
        board(signals).await;
    }
}

async fn send_set_board_title_request(
    signals: BoardSignals,
    title: String,
) -> Result<(), anyhow::Error> {
    let url = {
        let url = &signals.url.read().0;
        let board = signals.board.read();
        url.join(&format!("/api/boards/{}/title", board.board_name))?
    };
    Ok(Client::new()
        .put(url)
        .json(&title)
        .send()
        .await?
        .json::<()>()
        .await?)
}
