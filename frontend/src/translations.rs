use dioxus_sdk::i18n::Language;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
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
pub struct Translation<T> {
    pub id: T,
    pub name: T,
    text: Text<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Text<T> {
    to_do_column_title: T,
    in_progress_column_title: T,
    done_column_title: T,
    pick_language_tooltip: T,
    toggle_show_themes_tooltip: T,
    toggle_dense_view_tooltip: T,
    edit_board_title_tooltip: T,
    board_title_input_label: T,
    board_title_update_form_label: T,
    set_board_title_button_label: T,
    cancel_board_title_update_button_label: T,
    task_title_input_label: T,
    edit_task_title_tooltip: T,
    task_title_update_form_label: T,
    set_task_title_button_label: T,
    cancel_task_title_update_button_label: T,
    set_task_status_section_label: T,
    to_do_button_tooltip: T,
    in_progress_button_tooltip: T,
    done_button_tooltip: T,
    task_actions_section_label: T,
    duplicate_task_button_tooltip: T,
    archive_task_button_tooltip: T,
    unarchive_task_button_tooltip: T,
    assignees_section_label: T,
    assign_user_toggle_button_tooltip: T,
    toggle_user_filter_button_label: T,
    assignee_selection_section_label: T,
    add_user_button_label: T,
    add_user_form_label: T,
    user_name_input_label: T,
    cancel_adding_new_user_button_label: T,
    remove_user_from_task_button_label: T,
    tags_section_label: T,
    tag_selection_section_label: T,
    add_tag_button_label: T,
    add_tag_form_label: T,
    tag_name_input_label: T,
    add_tag_toggle_button_tooltip: T,
    cancel_adding_new_tag_button_label: T,
    toggle_tag_filter_button_label: T,
    remove_tag_from_task_button_label: T,
    toggle_expand_task_button_label: T,
    due_date_section_label: T,
    edit_due_date_tooltip: T,
    due_date_form_label: T,
    due_date_input_label: T,
    set_due_date_button_label: T,
    cancel_due_date_update_button_label: T,
    color_picker_legend_label: T,
    description_update_form_label: T,
    set_description_button_label: T,
    cancel_description_update_button_label: T,
    bullet_points_button_tooltip: T,
    task_list_button_tooltip: T,
    description_text_area_label: T,
    description_section_label: T,
    edit_description_tooltip: T,
    additional_actions_section_label: T,
    delete_task_tooltip: T,
    edit_tag_color_form_label: T,
    edit_tag_color_button_label: T,
    set_tag_color_button_label: T,
    cancel_tag_color_update_label: T,
    edit_tag_name_button_label: T,
    edit_tag_name_form_label: T,
    set_tag_name_button_label: T,
    cancel_tag_name_update_button_label: T,
    delete_tag_button_label: T,
    archive_tag_button_label: T,
    unarchive_tag_button_label: T,
    edit_user_color_form_label: T,
    set_user_color_button_label: T,
    cancel_user_color_update_button_label: T,
    edit_user_color_button_label: T,
    edit_user_name_form_label: T,
    set_user_name_button_label: T,
    cancel_user_name_update_button_label: T,
    edit_user_name_button_label: T,
    delete_user_button_label: T,
    task_status_section_label: T,
    filters_section_label: T,
    languages_section_title: T,
    custom_task_button_label: T,
    board_list_section_label: T,
    join_board_button_label: T,
    create_new_board_button_label: T,
    or_label: T,
    chat_gpt_limit_exceeded_title: T,
    chat_gpt_limit_exceeded_content: T,
    chat_gpt_waiting_message: T,
    chat_gpt_error_title: T,
    chat_gpt_error_content: T,
    chat_gpt_prompt_input_title: T,
    chat_gpt_daily_attempts_left: T,
    chat_gpt_prompt_input_content: T,
    chat_gpt_prompt_input_form_label: T,
    chat_gpt_prompt_input_label: T,
    suggest_cupcake_recipe_prompt: T,
    paint_bedroom_prompt: T,
    friends_over_for_bbq_prompt: T,
    prepare_for_rome_vacation_prompt: T,
    house_tidy_prompt: T,
    fix_fence_prompt: T,
    join_board_form_label: T,
    join_board_input_label: T,
    cancel_joining_board_button_label: T,
    add_task_button_label: T,
    remove_board_button_label: T,
    new_task_form_label: T,
    cancel_adding_new_task_button_label: T,
    navigation_section_label: T,
    toggle_actions_drawer_button_label: T,
    toggle_show_filters_button_label: T,
    themes_section_label: T,
    close_theme_selector_button_label: T,
    close_filters_button_label: T,
    board_link: T,
    tags_link: T,
    users_link: T,
    archive_link: T,
}

pub fn translations() -> Vec<Translation<&'static str>> {
    SupportedLanguage::iter()
        .sorted_unstable_by_key(|x| x.id())
        .map(Translation::from)
        .collect()
}

pub fn languages() -> Vec<Language> {
    translations().iter().map(Language::from).collect()
}

impl<T> From<&Translation<T>> for Language
where
    T: Serialize,
{
    fn from(translation: &Translation<T>) -> Self {
        let mut value = serde_json::to_value(translation).unwrap();
        let value = value.as_object_mut().unwrap();
        value.remove("name");
        let text = value.remove("text").unwrap();
        value.insert("texts".into(), text);
        Language::from_str(&serde_json::to_string(value).unwrap()).unwrap()
    }
}

impl From<SupportedLanguage> for Translation<&'static str> {
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
