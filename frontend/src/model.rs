use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

use chrono::{DateTime, Utc};
use reqwest::Url;
use shared_models::{BoardName, TaskEntry, TaskId, TaskSize, UserData, UserId};

pub struct Model {
    pub url: Url,
    pub board_name: BoardName,
    pub tasks: Tasks,
    pub users: HashMap<UserId, UserData>,
    pub to_do: Vec<TaskId>,
    pub in_progress: Vec<TaskId>,
    pub done: Vec<TaskId>,
}

impl Model {
    pub fn most_recent_titles(&self) -> Vec<(TaskId, String)> {
        let tasks = self.tasks.tasks();
        let mut titles = Vec::with_capacity(self.tasks.most_recently_updated().len());
        for (_, task_id) in self.tasks.most_recently_updated() {
            titles.push((*task_id, tasks[task_id].title.clone()))
        }
        titles
    }

    pub fn find_titles(&self, search_input: &str) -> Vec<(TaskId, String)> {
        self.tasks
            .tasks()
            .iter()
            .filter(|(task_id, task)| {
                task.title.find(search_input).is_some()
                    || task.description.find(search_input).is_some()
            })
            .map(|(task_id, task)| (*task_id, task.title.clone()))
            .collect()
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            url: Url::from_str("http://localhost:8080").unwrap(),
            board_name: BoardName::from(""),
            tasks: Tasks::default(),
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

const NUM_MOST_RECENTLY_UPDATED: usize = 5;

#[derive(Default, Debug)]
pub struct Tasks {
    tasks: HashMap<TaskId, TaskData>,
    most_recently_updated: BinaryHeap<(i64, TaskId)>,
}

impl Tasks {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            tasks: HashMap::with_capacity(capacity),
            most_recently_updated: BinaryHeap::with_capacity(NUM_MOST_RECENTLY_UPDATED),
        }
    }

    pub fn get(&self, task_id: &TaskId) -> &TaskData {
        &self.tasks[task_id]
    }

    pub fn insert(&mut self, task_id: TaskId, task_data: TaskData) -> Option<TaskData> {
        self.most_recently_updated
            .retain(|(task_id, _)| task_id != task_id);
        let timestamp = task_data.updated.timestamp();
        if self.most_recently_updated.len() < NUM_MOST_RECENTLY_UPDATED
            || self
                .most_recently_updated
                .peek()
                .map_or(false, |value| timestamp < value.0)
        {
            self.most_recently_updated.pop();
            self.most_recently_updated.push((timestamp, task_id));
        }
        self.tasks.insert(task_id, task_data)
    }

    pub fn tasks(&self) -> &HashMap<TaskId, TaskData> {
        &self.tasks
    }

    pub fn most_recently_updated(&self) -> &BinaryHeap<(i64, TaskId)> {
        &self.most_recently_updated
    }
}
