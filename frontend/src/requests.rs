use crate::model::Board;
use crate::model::Model;
use crate::model::QuickAddTasks;
use crate::model::Tags;
use crate::model::TaskData;
use crate::model::Tasks;
use crate::model::Users;
use dioxus::prelude::*;
use reqwest::Client;
use shared_models::QuickAddData;
use shared_models::QuickAddEntry;
use shared_models::QuickAddTaskId;
use shared_models::TagData;
use shared_models::TagEntry;
use shared_models::TagId;
use shared_models::{TaskEntry, TaskId, TaskStatus, UserData, UserEntry, UserId};
use std::collections::HashMap;
use tokio::join;

#[derive(Copy, Clone)]
pub struct BoardSignals {
    pub model: Signal<Model>,
    pub board: Signal<Board>,
    pub tasks: Signal<Tasks>,
    pub users: Signal<Users>,
    pub tags: Signal<Tags>,
    pub quick_add: Signal<QuickAddTasks>,
}

impl Default for BoardSignals {
    fn default() -> Self {
        Self {
            model: use_context::<Signal<Model>>(),
            board: use_context::<Signal<Board>>(),
            tasks: use_context::<Signal<Tasks>>(),
            users: use_context::<Signal<Users>>(),
            tags: use_context::<Signal<Tags>>(),
            quick_add: use_context::<Signal<QuickAddTasks>>(),
        }
    }
}

pub async fn board(mut signals: BoardSignals) {
    log::info!("sending board data request");
    if let (Ok(new_users), Ok(new_tasks), Ok(new_tags), Ok(new_quick_add)) = join!(
        get_users(signals.board),
        get_tasks(signals.board),
        get_tags(signals.board),
        get_quick_add(signals.board)
    ) {
        log::info!("got board data");
        let mut model = signals.model.write();
        model.users = new_users.clone();
        model.tasks = new_tasks.tasks.clone();
        model.tags = new_tags.clone();
        model.to_do = new_tasks.to_do.clone();
        model.in_progress = new_tasks.in_progress.clone();
        model.done = new_tasks.done.clone();
        model.quick_add = new_quick_add.clone();

        let mut board = signals.board.write();
        let mut tasks = signals.tasks.write();
        let mut users = signals.users.write();
        let mut tags = signals.tags.write();
        let mut quick_add = signals.quick_add.write();

        board.to_do = new_tasks.to_do;
        board.in_progress = new_tasks.in_progress;
        board.done = new_tasks.done;
        users.0 = new_users;
        tasks.0 = new_tasks.tasks;
        tags.0 = new_tags;
        quick_add.0 = new_quick_add;
    } else {
        log::info!("failed to get board data")
    }
}

async fn get_users(board: Signal<Board>) -> Result<HashMap<UserId, UserData>, anyhow::Error> {
    let url = {
        let model = board.read();
        model
            .url
            .join(&format!("/api/boards/{}/users", model.board_name))?
    };
    Ok(Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<UserEntry>>()
        .await?
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
        }))
}

async fn get_tags(board: Signal<Board>) -> Result<HashMap<TagId, TagData>, anyhow::Error> {
    let url = {
        let model = board.read();
        model
            .url
            .join(&format!("/api/boards/{}/tags", model.board_name))?
    };
    Ok(Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<TagEntry>>()
        .await?
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
        }))
}

async fn get_tasks(board: Signal<Board>) -> Result<TasksResponse, anyhow::Error> {
    let url = {
        let board = board.read();
        board
            .url
            .join(&format!("/api/boards/{}/tasks", board.board_name))?
    };
    let client = Client::new();
    Ok(client
        .get(url)
        .send()
        .await?
        .json::<Vec<TaskEntry>>()
        .await?
        .into_iter()
        .fold(TasksResponse::default(), |mut tasks, task| {
            tasks.tasks.insert(
                task.id,
                TaskData {
                    title: task.title,
                    description: task.description,
                    created: task.created,
                    updated: task.updated,
                    due: task.due,
                    size: task.size,
                    assignees: task.assignees,
                    tags: task.tags,
                },
            );
            match task.status {
                TaskStatus::ToDo => tasks.to_do.push(task.id),
                TaskStatus::InProgress => tasks.in_progress.push(task.id),
                TaskStatus::Done => tasks.done.push(task.id),
            }
            tasks
        }))
}

async fn get_quick_add(
    board: Signal<Board>,
) -> Result<HashMap<QuickAddTaskId, QuickAddData>, anyhow::Error> {
    let url = {
        let model = board.read();
        model
            .url
            .join(&format!("/api/boards/{}/quick-add", model.board_name))?
    };
    Ok(Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<QuickAddEntry>>()
        .await?
        .into_iter()
        .fold(HashMap::new(), |mut quick_add, task| {
            quick_add.insert(task.id, task.into());
            quick_add
        }))
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
                    size: task.size,
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
    board: Signal<Board>,
    mut user_data: UserData,
) -> Result<(UserId, String), anyhow::Error> {
    user_data.name = user_data.name.trim().to_string();
    let url = {
        let board = board.read();
        board
            .url
            .join(&format!("/api/boards/{}/users", board.board_name))?
    };
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
    board: Signal<Board>,
    mut tag_data: TagData,
) -> Result<(TagId, String), anyhow::Error> {
    tag_data.name = tag_data.name.trim().to_string();
    let url = {
        let board = board.read();
        board
            .url
            .join(&format!("/api/boards/{}/tags", board.board_name))?
    };
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
    board: Signal<Board>,
    task_data: &shared_models::TaskData,
) -> Result<TaskId, anyhow::Error> {
    let url = {
        let board = board.read();
        board
            .url
            .join(&format!("/api/boards/{}/tasks", board.board_name))?
    };
    Ok(Client::new()
        .post(url)
        .json(task_data)
        .send()
        .await?
        .json::<TaskId>()
        .await?)
}
