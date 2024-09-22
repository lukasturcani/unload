use dioxus::prelude::*;
use dioxus_sdk::i18n::use_i18;
use shared_models::BoardName;
use unic_langid_impl::LanguageIdentifier;

use crate::components::nav::NavBar;
use crate::model::{UnloadUrl, UrlLanguage};
use crate::pages::tags::components::TagList;
use crate::pages::tags::model::{TagEntries, TagsUrl};
use crate::themes::Theme;

mod components;
mod model;
mod requests;

#[component]
pub fn Tags(board_name: BoardName) -> Element {
    rsx! {
        LanguageTags {
            language: "",
            board_name
        }
    }
}

#[component]
pub fn LanguageTags(language: ReadOnlySignal<String>, board_name: BoardName) -> Element {
    let mut url_language = use_context::<Signal<UrlLanguage>>();
    let language = language.read();
    if *language != url_language.read().0 {
        url_language.write().0 = language.clone();
    }
    let mut i18 = use_i18();
    if !language.is_empty() && *language != i18.selected_language.read().to_string() {
        i18.set_language(language.parse::<LanguageIdentifier>().unwrap());
    }
    let url = use_context::<Signal<UnloadUrl>>();
    eval(&format!(r#"document.title = "{board_name}";"#));
    let tags = use_context_provider(|| Signal::new(TagEntries::default()));
    let url = use_context_provider({
        let board_name = board_name.clone();
        move || {
            let url = url
                .read()
                .0
                .join(&format!("/api/boards/{}/tags", board_name))
                .unwrap();
            Signal::new(TagsUrl(url))
        }
    });
    use_future(move || requests::get_tags(tags, url));
    rsx! { TagsPage { board_name } }
}

#[component]
fn TagsPage(board_name: BoardName) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("{} {}", theme.text_color, theme.bg_color_1);
    rsx! {
        div {
            class: "flex flex-col h-dvh w-screen {style}",
            div {
                class: "
                    grow container mx-auto sm:py-4 h-full overflow-y-auto
                    flex flex-col items-center justify-center
                ",
                TagList {}
            }
            NavBar { board_name }
        }
    }
}
