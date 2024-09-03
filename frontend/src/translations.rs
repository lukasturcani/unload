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
    to_do_column_title: &'static str,
    in_progress_column_title: &'static str,
    done_column_title: &'static str,
    pick_language_tooltip: &'static str,
    toggle_show_themes_tooltip: &'static str,
    toggle_dense_view_tooltip: &'static str,
    edit_board_title_tooltip: &'static str,
    board_title_input_label: &'static str,
    board_title_update_label: &'static str,
    set_board_title_label: &'static str,
    cancel_board_title_update_label: &'static str,
    task_title_input_label: &'static str,
    edit_task_title_tooltip: &'static str,
    task_title_update_label: &'static str,
    set_task_title_label: &'static str,
    cancel_task_title_update_label: &'static str,
    set_task_status_section_label: &'static str,
    to_do_button_tooltip: &'static str,
    in_progress_button_tooltip: &'static str,
    done_button_tooltip: &'static str,
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
            text: Text {
                to_do_column_title: "To Do",
                in_progress_column_title: "In Progress",
                done_column_title: "Done",
                pick_language_tooltip: "Pick Language",
                toggle_show_themes_tooltip: "Toggle Show Themes",
                toggle_dense_view_tooltip: "Toggle Dense View",
                edit_board_title_tooltip: "Edit Title",
                board_title_input_label: "Title",
                board_title_update_label: "Update Board Title",
                set_board_title_label: "Set Title",
                cancel_board_title_update_label: "Cancel Title Update",
                task_title_input_label: "Title",
                edit_task_title_tooltip: "Edit Title",
                task_title_update_label: "Update Task Title",
                set_task_title_label: "Set Title",
                cancel_task_title_update_label: "Cancel Title Update",
                set_task_status_section_label: "Set Task Status",
                to_do_button_tooltip: "To Do",
                in_progress_button_tooltip: "In Progress",
                done_button_tooltip: "Done",
                board_link: "Board",
                tags_link: "Tags",
                users_link: "Users",
                archive_link: "Archive",
            },
        },
        Translation {
            id: "sk",
            name: "SK - Slovenčina",
            text: Text {
                to_do_column_title: "Úlohy",
                in_progress_column_title: "Prebiehajúce",
                done_column_title: "Hotovo",
                pick_language_tooltip: "Vyberte jazyk",
                toggle_show_themes_tooltip: "Prepnúť zobrazenie tém",
                toggle_dense_view_tooltip: "Prepnúť zobrazenie rozloženia",
                edit_board_title_tooltip: "Zmeniť názov",
                board_title_input_label: "Názov",
                board_title_update_label: "Zmeniť názov tabuľky",
                set_board_title_label: "Zmeniť názov",
                cancel_board_title_update_label: "Zrušiť zmenu názvu",
                task_title_input_label: "Názov",
                edit_task_title_tooltip: "Zmeniť názov",
                task_title_update_label: "Zmeniť názov úlohy",
                set_task_title_label: "Zmeniť názov",
                cancel_task_title_update_label: "Zrušiť zmenu názvu",
                set_task_status_section_label: "Zmeniť stav úlohy",
                to_do_button_tooltip: "Úloha",
                in_progress_button_tooltip: "Prebiehajúca",
                done_button_tooltip: "Hotová",
                board_link: "Tabuľa",
                tags_link: "Značky",
                users_link: "Používatelia",
                archive_link: "Archív",
            },
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
            "texts": serde_json::to_value(&self.text).unwrap(),
        })
    }
}

impl From<Translation> for Language {
    fn from(translation: Translation) -> Self {
        Language::from_str(&translation.to_json().to_string()).unwrap()
    }
}