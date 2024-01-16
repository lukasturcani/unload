use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct BoardName(String);

impl From<&str> for BoardName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for BoardName {
    fn from(value: String) -> Self {
        Self(value)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub enum TaskSize {
    Small,
    Medium,
    Large,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
    pub size: TaskSize,
    pub status: TaskStatus,
    pub assignees: Vec<UserId>,
    pub blocks: Vec<TaskId>,
    pub blocked_by: Vec<TaskId>,
    pub tags: Vec<TagId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEntry {
    pub id: TaskId,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub due: Option<DateTime<Utc>>,
    pub size: TaskSize,
    pub status: TaskStatus,
    pub assignees: Vec<UserId>,
    pub blocks: Vec<TaskId>,
    pub blocked_by: Vec<TaskId>,
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
