use dioxus::prelude::*;
use shared_models::BoardName;

use crate::{
    components::{
        icons::{DoneIcon, SolidDoneIcon, SolidTagIcon, TagIcon},
        nav::NavBar,
    },
    pages::archive::components::{TagArchive, TaskArchive},
    themes::Theme,
};

mod components;
mod model;
mod requests;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tab {
    Tasks,
    Tags,
}

#[component]
pub fn Archive(board_name: BoardName) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let mut tab = use_signal(|| Tab::Tasks);
    let tab_ = tab();
    let style = "
        sm:hover:underline
        sm:hover:aria-selected:no-underline
        aria-selected:border-b border-white
    ";
    let page_style = format!("{} {}", theme.bg_color_1, theme.text_color);
    rsx! {
        div {
            class: "
                w-screen h-dvh
                flex flex-col
                {page_style}
            ",
            div {
                class: "grid grid-cols-2 place-items-center h-14 shrink-0 grow-0",
                button {
                    "aria-selected": tab_ == Tab::Tasks,
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
                    "aria-selected": tab_ == Tab::Tags,
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
                Tab::Tasks => rsx!{},
                Tab::Tags => rsx!{TagArchive { board_name: board_name.clone() }},
            }
            NavBar { board_name }
        }
    }
}
