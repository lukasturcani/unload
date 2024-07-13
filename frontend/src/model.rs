use reqwest::Url;
use serde::{Deserialize, Serialize};
use shared_models::SavedBoard;

#[derive(Debug)]
pub struct UnloadUrl(pub Url);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedBoards(pub Vec<SavedBoard>);
