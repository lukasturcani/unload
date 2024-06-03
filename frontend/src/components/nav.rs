use dioxus::prelude::*;
use shared_models::BoardName;

use crate::{
    components::icons::{ArchiveIcon, BoardIcon, TagIcon, UsersIcon},
    route::Route,
    themes::Theme,
};

#[component]
pub fn NavBar(board_name: BoardName) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        border-t {}
        ",
        theme.border_color
    );
    rsx! {
        nav {
            class: "
                flex flex-row justify-center
                grow-0 shrink-0 w-full h-16
                {style}
            ",
            NavLink {
                to: Route::Board { board_name: board_name.clone() },
                body: rsx!{
                    div {
                        class: "flex flex-col items-center justify-center",
                        div { class: "size-8", BoardIcon {} }
                        "Board"
                    }
                }
            }
            NavLink {
                to: Route::TaskArchive { board_name: board_name.clone() },
                body: rsx!{
                    div {
                        class: "flex flex-col items-center justify-center",
                        div { class: "size-8", ArchiveIcon {} }
                        "Archive"
                    }
                }
            }
            NavLink {
                to: Route::Tags { board_name: board_name.clone() },
                body: rsx!{
                    div {
                        class: "flex flex-col items-center justify-center",
                        div { class: "size-8", TagIcon {} }
                        "Tags"
                    }
                }
            }
            NavLink {
                to: Route::Users { board_name },
                body: rsx!{
                    div {
                        class: "flex flex-col items-center justify-center",
                        div { class: "size-8", UsersIcon {} }
                        "Users"
                    }
                }
            }
        }
    }
}

#[component]
fn NavLink(to: Route, body: Element) -> Element {
    let shared_css = "
        h-full w-1/6
    ";
    let shared_style = "";
    let style = "";
    let active_style = "bg-white stroke-black text-black";
    rsx! {
        Link {
            class: "{shared_css} {shared_style} {style}",
            active_class: "{shared_css} {shared_style} {active_style}",
            to,
            {body}
        }
    }
}
