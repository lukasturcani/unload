use dioxus_sdk::i18n::Language;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Translation {
    pub id: &'static str,
    pub name: &'static str,
    to_do_column_title: &'static str,
    in_progress_column_title: &'static str,
    done_column_title: &'static str,
    board_link: &'static str,
    tags_link: &'static str,
    users_link: &'static str,
    archive_link: &'static str,
}

pub fn translations() -> Vec<Translation> {
    let mut translations = vec![
        Translation {
            id: "en",
            name: "EN - English",
            to_do_column_title: "To Do",
            in_progress_column_title: "In Progress",
            done_column_title: "Done",
            board_link: "Board",
            tags_link: "Tags",
            users_link: "Users",
            archive_link: "Archive",
        },
        Translation {
            id: "sk",
            name: "SK - Slovenčina",
            to_do_column_title: "Úlohy",
            in_progress_column_title: "Prebiehajúce",
            done_column_title: "Hotovo",
            board_link: "Tabuľa",
            tags_link: "Značky",
            users_link: "Používatelia",
            archive_link: "Archív",
        },
    ];
    translations.sort_by_key(|t| t.name);
    translations
}

pub fn languages() -> Vec<Language> {
    translations().into_iter().map(Language::from).collect()
}

impl Translation {
    pub fn to_json(&self) -> Value {
        json!({
            "id": self.id,
            "texts": {
                "to_do_column_title": self.to_do_column_title,
                "in_progress_column_title": self.in_progress_column_title,
                "done_column_title": self.done_column_title,
                "board_link": self.board_link,
                "tags_link": self.tags_link,
                "users_link": self.users_link,
                "archive_link": self.archive_link,
            }
        })
    }
}

impl From<Translation> for Language {
    fn from(translation: Translation) -> Self {
        Language::from_str(&translation.to_json().to_string()).unwrap()
    }
}
