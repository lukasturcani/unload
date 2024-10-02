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
    open_main_menu: &'static str,
    home_section_label: &'static str,
    features_section_label: &'static str,
    pricing_section_label: &'static str,
    contact_section_label: &'static str,
    select_language: &'static str,
    go_to_app: &'static str,
    app_link: &'static str,
    home_link: &'static str,
    new_board_link: &'static str,
    new_board: &'static str,
    h1_main: &'static str,
    h1_sub: &'static str,
    dense_button_label: &'static str,
    dark_button_label: &'static str,
    mobile_button_label: &'static str,
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
                open_main_menu: "Open main menu",
                home_section_label: "Home",
                features_section_label: "Features",
                pricing_section_label: "Pricing",
                contact_section_label: "Contact",
                select_language: "Select language",
                go_to_app: "Go to app",
                app_link: "/en/app",
                home_link: "/en",
                new_board_link: "/en/new-board",
                new_board: "New board",
                h1_main: "Shared task management.",
                h1_sub: "Simple, with no sign ups.",
                dense_button_label: "Dense",
                dark_button_label: "Dark",
                mobile_button_label: "Mobile",
            },
        },
        Translation {
            id: "sk",
            name: "SK - Slovenčina",
            text: Text {
                open_main_menu: "Otvoriť hlavné menu",
                home_section_label: "Domov",
                features_section_label: "Funkcie",
                pricing_section_label: "Ceny",
                contact_section_label: "Kontakt",
                select_language: "Vyberte jazyk",
                go_to_app: "Prejsť na aplikáciu",
                app_link: "/sk/app",
                home_link: "/sk",
                new_board_link: "/sk/new-board",
                new_board: "Nová nástenka",
                h1_main: "Správa úloh.",
                h1_sub: "Jednoducho, bez registrácie.",
                dense_button_label: "Husté",
                dark_button_label: "Tmavé",
                mobile_button_label: "Mobil",
            },
        },
        Translation {
            id: "ko",
            name: "KO - 한국어",
            text: Text {
                open_main_menu: "메인 메뉴 열기",
                home_section_label: "홈",
                features_section_label: "기능",
                pricing_section_label: "요금",
                contact_section_label: "연락처",
                select_language: "언어 선택",
                go_to_app: "앱으로 이동",
                app_link: "/ko/app",
                home_link: "/ko",
                new_board_link: "/ko/new-board",
                new_board: "새 보드",
                h1_main: "동기 작업 관리.",
                h1_sub: "등록 없이 간단하고 쉽게.",
                dense_button_label: "밀집",
                dark_button_label: "어두운",
                mobile_button_label: "모바일",
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
