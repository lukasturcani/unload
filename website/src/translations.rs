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
    open_main_menu: T,
    home_section_label: T,
    features_section_label: T,
    pricing_section_label: T,
    contact_section_label: T,
    select_language: T,
    go_to_app: T,
    app_link: T,
    home_link: T,
    new_board_link: T,
    new_board: T,
    h1_main: T,
    h1_sub: T,
    dense_button_label: T,
    dark_button_label: T,
    mobile_button_label: T,
}

pub fn languages() -> Vec<Language> {
    translations().iter().map(Language::from).collect()
}

pub fn translations() -> Vec<Translation<&'static str>> {
    SupportedLanguage::iter()
        .sorted_unstable_by_key(|x| x.id())
        .map(Translation::from)
        .collect()
}

impl<T> From<&Translation<T>> for Language
where
    T: Serialize,
{
    fn from(translation: &Translation<T>) -> Self {
        Language::from_str(&serde_json::to_string(translation).unwrap()).unwrap()
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
