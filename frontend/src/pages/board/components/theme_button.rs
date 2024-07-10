use dioxus::prelude::*;

use crate::themes::{SavedTheme, Theme};

#[component]
pub fn ThemeButton(theme: Theme) -> Element {
    let mut app_theme = use_context::<Signal<Theme>>();
    let mut saved_theme = use_context::<Signal<SavedTheme>>();
    let app_theme_ = app_theme.read();
    let style = format!(
        "
        rounded border-2 {}
    ",
        app_theme_.button
    );
    rsx! {
        button {
            class: "
                flex flex-row items-center
                shrink-0
                px-1.5 py-0.5
                {style}
            ",
            "aria-pressed": app_theme_.name == theme.name,
            onclick: move |_| {
                saved_theme.set(SavedTheme(theme.name.into()));
                app_theme.set(theme);
            },
            {theme.name}
        }
    }
}
