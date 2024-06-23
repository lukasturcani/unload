use crate::commands::{FocusCommand, FocusTarget, ScrollCommand, ScrollTarget};
use crate::model::UnloadUrl;
use crate::route::Route;
use crate::themes::themes;
use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use reqwest::Url;

#[component]
pub fn App(origin: Url) -> Element {
    let themes = use_context_provider(|| Signal::new(themes()));
    let stored_theme = use_synced_storage::<LocalStorage, String>("theme".to_string(), move || {
        themes.read()[0].name.to_string()
    });
    use_context_provider(move || {
        let themes = themes.read();
        let stored_theme = stored_theme.read();
        match themes
            .iter()
            .find(|theme| theme.name == stored_theme.as_str())
        {
            Some(theme) => Signal::new(*theme),
            None => Signal::new(themes[0]),
        }
    });
    use_context_provider(|| Signal::new(UnloadUrl(origin)));
    use_context_provider(|| Signal::new(ScrollTarget::default()));
    use_context_provider(|| Signal::new(FocusTarget::default()));
    rsx! {
        Router::<Route>{}
        ScrollCommand {}
        FocusCommand {}
    }
}
