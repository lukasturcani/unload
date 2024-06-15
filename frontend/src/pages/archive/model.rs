use shared_models::TagEntry;

pub struct TagsUrl(pub reqwest::Url);

#[derive(Default)]
pub struct TagEntries(pub Vec<TagEntry>);
