use crate::model::Model;
use crate::model::TaskData;
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

pub async fn board(model: Signal<Model>) {
    log::info!("sending board data request");
    if let (Ok(users), Ok(tasks), Ok(tags), Ok(quick_add)) =
        join!(users(model), tasks(model), tags(model), quick_add(model))
    {
        log::info!("got board data");
        let mut model = model.write();
        model.users = users;
        model.tasks = tasks.tasks;
        model.tags = tags;
        model.to_do = tasks.to_do;
        model.in_progress = tasks.in_progress;
        model.done = tasks.done;
        model.quick_add = quick_add;
    } else {
        log::info!("failed to get board data")
    }
}

pub async fn board_tags(model: Signal<Model>) {
    log::info!("sending board tags request");
    if let Ok(tags) = tags(model).await {
        log::info!("got board tags");
        let mut model = model.write();
        model.tags = tags;
    } else {
        log::info!("failed to get board tags")
    }
}

pub async fn board_users(model: Signal<Model>) {
    log::info!("sending board users request");
    if let Ok(users) = users(model).await {
        log::info!("got board users");
        let mut model = model.write();
        model.users = users;
    } else {
        log::info!("failed to get board users")
    }
}

async fn users(model: Signal<Model>) -> Result<HashMap<UserId, UserData>, anyhow::Error> {
    let url = {
        let model = model.read();
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

async fn tags(model: Signal<Model>) -> Result<HashMap<TagId, TagData>, anyhow::Error> {
    let url = {
        let model = model.read();
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

async fn tasks(model: Signal<Model>) -> Result<TasksResponse, anyhow::Error> {
    let url = {
        let model = model.read();
        model
            .url
            .join(&format!("/api/boards/{}/tasks", model.board_name))?
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

async fn quick_add(
    model: Signal<Model>,
) -> Result<HashMap<QuickAddTaskId, QuickAddData>, anyhow::Error> {
    let url = {
        let model = model.read();
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
    model: Signal<Model>,
    mut user_data: UserData,
) -> Result<(UserId, String), anyhow::Error> {
    user_data.name = user_data.name.trim().to_string();
    let url = {
        let model = model.read();
        model
            .url
            .join(&format!("/api/boards/{}/users", model.board_name))?
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
    model: Signal<Model>,
    mut tag_data: TagData,
) -> Result<(TagId, String), anyhow::Error> {
    tag_data.name = tag_data.name.trim().to_string();
    let url = {
        let model = model.read();
        model
            .url
            .join(&format!("/api/boards/{}/tags", model.board_name))?
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
    model: Signal<Model>,
    task_data: &shared_models::TaskData,
) -> Result<TaskId, anyhow::Error> {
    let url = {
        let model = model.read();
        model
            .url
            .join(&format!("/api/boards/{}/tasks", model.board_name))?
    };
    Ok(Client::new()
        .post(url)
        .json(task_data)
        .send()
        .await?
        .json::<TaskId>()
        .await?)
}
