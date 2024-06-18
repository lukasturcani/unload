use dioxus::prelude::*;

use crate::{model::AppSettings, themes::Theme};

#[component]
pub fn ThemeButton(theme: Theme) -> Element {
    let mut settings = use_context::<Signal<AppSettings>>();
    let mut app_theme = use_context::<Signal<Theme>>();
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
                px-1.5 py-0.5 {style}
            ",
            "aria-pressed": app_theme() == theme,
            onclick: move |_| {
                let mut settings = settings.write();
                settings.set_theme(theme.name.to_string());
                app_theme.set(theme);
            },
            {theme.name}
        }
    }
}
