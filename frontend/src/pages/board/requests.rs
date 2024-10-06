use crate::model::SavedBoards;
use crate::model::UnloadUrl;
use crate::model::Welcome;
use crate::pages::board::model::Board;
use crate::pages::board::model::Tags;
use crate::pages::board::model::TaskData;
use crate::pages::board::model::Tasks;
use crate::pages::board::model::Users;
use dioxus::prelude::*;
use reqwest::Client;
use shared_models::BoardData;
use shared_models::BoardName;
use shared_models::ChatGptRequest;
use shared_models::SavedBoard;
use shared_models::TagData;
use shared_models::TagId;
use shared_models::{TaskEntry, TaskId, TaskStatus, UserData, UserId};
use std::collections::HashMap;

use super::model::ChatGptResponse;
use super::model::NumChatGptCalls;

#[derive(Copy, Clone)]
pub struct BoardSignals {
    pub board: Signal<Board>,
    pub url: Signal<UnloadUrl>,
    pub tasks: Signal<Tasks>,
    pub users: Signal<Users>,
    pub tags: Signal<Tags>,
    pub saved_boards: Signal<SavedBoards>,
    pub num_chat_gpt_calls: Signal<NumChatGptCalls>,
    pub welcome: Signal<Welcome>,
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
            num_chat_gpt_calls: use_context::<Signal<NumChatGptCalls>>(),
            welcome: use_context::<Signal<Welcome>>(),
        }
    }
}

pub async fn board(mut signals: BoardSignals) {
    log::info!("sending board data request");
    if let Ok(board_data) = send_board_data_request(signals).await {
        log::info!("got board data");
        let Ok(mut board) = signals.board.try_write() else {
            return;
        };
        let Ok(mut tasks) = signals.tasks.try_write() else {
            return;
        };
        let Ok(mut users) = signals.users.try_write() else {
            return;
        };
        let Ok(mut tags) = signals.tags.try_write() else {
            return;
        };
        let Ok(mut saved_boards) = signals.saved_boards.try_write() else {
            return;
        };
        let Ok(mut num_chat_gpt_calls) = signals.num_chat_gpt_calls.try_write() else {
            return;
        };
        let Ok(mut welcome) = signals.welcome.try_write() else {
            return;
        };

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
        num_chat_gpt_calls.0 = board_data.num_chat_gpt_calls;
        *welcome = if *welcome == Welcome::Pending && tasks.0.is_empty() {
            Welcome::True
        } else {
            Welcome::False
        };
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
    task_data: shared_models::NewTaskData,
) -> Result<TaskId, anyhow::Error> {
    let url = {
        let url = &url.read().0;
        let board = board.read();
        url.join(&format!("/api/boards/{}/tasks", board.board_name))?
    };
    Ok(Client::new()
        .post(url)
        .json(&task_data)
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

pub async fn send_chat_gpt_prompt(
    board_name: BoardName,
    url: Signal<UnloadUrl>,
    prompt: String,
    language: String,
    mut chat_gpt_response: Signal<Option<ChatGptResponse>>,
    mut num_calls_left: Signal<NumChatGptCalls>,
) {
    chat_gpt_response.set(Some(ChatGptResponse::Waiting));
    match send_chat_gpt_prompt_request(board_name, url, prompt, language).await {
        Ok(shared_models::ChatGptResponse::Suggestions(suggestions)) => {
            chat_gpt_response.set(Some(ChatGptResponse::Suggestions(suggestions)));
            num_calls_left.write().0 -= 1;
        }
        Ok(shared_models::ChatGptResponse::LimitExceeded) => {
            chat_gpt_response.set(Some(ChatGptResponse::LimitExceeded));
        }
        Err(_) => chat_gpt_response.set(Some(ChatGptResponse::Error)),
    }
}

async fn send_chat_gpt_prompt_request(
    board_name: BoardName,
    url: Signal<UnloadUrl>,
    prompt: String,
    language: String,
) -> Result<shared_models::ChatGptResponse, anyhow::Error> {
    let url = &url.read().0;
    let url = url.join("/api/chat-gpt/suggest-tasks")?;
    Ok(Client::new()
        .post(url)
        .json(&ChatGptRequest {
            board_name,
            prompt,
            language,
        })
        .send()
        .await?
        .json::<shared_models::ChatGptResponse>()
        .await?)
}

pub async fn delete_task_tag(signals: BoardSignals, task_id: TaskId, tag_id: TagId) {
    if send_delete_task_tag_request(signals, task_id, tag_id)
        .await
        .is_ok()
    {
        board(signals).await;
    }
}

async fn send_delete_task_tag_request(
    signals: BoardSignals,
    task_id: TaskId,
    tag_id: TagId,
) -> Result<(), anyhow::Error> {
    let url = {
        let url = &signals.url.read().0;
        let board = signals.board.read();
        url.join(&format!(
            "/api/boards/{}/tasks/{}/tags/{}",
            board.board_name, task_id, tag_id
        ))?
    };
    Ok(reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await?)
}
