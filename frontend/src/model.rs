use reqwest::Url;
use serde::{Deserialize, Serialize};
use shared_models::BoardName;

#[derive(Debug)]
pub struct UnloadUrl(pub Url);

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SavedBoard {
    pub name: BoardName,
    pub title: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedBoards(pub Vec<SavedBoard>);
