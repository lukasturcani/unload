use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use chrono::{DateTime, Utc};
use reqwest::Url;
use shared_models::{
    BoardName, QuickAddData, TagData, TagId, TaskEntry, TaskId, TaskSize, UserData, UserId,
};

pub struct Model {
    pub url: Url,
    pub board_name: BoardName,
    pub tasks: HashMap<TaskId, TaskData>,
    pub users: HashMap<UserId, UserData>,
    pub tags: HashMap<TagId, TagData>,
    pub to_do: Vec<TaskId>,
    pub in_progress: Vec<TaskId>,
    pub done: Vec<TaskId>,
    pub user_search_created_user: Option<(UserId, String)>,
    pub tag_search_created_tag: Option<(TagId, String)>,
    pub tag_filter: HashSet<TagId>,
    pub size_filter: Option<TaskSize>,
    pub user_filter: HashSet<UserId>,
    pub dense_view: bool,
    pub quick_add: Vec<QuickAddData>,
}

impl Model {
    pub fn show_task(&self, task_id: TaskId) -> bool {
        let task = &self.tasks[&task_id];
        if self.size_filter.map_or(false, |filter| filter != task.size) {
            return false;
        }
        if self
            .user_filter
            .iter()
            .any(|user_id| !task.assignees.contains(user_id))
        {
            return false;
        }
        if self
            .tag_filter
            .iter()
            .any(|tag_id| !task.tags.contains(tag_id))
        {
            return false;
        }
        true
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            #[cfg(debug_assertions)]
            url: Url::from_str("http://localhost:8080").unwrap(),
            #[cfg(not(debug_assertions))]
            url: Url::from_str("https://unload.fly.dev").unwrap(),
            board_name: BoardName::from(""),
            tasks: HashMap::default(),
            users: HashMap::default(),
            tags: HashMap::default(),
            to_do: Vec::default(),
            in_progress: Vec::default(),
            done: Vec::default(),
            user_search_created_user: Option::default(),
            tag_search_created_tag: Option::default(),
            tag_filter: HashSet::default(),
            size_filter: None,
            user_filter: HashSet::default(),
            dense_view: false,
            quick_add: Vec::default(),
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
    pub tags: Vec<TagId>,
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
            tags: value.tags,
        }
    }
}

impl From<TaskData> for QuickAddData {
    fn from(value: TaskData) -> Self {
        Self {
            title: value.title,
            description: value.description,
            size: value.size,
            assignees: value.assignees,
            tags: value.tags,
        }
    }
}

impl From<&TaskData> for QuickAddData {
    fn from(value: &TaskData) -> Self {
        Self {
            title: value.title.clone(),
            description: value.description.clone(),
            size: value.size,
            assignees: value.assignees.clone(),
            tags: value.tags.clone(),
        }
    }
}
