use crate::commands::{FocusCommand, FocusTarget, ScrollCommand, ScrollTarget};
use crate::model::{BoardLanguage, UnloadUrl, UrlLanguage, Welcome};
use crate::route::Route;
use crate::themes::{themes, SavedTheme};
use crate::translations::languages;
use dioxus::prelude::*;
use dioxus_sdk::i18n::*;
use dioxus_sdk::storage::*;
use reqwest::Url;
use unic_langid_impl::LanguageIdentifier;

#[component]
pub fn App(origin: Url, default_language: BoardLanguage) -> Element {
    let themes = use_context_provider(|| Signal::new(themes()));
    let saved_theme =
        use_synced_storage::<LocalStorage, SavedTheme>("theme".to_string(), move || {
            SavedTheme(themes.read()[0].name.to_string())
        });
    use_context_provider(move || {
        let themes = themes.read();
        let saved_theme = saved_theme.read();
        match themes
            .iter()
            .find(|theme| theme.name == saved_theme.0.as_str())
        {
            Some(theme) => Signal::new(*theme),
            None => Signal::new(themes[0]),
        }
    });
    let language =
        use_synced_storage::<LocalStorage, BoardLanguage>("language".to_string(), || {
            default_language
        });
    let language_ = language.read().0.parse::<LanguageIdentifier>().unwrap();
    use_init_i18n(language_.clone(), language_, languages);
    use_context_provider(|| language);
    use_context_provider(|| saved_theme);
    use_context_provider(|| Signal::new(UrlLanguage::default()));
    use_context_provider(|| Signal::new(UnloadUrl(origin)));
    use_context_provider(|| Signal::new(ScrollTarget::default()));
    use_context_provider(|| Signal::new(FocusTarget::default()));
    use_context_provider(|| Signal::new(Welcome::default()));
    rsx! {
        Router::<Route>{}
        ScrollCommand {}
        FocusCommand {}
    }
}
