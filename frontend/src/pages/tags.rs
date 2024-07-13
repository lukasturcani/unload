use dioxus::prelude::*;
use shared_models::BoardName;

use crate::components::nav::NavBar;
use crate::model::UnloadUrl;
use crate::pages::tags::components::TagList;
use crate::pages::tags::model::{TagEntries, TagsUrl};
use crate::themes::Theme;

mod components;
mod model;
mod requests;

#[component]
pub fn Tags(board_name: BoardName) -> Element {
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
    let style = format!("{} {}", theme.app_style, theme.bg_color_1);
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
