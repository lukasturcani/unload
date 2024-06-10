use dioxus::prelude::*;
use shared_models::BoardName;

use crate::{
    components::{
        icons::{DoneIcon, SolidDoneIcon, SolidTagIcon, TagIcon},
        nav::NavBar,
    },
    tag_archive::ArchivedTags,
    task_archive::TaskArchive,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tab {
    Tasks,
    Tags,
}

#[component]
pub fn Archive(board_name: BoardName) -> Element {
    let mut tab = use_signal(|| Tab::Tasks);
    let tab_ = tab();
    let style = "
        sm:hover:underline
        aria-pressed:underline
    ";
    rsx! {
        div {
            class: "
                w-screen h-dvh
                bg-gray-900
                flex flex-col
                text-white stroke-white
            ",
            div {
                class: "grid grid-cols-2 place-items-center h-14 shrink-0 grow-0",
                button {
                    "aria-pressed": tab_ == Tab::Tasks,
                    class: "
                        size-full flex flex-row items-center justify-center gap-1
                        {style}
                    ",
                    onclick: move |_| tab.set(Tab::Tasks),
                    div {
                        class: "size-5",
                        match tab_ {
                            Tab::Tasks => rsx!{SolidDoneIcon {}},
                            Tab::Tags => rsx!{DoneIcon {}},
                        }
                    }
                    "Tasks"
                }
                button {
                    "aria-pressed": tab_ == Tab::Tags,
                    class: "
                        size-full flex flex-row items-center justify-center gap-1
                        {style}
                    ",
                    onclick: move |_| tab.set(Tab::Tags),
                    div {
                        class: "size-5",
                        match tab_ {
                            Tab::Tasks => rsx!{TagIcon {}},
                            Tab::Tags => rsx!{SolidTagIcon {}},
                        }
                    }
                    "Tags"
                }
            }
            match tab_ {
                Tab::Tasks => rsx!{TaskArchive { board_name: board_name.clone() }},
                Tab::Tags => rsx!{ArchivedTags { board_name: board_name.clone() }},
            }
            NavBar { board_name }
        }
    }
}
