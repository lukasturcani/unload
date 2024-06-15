use dioxus::prelude::*;

use crate::pages::archive::components::task_list::TaskList;
use crate::pages::archive::model::{BoardUrl, TaskEntries};
use crate::pages::archive::requests;
use shared_models::BoardName;

#[component]
pub fn TaskArchive(board_name: BoardName) -> Element {
    let url = use_context::<Signal<BoardUrl>>();
    let tags = use_context_provider(|| Signal::new(TaskEntries::default()));
    use_future(move || requests::get_tasks(tags, url));
    rsx! { TasksTab { board_name } }
}

#[component]
fn TasksTab(board_name: BoardName) -> Element {
    rsx! {
        div {
            class: "
                grow container mx-auto py-4 h-full overflow-y-auto
                flex flex-col items-center justify-center
            ",
            TaskList {}
        }
    }
}
