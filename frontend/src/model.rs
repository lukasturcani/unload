use std::{collections::HashMap, str::FromStr};

use chrono::{DateTime, Utc};
use reqwest::Url;
use shared_models::{BoardName, TaskEntry, TaskId, TaskSize, UserData, UserId};

pub struct Model {
    pub url: Url,
    pub board_name: BoardName,
    pub tasks: HashMap<TaskId, TaskData>,
    pub users: HashMap<UserId, UserData>,
    pub to_do: Vec<TaskId>,
    pub in_progress: Vec<TaskId>,
    pub done: Vec<TaskId>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            url: Url::from_str("http://localhost:8080").unwrap(),
            board_name: BoardName::from(""),
            tasks: HashMap::default(),
            users: HashMap::default(),
            to_do: Vec::default(),
            in_progress: Vec::default(),
            done: Vec::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TaskData {
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub due: Option<DateTime<Utc>>,
    pub size: TaskSize,
    pub assignees: Vec<UserId>,
    pub blocks: Vec<TaskId>,
    pub blocked_by: Vec<TaskId>,
}

impl From<TaskEntry> for TaskData {
    fn from(value: TaskEntry) -> Self {
        Self {
            title: value.title,
            description: value.description,
            created: value.created,
            updated: value.updated,
            due: value.due,
            size: value.size,
            assignees: value.assignees,
            blocks: value.blocks,
            blocked_by: value.blocked_by,
        }
    }
}
