use crate::commands::{FocusCommand, FocusTarget, ScrollCommand, ScrollTarget};
use crate::model::UnloadUrl;
use crate::route::Route;
use crate::themes::{themes, SavedTheme};
use dioxus::prelude::*;
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
    use_context_provider(|| saved_theme);
    use_context_provider(|| Signal::new(UnloadUrl(origin)));
    use_context_provider(|| Signal::new(ScrollTarget::default()));
    use_context_provider(|| Signal::new(FocusTarget::default()));
    rsx! {
        Router::<Route>{}
        ScrollCommand {}
        FocusCommand {}
    }
}
