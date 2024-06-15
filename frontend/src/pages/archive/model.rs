use std::collections::HashMap;

use shared_models::{TagData, TagEntry, TagId, TaskEntry, UserData, UserId};

#[derive(Default)]
pub struct Users(pub HashMap<UserId, UserData>);

#[derive(Default)]
pub struct Tags(pub HashMap<TagId, TagData>);

#[derive(Default)]
pub struct TagEntries(pub Vec<TagEntry>);

#[derive(Default)]
pub struct TaskEntries(pub Vec<TaskEntry>);
