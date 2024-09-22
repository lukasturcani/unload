use dioxus::prelude::*;
use dioxus_sdk::{i18n::use_i18, translate};
use shared_models::BoardName;

use crate::{
    components::icons::{
        ArchiveIcon, BoardIcon, SolidArchiveIcon, SolidBoardIcon, SolidTagIcon, SolidUsersIcon,
        TagIcon, UsersIcon,
    },
    route::Route,
    themes::Theme,
};

#[component]
pub fn NavBar(board_name: BoardName) -> Element {
    let i18 = use_i18();
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
                grow-0 shrink-0 w-full h-10 sm:h-16
                text-xs sm:text-base
                {style}
            ",
            NavLink {
                to: Route::Board { board_name: board_name.clone() },
                body: rsx!{
                    div {
                        class: "h-full flex flex-col items-center justify-center",
                        div { class: "size-5 sm:size-8 hidden group-[:not(.current-page)]:block", BoardIcon {} }
                        div { class: "size-5 sm:size-8 hidden group-[.current-page]:block", SolidBoardIcon {} }
                        {translate!(i18, "board_link")}
                    }
                }
            }
            NavLink {
                to: Route::Tags { board_name: board_name.clone() },
                body: rsx!{
                    div {
                        class: "h-full flex flex-col items-center justify-center",
                        div { class: "size-5 sm:size-8 hidden group-[:not(.current-page)]:block", TagIcon {} }
                        div { class: "size-5 sm:size-8 hidden group-[.current-page]:block", SolidTagIcon {} }
                        {translate!(i18, "tags_link")}
                    }
                }
            }
            NavLink {
                to: Route::Users { board_name: board_name.clone() },
                body: rsx!{
                    div {
                        class: "h-full flex flex-col items-center justify-center",
                        div { class: "size-5 sm:size-8 hidden group-[:not(.current-page)]:block", UsersIcon {} }
                        div { class: "size-5 sm:size-8 hidden group-[.current-page]:block", SolidUsersIcon {} }
                        {translate!(i18, "users_link")}
                    }
                }
            }
            NavLink {
                to: Route::Archive { board_name },
                body: rsx!{
                    div {
                        class: "h-full flex flex-col items-center justify-center",
                        div { class: "size-5 sm:size-8 hidden group-[:not(.current-page)]:block", ArchiveIcon {} }
                        div { class: "size-5 sm:size-8 hidden group-[.current-page]:block", SolidArchiveIcon {} }
                        {translate!(i18, "archive_link")}
                    }
                }
            }
        }
    }
}

#[component]
fn NavLink(to: Route, body: Element) -> Element {
    let shared_css = "
        group h-full w-full sm:max-w-44
    ";
    let style = "sm:hover:underline";
    let active_style = "current-page sm:underline";
    rsx! {
        Link {
            class: "{shared_css} {style}",
            active_class: active_style,
            to,
            {body}
        }
    }
}
