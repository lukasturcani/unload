use crate::commands::{FocusCommand, FocusTarget, ScrollCommand, ScrollTarget};
use crate::model::{AppSettings, UnloadUrl};
use crate::route::Route;
use crate::themes::THEMES;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let settings = use_context_provider(|| Signal::new(AppSettings::default()));
    use_context_provider(move || {
        let theme_name = settings.read().theme();
        let theme = THEMES
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
