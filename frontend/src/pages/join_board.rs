use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use reqwest::Client;
use shared_models::BoardName;

use crate::{components::input::TextInput, model::UnloadUrl, route::Route, themes::Theme};

#[component]
pub fn JoinBoard() -> Element {
    let url = use_context::<Signal<UnloadUrl>>();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("{} {}", theme.text_color, theme.bg_color_1);
    let nav = use_navigator();
    let mut boards =
        use_synced_storage::<LocalStorage, Vec<BoardName>>("boards".to_string(), Vec::default);
    log::info!("Boards: {:?}", boards.read());
    rsx! {
        div{
            class: "flex flex-col h-dvh w-screen py-4 px-2 gap-4 {style}",
            form {
                class: "w-full flex flex-row flex-wrap items-center gap-2 justify-center",
                onsubmit: move |event| {
                    let board_name: BoardName = event.values()["Board Name"].as_value().into();
                    if !boards.read().contains(&board_name) {
                        boards.write().push(board_name.clone());
                    }
                    log::info!("Boards: {:?}", boards.read());
                    // nav.push(Route::Board { board_name });
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
                class: "inline-flex items-center justify-center w-full",
                hr {
                    class: "w-64 h-px border-0 bg-gray-700",
                },
                span {
                    class: "absolute px-3 font-medium -translate-x-1/2 left-1/2 text-white bg-gray-900",
                    "or"
                },
            },
            div {
                class: "inline-flex items-center justify-center w-full",
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
    let boards =
        use_synced_storage::<LocalStorage, Vec<BoardName>>("boards".to_string(), Vec::default);
    rsx! {
        ul {
            class: "
                flex flex-col items-center justify-center
                {style}
            ",
            for board in boards.read().clone() {
                BoardListItem { board }
            }
        }
    }
}

#[component]
fn BoardListItem(board: BoardName) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("last:border-none border-b {}", theme.border_color);
    rsx! {
        li {
            class: "w-full px-2 {style}",
            a {
                class: "w-full",
                href: format!("/boards/{}", board),
                div { class: "w-full", "{board}" },
            }
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
