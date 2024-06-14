use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use shared_models::{
    BoardName, QuickAddData, QuickAddTaskId, TagData, TagId, TaskEntry, TaskId, TaskSize, UserData,
    UserId,
};

#[derive(Debug)]
pub struct Board {
    pub board_name: BoardName,
    pub to_do: Vec<TaskId>,
    pub in_progress: Vec<TaskId>,
    pub done: Vec<TaskId>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board_name: BoardName::from(""),
            to_do: Vec::default(),
            in_progress: Vec::default(),
            done: Vec::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Tasks(pub HashMap<TaskId, TaskData>);
#[derive(Debug, Default)]
pub struct Users(pub HashMap<UserId, UserData>);
#[derive(Debug, Default)]
pub struct Tags(pub HashMap<TagId, TagData>);
#[derive(Debug, Default)]
pub struct QuickAddTasks(pub HashMap<QuickAddTaskId, QuickAddData>);
#[derive(Debug, Default)]
pub struct UserFilter(pub HashSet<UserId>);
#[derive(Debug, Default)]
pub struct TagFilter(pub HashSet<TagId>);

#[derive(Clone, Debug, PartialEq, Eq)]
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

pub fn task_filter(
    task_id: &TaskId,
    tasks: &HashMap<TaskId, TaskData>,
    user_filter: &HashSet<UserId>,
    tag_filter: &HashSet<TagId>,
) -> bool {
    let task = &tasks[task_id];
    if user_filter
        .iter()
        .any(|user_id| !task.assignees.contains(user_id))
    {
        return false;
    }
    if tag_filter.iter().any(|tag_id| !task.tags.contains(tag_id)) {
        return false;
    }
    true
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
