use shared_models::{TagEntry, TaskEntry};

#[derive(Default)]
pub struct TagEntries(pub Vec<TagEntry>);

#[derive(Default)]
pub struct TaskEntries(pub Vec<TaskEntry>);
