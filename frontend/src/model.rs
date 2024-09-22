use reqwest::Url;
use serde::{Deserialize, Serialize};
use shared_models::SavedBoard;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct BoardLanguage(pub String);

#[derive(Debug)]
pub struct UnloadUrl(pub Url);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedBoards(pub Vec<SavedBoard>);

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub enum Welcome {
    #[default]
    Pending,
    True,
    False,
}
