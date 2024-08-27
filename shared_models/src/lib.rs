use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct BoardName(String);

impl From<&str> for BoardName {
    fn from(value: &str) -> Self {
        Self(value.to_lowercase())
    }
}

impl From<String> for BoardName {
    fn from(value: String) -> Self {
        Self(value.to_lowercase())
    }
}

impl Display for BoardName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for BoardName {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SavedBoard {
    pub name: BoardName,
    pub title: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BoardData {
    pub title: String,
    pub users: Vec<UserEntry>,
    pub tasks: Vec<TaskEntry>,
    pub tags: Vec<TagEntry>,
    pub saved_boards: Vec<SavedBoard>,
    pub num_chat_gpt_calls: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct TaskId(i64);

impl From<i64> for TaskId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TaskId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<i64>()?.into())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct QuickAddTaskId(i64);

impl From<i64> for QuickAddTaskId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Display for QuickAddTaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for QuickAddTaskId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<i64>()?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub enum TaskStatus {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct UserId(i64);

impl From<i64> for UserId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Hash)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct TagId(i64);

impl From<i64> for TagId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Display for TagId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TagData {
    pub name: String,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TagEntry {
    pub id: TagId,
    pub name: String,
    pub color: Color,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TaskData {
    pub title: String,
    pub description: String,
    pub due: Option<DateTime<Utc>>,
    pub status: TaskStatus,
    pub assignees: Vec<UserId>,
    pub tags: Vec<TagId>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NewTaskData {
    pub title: String,
    pub description: String,
    pub due: Option<DateTime<Utc>>,
    pub status: TaskStatus,
    pub assignees: Vec<UserId>,
    pub tags: Vec<TagId>,
    pub new_tags: Vec<TagData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEntry {
    pub id: TaskId,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub due: Option<DateTime<Utc>>,
    pub status: TaskStatus,
    pub assignees: Vec<UserId>,
    pub tags: Vec<TagId>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct UserData {
    pub name: String,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserEntry {
    pub id: UserId,
    pub name: String,
    pub color: Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub enum Color {
    Black,
    White,
    Gray,
    Silver,
    Maroon,
    Red,
    Purple,
    Fushsia,
    Green,
    Lime,
    Olive,
    Yellow,
    Navy,
    Blue,
    Teal,
    Aqua,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct QuickAddData {
    pub title: String,
    pub description: String,
    pub assignees: Vec<UserId>,
    pub tags: Vec<TagId>,
}

impl From<QuickAddEntry> for QuickAddData {
    fn from(value: QuickAddEntry) -> Self {
        Self {
            title: value.title,
            description: value.description,
            assignees: value.assignees,
            tags: value.tags,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct QuickAddEntry {
    pub id: QuickAddTaskId,
    pub title: String,
    pub description: String,
    pub assignees: Vec<UserId>,
    pub tags: Vec<TagId>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TaskSuggestion {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ChatGptResponse {
    Suggestions(Vec<TaskSuggestion>),
    LimitExceeded,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Language {
    English,
    Slovak,
}

impl Language {
    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Slovak => "Slovak",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChatGptRequest {
    pub board_name: BoardName,
    pub language: Language,
    pub prompt: String,
}
