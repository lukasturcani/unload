use dioxus_sdk::i18n::Language;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use shared_models::{IntoEnumIterator, SupportedLanguage};
use std::str::FromStr;

mod am;
mod ar;
mod az;
mod bg;
mod bho;
mod bn;
mod cs;
mod da;
mod de;
mod el;
mod en;
mod es;
mod et;
mod fa;
mod fi;
mod fr;
mod gu;
mod ha;
mod hi;
mod hr;
mod hu;
mod hy;
mod id;
mod ig;
mod it;
mod ja;
mod jv;
mod ka;
mod kk;
mod kn;
mod ko;
mod lt;
mod lv;
mod mr;
mod ms;
mod nl;
mod no;
mod pa;
mod pl;
mod pt;
mod ro;
mod ru;
mod sk;
mod sr;
mod sv;
mod sw;
mod ta;
mod te;
mod tg;
mod th;
mod tl;
mod tr;
mod uk;
mod ur;
mod uz;
mod vi;
mod yo;
mod yue;
mod zh;

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
    unarchive_task_button_tooltip: &'static str,
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
    create_new_board_button_label: &'static str,
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
    remove_board_button_label: &'static str,
    new_task_form_label: &'static str,
    cancel_adding_new_task_button_label: &'static str,
    navigation_section_label: &'static str,
    toggle_actions_drawer_button_label: &'static str,
    toggle_show_filters_button_label: &'static str,
    themes_section_label: &'static str,
    close_theme_selector_button_label: &'static str,
    close_filters_button_label: &'static str,
    board_link: &'static str,
    tags_link: &'static str,
    users_link: &'static str,
    archive_link: &'static str,
}

pub fn translations() -> Vec<Translation> {
    SupportedLanguage::iter()
        .sorted_unstable_by_key(|x| x.id())
        .map(Translation::from)
        .collect()
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

impl From<SupportedLanguage> for Translation {
    fn from(language: SupportedLanguage) -> Self {
        match language {
            SupportedLanguage::English => en::EN,
            SupportedLanguage::Slovak => sk::SK,
            SupportedLanguage::Korean => ko::KO,
            SupportedLanguage::French => fr::FR,
            SupportedLanguage::Italian => it::IT,
            SupportedLanguage::Portuguese => pt::PT,
            SupportedLanguage::Spanish => es::ES,
            SupportedLanguage::Czech => cs::CS,
            SupportedLanguage::Polish => pl::PL,
            SupportedLanguage::Croatian => hr::HR,
            SupportedLanguage::Serbian => sr::SR,
            SupportedLanguage::Bulgarian => bg::BG,
            SupportedLanguage::Ukranian => uk::UK,
            SupportedLanguage::Russian => ru::RU,
            SupportedLanguage::Romanian => ro::RO,
            SupportedLanguage::Hungarian => hu::HU,
            SupportedLanguage::German => de::DE,
            SupportedLanguage::Turkish => tr::TR,
            SupportedLanguage::Farsi => fa::FA,
            SupportedLanguage::Hindi => hi::HI,
            SupportedLanguage::Bengali => bn::BN,
            SupportedLanguage::Japanese => ja::JA,
            SupportedLanguage::Mandarin => zh::ZH,
            SupportedLanguage::Vietnamese => vi::VI,
            SupportedLanguage::Cantonese => yue::YUE,
            SupportedLanguage::Marathi => mr::MR,
            SupportedLanguage::Telugu => te::TE,
            SupportedLanguage::Tamil => ta::TA,
            SupportedLanguage::Urdu => ur::UR,
            SupportedLanguage::Gujarati => gu::GU,
            SupportedLanguage::Hausa => ha::HA,
            SupportedLanguage::Arabic => ar::AR,
            SupportedLanguage::Javanese => jv::JV,
            SupportedLanguage::Punjabi => pa::PA,
            SupportedLanguage::Dutch => nl::NL,
            SupportedLanguage::Swedish => sv::SV,
            SupportedLanguage::Norwegian => no::NO,
            SupportedLanguage::Danish => da::DA,
            SupportedLanguage::Greek => el::EL,
            SupportedLanguage::Lithuanian => lt::LT,
            SupportedLanguage::Latvian => lv::LV,
            SupportedLanguage::Finnish => fi::FI,
            SupportedLanguage::Estonian => et::ET,
            SupportedLanguage::Armenian => hy::HY,
            SupportedLanguage::Georgian => ka::KA,
            SupportedLanguage::Kazakh => kk::KK,
            SupportedLanguage::Tajik => tg::TG,
            SupportedLanguage::Uzbek => uz::UZ,
            SupportedLanguage::Azeri => az::AZ,
            SupportedLanguage::Malay => ms::MS,
            SupportedLanguage::Indonesian => id::ID,
            SupportedLanguage::Yoruba => yo::YO,
            SupportedLanguage::Igbo => ig::IG,
            SupportedLanguage::Swahili => sw::SW,
            SupportedLanguage::Tagalog => tl::TL,
            SupportedLanguage::Thai => th::TH,
            SupportedLanguage::Amharic => am::AM,
            SupportedLanguage::Bhojpuri => bho::BHO,
            SupportedLanguage::Kannada => kn::KN,
        }
    }
}
