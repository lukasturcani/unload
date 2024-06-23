use dioxus::prelude::*;
use dioxus_sdk::storage::*;

use crate::themes::Theme;

#[component]
pub fn ThemeButton(theme: Theme) -> Element {
    let mut app_theme = use_context::<Signal<Theme>>();
    let mut stored_theme =
        use_synced_storage::<LocalStorage, String>("theme".to_string(), move || {
            app_theme.read().name.to_string()
        });
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
                stored_theme.set(theme.name.into());
                app_theme.set(theme);
            },
            {theme.name}
        }
    }
}
