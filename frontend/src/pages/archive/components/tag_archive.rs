use dioxus::prelude::*;
use shared_models::BoardName;

use crate::model::UnloadUrl;
use crate::pages::archive::components::tag_list::TagList;
use crate::pages::archive::model::{TagEntries, TagsUrl};
use crate::pages::archive::requests;

#[component]
pub fn TagArchive(board_name: BoardName) -> Element {
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
    rsx! { TagsTab { board_name } }
}

#[component]
fn TagsTab(board_name: BoardName) -> Element {
    rsx! {
        div {
            class: "
                grow container mx-auto py-4 h-full overflow-y-auto
                flex flex-col items-center justify-center
            ",
            TagList {}
        }
    }
}
