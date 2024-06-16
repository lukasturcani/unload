use dioxus::prelude::*;
use shared_models::BoardName;

use crate::{components::nav::NavBar, themes::Theme};

#[component]
pub fn OneColumnBoard(board_name: BoardName) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("{} {}", theme.text_color, theme.bg_color_1);
    rsx! {
        div {
            class: "flex flex-col h-dvh w-screen {style}",
            Header {
                body: rsx! {

                }
            }
            Column {}
            NavBar { board_name }
        }
    }
}

#[component]
fn Header(body: Element) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        border-b {}
        ",
        theme.border_color
    );
    rsx! {
        header {
            class: "
                flex flex-row items-center justify-around
                w-full h-14 shrink-0 grow-0
                {style}
            ",
            {body}
        }
    }
}

#[component]
fn Column() -> Element {
    rsx! {
        div {}
    }
}
