use dioxus_sdk::i18n::Language;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Translation {
    pub id: &'static str,
    pub name: &'static str,
    text: Text,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Text {
    go_to_app: &'static str,
    app_link: &'static str,
    home_link: &'static str,
}

pub fn languages() -> Vec<Language> {
    translations().into_iter().map(Language::from).collect()
}

pub fn translations() -> Vec<Translation> {
    let mut translations = vec![
        Translation {
            id: "en",
            name: "EN - English",
            text: Text {
                go_to_app: "Go to app",
                app_link: "/app/en",
                home_link: "/en",
            },
        },
        Translation {
            id: "sk",
            name: "SK - Slovenčina",
            text: Text {
                go_to_app: "Prejsť na aplikáciu",
                app_link: "/app/sk",
                home_link: "/sk",
            },
        },
        Translation {
            id: "ko",
            name: "KO - 한국어",
            text: Text {
                go_to_app: "앱으로 이동",
                app_link: "/app/ko",
                home_link: "/ko",
            },
        },
    ];
    translations.sort_by_key(|t| t.name);
    translations
}

impl Translation {
    pub fn to_json(&self) -> Value {
        json!({
            "id": self.id,
            "texts": serde_json::to_value(&self.text).unwrap(),
        })
    }
}

impl From<Translation> for Language {
    fn from(translation: Translation) -> Self {
        Language::from_str(&translation.to_json().to_string()).unwrap()
    }
}
