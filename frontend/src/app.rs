use crate::commands::{FocusCommand, FocusTarget, ScrollCommand, ScrollTarget};
use crate::model::{AppSettings, UnloadUrl};
use crate::route::Route;
use crate::themes::themes;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let themes = use_context_provider(|| Signal::new(themes()));
    let theme = themes.read()[0].name.to_string();
    let settings = use_context_provider(|| Signal::new(AppSettings::new(theme)));
    use_context_provider(move || {
        let theme_name = settings.read().theme();
        let themes = themes.read();
        let theme = themes
            .iter()
            .find(|theme| theme.name == theme_name)
            .unwrap();
        Signal::new(*theme)
    });
    use_context_provider(|| Signal::new(UnloadUrl::default()));
    use_context_provider(|| Signal::new(ScrollTarget::default()));
    use_context_provider(|| Signal::new(FocusTarget::default()));
    rsx! {
        Router::<Route>{}
        ScrollCommand {}
        FocusCommand {}
    }
}
