use crate::commands::{FocusCommand, FocusTarget, ScrollCommand, ScrollTarget};
use crate::model::{UnloadUrl, Welcome};
use crate::route::Route;
use crate::themes::{themes, SavedTheme};
use crate::translations::languages;
use dioxus::prelude::*;
use dioxus_sdk::i18n::*;
use dioxus_sdk::storage::*;
use reqwest::Url;

#[component]
pub fn App(origin: Url) -> Element {
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
    use_init_i18n("en".parse().unwrap(), "en".parse().unwrap(), languages);
    use_context_provider(|| saved_theme);
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
