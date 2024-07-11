use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use reqwest::Client;
use shared_models::{BoardName, SavedBoard};

use crate::{
    components::{icons::TrashIcon, input::TextInput},
    model::{SavedBoards, UnloadUrl},
    route::Route,
    themes::Theme,
};

#[component]
pub fn JoinBoard() -> Element {
    let boards =
        use_synced_storage::<LocalStorage, SavedBoards>("boards".to_string(), SavedBoards::default);
    use_context_provider(|| boards);
    let url = use_context::<Signal<UnloadUrl>>();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("{} {}", theme.text_color, theme.bg_color_1);
    let nav = use_navigator();
    rsx! {
        div{
            class: "
                flex flex-col items-center
                h-dvh w-screen py-4 px-2 gap-4
                {style}
            ",
            form {
                class: "flex flex-row flex-wrap items-center gap-2 justify-center",
                onsubmit: move |event| {
                    let board_name = event.values()["Board Name"].as_value().into();
                    nav.push(Route::Board { board_name });
                },
                TextInput {
                    id: "board_name",
                    label: "Board Name",
                }
                button {
                    r#type: "submit",
                    class: "
                        text-white
                        font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5
                        text-center bg-blue-600
                        active:bg-blue-700 sm:hover:bg-blue-700
                    ",
                    "Join Board"
                }
            }
            BoardList {}
            div {
                class: "inline-flex items-center justify-center",
                hr {
                    class: "w-64 h-px border-0 bg-gray-700",
                },
                span {
                    class: "absolute px-3 font-medium -translate-x-1/2 left-1/2 text-white bg-gray-900",
                    "or"
                },
            },
            div {
                class: "inline-flex items-center justify-center",
                button {
                    class: "
                        text-white
                        font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5
                        text-center bg-blue-600
                        active:bg-blue-700 sm:hover:bg-blue-700
                    ",
                    onclick: move |_| create_board(url, nav),
                    "Create New Board",
                },
            }
        }
    }
}

#[component]
pub fn BoardList() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border rounded-lg {}", theme.border_color);
    let boards = use_context::<Signal<SavedBoards>>();
    rsx! {
        ul {
            class: "
                flex flex-col items-center justify-center
                w-full max-w-96
                {style}
            ",
            for board in boards.read().0.clone() {
                BoardListItem { boards, board }
            }
        }
    }
}

#[component]
fn BoardListItem(boards: Signal<SavedBoards>, board: SavedBoard) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("last:border-none border-b {}", theme.border_color);
    rsx! {
        li {
            class: "
                w-full px-2
                flex flex-row justify-between items-center
                {style}
            ",
            a {
                class: "w-full",
                href: format!("/boards/{}", board.name),
                div {
                    class: "w-full flex flex-row gap-1",
                    p {
                        b { "{board.title} " }
                        "({board.name})"
                    }
                },
            }
            RemoveBoardButton { boards, board: board.clone() }
        }
    }
}

#[component]
fn RemoveBoardButton(boards: Signal<SavedBoards>, board: SavedBoard) -> Element {
    let style = "stroke-red-600";
    rsx! {
        button {
            "aria-label": "remove board",
            class: "size-5 {style}",
            onclick: move |_| {
                boards.write().0.retain(|b| b != &board);
            },
            TrashIcon {}
        }
    }
}

async fn create_board(url: Signal<UnloadUrl>, nav: Navigator) {
    if let Ok(board_name) = send_create_board_request(url).await {
        nav.push(Route::Board { board_name });
    }
}

async fn send_create_board_request(url: Signal<UnloadUrl>) -> Result<BoardName, anyhow::Error> {
    let request = {
        let url = &url.read().0;
        let client = Client::new();
        let url = url.join("/api/boards")?;
        client.post(url)
    };
    Ok(request.send().await?.json::<BoardName>().await?)
}
