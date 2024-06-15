use dioxus::prelude::*;
use shared_models::BoardName;

use crate::pages::archive::components::tag_list::TagList;
use crate::pages::archive::model::{BoardUrl, TagEntries};
use crate::pages::archive::requests;

#[component]
pub fn TagArchive(board_name: BoardName) -> Element {
    let url = use_context::<Signal<BoardUrl>>();
    let tags = use_context_provider(|| Signal::new(TagEntries::default()));
    use_future(move || requests::get_tag_entries(tags, url));
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
