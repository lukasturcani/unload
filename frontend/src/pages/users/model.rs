use shared_models::UserEntry;

pub struct UsersUrl(pub reqwest::Url);

#[derive(Default)]
pub struct UserEntries(pub Vec<UserEntry>);
