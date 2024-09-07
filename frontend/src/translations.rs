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
    board_title_update_form_label: &'static str,
    set_board_title_button_label: &'static str,
    cancel_board_title_update_button_label: &'static str,
    task_title_input_label: &'static str,
    edit_task_title_tooltip: &'static str,
    task_title_update_form_label: &'static str,
    set_task_title_button_label: &'static str,
    cancel_task_title_update_button_label: &'static str,
    set_task_status_section_label: &'static str,
    to_do_button_tooltip: &'static str,
    in_progress_button_tooltip: &'static str,
    done_button_tooltip: &'static str,
    task_actions_section_label: &'static str,
    duplicate_task_button_tooltip: &'static str,
    archive_task_button_tooltip: &'static str,
    assignees_section_label: &'static str,
    assign_user_toggle_button_tooltip: &'static str,
    toggle_user_filter_button_label: &'static str,
    assignee_selection_section_label: &'static str,
    add_user_button_label: &'static str,
    add_user_form_label: &'static str,
    user_name_input_label: &'static str,
    cancel_adding_new_user_button_label: &'static str,
    remove_user_from_task_button_label: &'static str,
    tags_section_label: &'static str,
    tag_selection_section_label: &'static str,
    add_tag_button_label: &'static str,
    add_tag_form_label: &'static str,
    tag_name_input_label: &'static str,
    add_tag_toggle_button_tooltip: &'static str,
    cancel_adding_new_tag_button_label: &'static str,
    toggle_tag_filter_button_label: &'static str,
    remove_tag_from_task_button_label: &'static str,
    toggle_expand_task_button_label: &'static str,
    due_date_section_label: &'static str,
    edit_due_date_tooltip: &'static str,
    due_date_form_label: &'static str,
    due_date_input_label: &'static str,
    set_due_date_button_label: &'static str,
    cancel_due_date_update_button_label: &'static str,
    color_picker_legend_label: &'static str,
    description_update_form_label: &'static str,
    set_description_button_label: &'static str,
    cancel_description_update_button_label: &'static str,
    bullet_points_button_tooltip: &'static str,
    task_list_button_tooltip: &'static str,
    description_text_area_label: &'static str,
    description_section_label: &'static str,
    edit_description_tooltip: &'static str,
    additional_actions_section_label: &'static str,
    delete_task_tooltip: &'static str,
    edit_tag_color_form_label: &'static str,
    edit_tag_color_button_label: &'static str,
    set_tag_color_button_label: &'static str,
    cancel_tag_color_update_label: &'static str,
    edit_tag_name_button_label: &'static str,
    edit_tag_name_form_label: &'static str,
    set_tag_name_button_label: &'static str,
    cancel_tag_name_update_button_label: &'static str,
    delete_tag_button_label: &'static str,
    archive_tag_button_label: &'static str,
    unarchive_tag_button_label: &'static str,
    edit_user_color_form_label: &'static str,
    set_user_color_button_label: &'static str,
    cancel_user_color_update_button_label: &'static str,
    edit_user_color_button_label: &'static str,
    edit_user_name_form_label: &'static str,
    set_user_name_button_label: &'static str,
    cancel_user_name_update_button_label: &'static str,
    edit_user_name_button_label: &'static str,
    delete_user_button_label: &'static str,
    task_status_section_label: &'static str,
    filters_section_label: &'static str,
    languages_section_title: &'static str,
    custom_task_button_label: &'static str,
    board_list_section_label: &'static str,
    join_board_button_label: &'static str,
    or_label: &'static str,
    chat_gpt_limit_exceeded_title: &'static str,
    chat_gpt_limit_exceeded_content: &'static str,
    chat_gpt_waiting_message: &'static str,
    chat_gpt_error_title: &'static str,
    chat_gpt_error_content: &'static str,
    chat_gpt_prompt_input_title: &'static str,
    chat_gpt_daily_attempts_left: &'static str,
    chat_gpt_prompt_input_content: &'static str,
    chat_gpt_prompt_input_form_label: &'static str,
    chat_gpt_prompt_input_label: &'static str,
    suggest_cupcake_recipe_prompt: &'static str,
    paint_bedroom_prompt: &'static str,
    friends_over_for_bbq_prompt: &'static str,
    prepare_for_rome_vacation_prompt: &'static str,
    house_tidy_prompt: &'static str,
    fix_fence_prompt: &'static str,
    join_board_form_label: &'static str,
    join_board_input_label: &'static str,
    cancel_joining_board_button_label: &'static str,
    add_task_button_label: &'static str,
    delete_task_button_label: &'static str,
    remove_board_button_label: &'static str,
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
                board_title_update_form_label: "Update Board Title",
                set_board_title_button_label: "Set Title",
                cancel_board_title_update_button_label: "Cancel Title Update",
                task_title_input_label: "Title",
                edit_task_title_tooltip: "Edit Title",
                task_title_update_form_label: "Update Task Title",
                set_task_title_button_label: "Set Title",
                cancel_task_title_update_button_label: "Cancel Title Update",
                set_task_status_section_label: "Set Task Status",
                to_do_button_tooltip: "To Do",
                in_progress_button_tooltip: "In Progress",
                done_button_tooltip: "Done",
                task_actions_section_label: "Task Actions",
                duplicate_task_button_tooltip: "Duplicate Task",
                archive_task_button_tooltip: "Archive Task",
                assignees_section_label: "Assignees",
                assign_user_toggle_button_tooltip: "Assign User",
                toggle_user_filter_button_label: "toggle user filter",
                assignee_selection_section_label: "assignee selection",
                add_user_button_label: "Add User",
                add_user_form_label: "Add User",
                cancel_adding_new_user_button_label: "cancel adding user",
                user_name_input_label: "Name",
                remove_user_from_task_button_label: "remove user from task",
                tags_section_label: "Tags",
                tag_selection_section_label: "Tag Selection",
                add_tag_button_label: "Add Tag",
                add_tag_form_label: "Add Tag",
                tag_name_input_label: "Name",
                add_tag_toggle_button_tooltip: "Add Tag",
                cancel_adding_new_tag_button_label: "cancel adding tag",
                toggle_tag_filter_button_label: "toggle tag filter",
                remove_tag_from_task_button_label: "remove tag from task",
                toggle_expand_task_button_label: "toggle expand task",
                due_date_section_label: "due date",
                edit_due_date_tooltip: "Edit Due Date",
                due_date_form_label: "set due date",
                due_date_input_label: "Due",
                set_due_date_button_label: "set due date",
                cancel_due_date_update_button_label: "cancel due date update",
                color_picker_legend_label: "Color",
                description_update_form_label: "update description",
                set_description_button_label: "set description",
                cancel_description_update_button_label: "cancel description update",
                bullet_points_button_tooltip: "Bullet Points",
                task_list_button_tooltip: "Task List",
                description_text_area_label: "Description",
                description_section_label: "Description",
                edit_description_tooltip: "Edit Description",
                additional_actions_section_label: "additional actions",
                delete_task_tooltip: "Delete Task",
                edit_tag_color_form_label: "Edit Color",
                edit_tag_color_button_label: "Edit Color",
                set_tag_color_button_label: "Set Color",
                cancel_tag_color_update_label: "Cancel Color Update",
                edit_tag_name_button_label: "Edit Name",
                edit_tag_name_form_label: "Edit Name",
                set_tag_name_button_label: "Set Name",
                cancel_tag_name_update_button_label: "Cancel Name Update",
                delete_tag_button_label: "Delete Tag",
                archive_tag_button_label: "Archive Tag",
                unarchive_tag_button_label: "Unarchive Tag",
                edit_user_color_form_label: "Edit Color",
                set_user_color_button_label: "Set Color",
                cancel_user_color_update_button_label: "Cancel Color Update",
                edit_user_color_button_label: "Edit Color",
                edit_user_name_form_label: "Edit Name",
                set_user_name_button_label: "Set Name",
                cancel_user_name_update_button_label: "Cancel Name Update",
                edit_user_name_button_label: "Edit Name",
                delete_user_button_label: "Delete User",
                task_status_section_label: "Task Status",
                filters_section_label: "Filters",
                languages_section_title: "Languages",
                board_list_section_label: "Board List",
                custom_task_button_label: "Custom Task",
                join_board_button_label: "Join Board",
                chat_gpt_limit_exceeded_title: "ChatGPT Limit Exceeded",
                or_label: "or",
                chat_gpt_limit_exceeded_content:
                    "You have reached the limit of ChatGPT calls. Please try again later.",
                chat_gpt_waiting_message: "Talking to ChatGPT...",
                chat_gpt_error_title: "ChatGPT Error",
                chat_gpt_error_content:
                    "An error occurred while trying to connect to Chat GPT. Please try again later.",
                chat_gpt_prompt_input_title: "ChatGPT Prompt",
                chat_gpt_daily_attempts_left: "daily attempts left",
                chat_gpt_prompt_input_content: "or pick one from the suggestions below:",
                chat_gpt_prompt_input_form_label: "chat gpt prompt",
                suggest_cupcake_recipe_prompt: "suggest cupcake recipe",
                paint_bedroom_prompt: "paint bedroom",
                friends_over_for_bbq_prompt: "friends over for BBQ",
                prepare_for_rome_vacation_prompt: "prepare for Rome vacation",
                house_tidy_prompt: "house tidy",
                fix_fence_prompt: "fix fence",
                chat_gpt_prompt_input_label: "Prompt:",
                join_board_form_label: "Join Board",
                join_board_input_label: "Board Name",
                cancel_joining_board_button_label: "Cancel Joining Board",
                add_task_button_label: "Add Task",
                delete_task_button_label: "Delete Task",
                remove_board_button_label: "Remove Board",
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
                to_do_column_title: "Nezačané",
                in_progress_column_title: "Prebiehajúce",
                done_column_title: "Hotovo",
                pick_language_tooltip: "Vybrať jazyk",
                toggle_show_themes_tooltip: "Prepnúť zobrazenie tém",
                toggle_dense_view_tooltip: "Prepnúť zobrazenie rozloženia",
                edit_board_title_tooltip: "Zmeniť názov",
                board_title_input_label: "Názov",
                board_title_update_form_label: "Zmeniť názov tabuľky",
                set_board_title_button_label: "Zmeniť názov",
                cancel_board_title_update_button_label: "Zrušiť zmenu názvu",
                task_title_input_label: "Názov",
                edit_task_title_tooltip: "Zmeniť názov",
                task_title_update_form_label: "Zmeniť názov úlohy",
                set_task_title_button_label: "Zmeniť názov",
                cancel_task_title_update_button_label: "Zrušiť zmenu názvu",
                set_task_status_section_label: "Zmeniť stav úlohy",
                to_do_button_tooltip: "Nezačaná",
                in_progress_button_tooltip: "Prebiehajúca",
                done_button_tooltip: "Hotová",
                task_actions_section_label: "Úkony",
                duplicate_task_button_tooltip: "Duplikovať Úlohu",
                archive_task_button_tooltip: "Archivovať Úlohu",
                assignees_section_label: "Pridelení",
                assign_user_toggle_button_tooltip: "Prideliť Používateľa",
                toggle_user_filter_button_label: "prepnúť filter používateľa",
                assignee_selection_section_label: "Vyber používatelov",
                add_user_button_label: "Pridať Používatela",
                add_user_form_label: "Pridať Používatela",
                user_name_input_label: "Meno",
                cancel_adding_new_user_button_label: "Zrušiť pridanie nového používateľa",
                remove_user_from_task_button_label: "odstrániť používateľa z úlohy",
                tags_section_label: "Značky",
                tag_selection_section_label: "Vyber značiek",
                add_tag_button_label: "Pridať Značku",
                add_tag_form_label: "Pridať Značku",
                tag_name_input_label: "Meno",
                add_tag_toggle_button_tooltip: "Pridať Značku",
                cancel_adding_new_tag_button_label: "Zrušiť pridanie novej značky",
                toggle_tag_filter_button_label: "prepnúť filter značky",
                remove_tag_from_task_button_label: "odstrániť značku z úlohy",
                toggle_expand_task_button_label: "Prepnúť ...",
                due_date_section_label: "foofe...",
                edit_due_date_tooltip: "Zmenit...",
                due_date_form_label: "...",
                due_date_input_label: "due date...",
                set_due_date_button_label: "foo...",
                cancel_due_date_update_button_label: "bar...",
                color_picker_legend_label: "Farba",
                description_update_form_label: "update description...",
                set_description_button_label: "set description...",
                cancel_description_update_button_label: "cancel description update...",
                bullet_points_button_tooltip: "Bullet Points...",
                task_list_button_tooltip: "Task List...",
                description_text_area_label: "Description...",
                description_section_label: "Description...",
                edit_description_tooltip: "Edit Description...",
                additional_actions_section_label: "additional actions...",
                delete_task_tooltip: "Delete Task...",
                edit_tag_color_form_label: "Edit Color...",
                edit_tag_color_button_label: "Edit Color...",
                set_tag_color_button_label: "Set Color...",
                cancel_tag_color_update_label: "Cancel Color Update...",
                edit_tag_name_button_label: "Edit Name...",
                edit_tag_name_form_label: "Edit Name...",
                set_tag_name_button_label: "Set Name...",
                cancel_tag_name_update_button_label: "Cancel Name Update...",
                delete_tag_button_label: "Delete Tag...",
                archive_tag_button_label: "Archive Tag...",
                unarchive_tag_button_label: "Unarchive Tag...",
                edit_user_color_form_label: "Edit Color...",
                set_user_color_button_label: "Set Color...",
                cancel_user_color_update_button_label: "Cancel Color Update...",
                edit_user_color_button_label: "Edit Color...",
                edit_user_name_form_label: "Edit Name...",
                set_user_name_button_label: "Set Name...",
                cancel_user_name_update_button_label: "Cancel Name Update...",
                edit_user_name_button_label: "Edit Name...",
                delete_user_button_label: "Delete User...",
                task_status_section_label: "Task Status...",
                filters_section_label: "Filters...",
                languages_section_title: "Languages...",
                custom_task_button_label: "Custom Task...",
                board_list_section_label: "Board List...",
                join_board_button_label: "Join Board...",
                or_label: "or...",
                chat_gpt_limit_exceeded_title: "ChatGPT Limit Exceeded...",
                chat_gpt_limit_exceeded_content:
                    "You have reached the limit of ChatGPT calls. Please try again later...",
                chat_gpt_waiting_message: "Talking to ChatGPT!!!",
                chat_gpt_error_title: "ChatGPT Error...",
                chat_gpt_error_content:
                    "An error occurred while trying to connect to Chat GPT. Please try again later...",
                chat_gpt_prompt_input_title: "ChatGPT Prompt...",
                chat_gpt_daily_attempts_left: "daily attempts left...",
                chat_gpt_prompt_input_content: "or pick one from the suggestions below...",
                chat_gpt_prompt_input_form_label: "chat gpt prompt...",
                chat_gpt_prompt_input_label: "Prompt...",
                suggest_cupcake_recipe_prompt: "suggest cupcake recipe...",
                paint_bedroom_prompt: "paint bedroom...",
                friends_over_for_bbq_prompt: "friends over for BBQ...",
                prepare_for_rome_vacation_prompt: "prepare for Rome vacation...",
                house_tidy_prompt: "house tidy...",
                fix_fence_prompt: "fix fence...",
                join_board_form_label: "Join Board...",
                join_board_input_label: "Board Name...",
                cancel_joining_board_button_label: "Cancel Joining Board...",
                add_task_button_label: "Add Task...",
                delete_task_button_label: "Delete Task...",
                remove_board_button_label: "Remove Board...",
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
