use dioxus::prelude::*;
use dioxus_sdk::{i18n::use_i18, storage::*, translate};
use reqwest::Client;
use shared_models::{BoardName, SavedBoard};
use unic_langid_impl::LanguageIdentifier;

use crate::{
    components::{icons::TrashIcon, input::TextInput},
    model::{SavedBoards, UnloadUrl, UrlLanguage},
    route::Route,
    themes::Theme,
};

#[component]
pub fn JoinBoard(language: ReadOnlySignal<String>) -> Element {
    let mut url_language = use_context::<Signal<UrlLanguage>>();
    let language = language.read();
    if *language != url_language.read().0 {
        url_language.write().0 = language.clone();
    }
    let mut i18 = use_i18();
    if *language != i18.selected_language.read().to_string() {
        i18.set_language(language.parse::<LanguageIdentifier>().unwrap());
    }
    let boards =
        use_synced_storage::<LocalStorage, SavedBoards>("boards".to_string(), SavedBoards::default);
    use_context_provider(|| boards);
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("{} {}", theme.text_color, theme.bg_color_1);
    let nav = use_navigator();
    let input_label = translate!(i18, "board_name_input_label");
    rsx! {
        div{
            class: "
                flex flex-col items-center
                h-dvh w-screen py-4 px-2 gap-4
                {style}
            ",
            BoardList {}
            form {
                class: "flex flex-row flex-wrap items-center gap-2 justify-center",
                onsubmit: move |event| {
                    let board_name = event.values()[&input_label].as_value().into();
                    let language = &url_language.read().0;
                    if language.is_empty() {
                        nav.push(Route::Board { board_name });
                    } else {
                        nav.push(Route::LanguageBoard {
                            language: language.clone(),
                            board_name,
                        });
                    }
                },
                TextInput {
                    id: "board_name",
                    label: input_label.clone(),
                }
                JoinBoardButton {}
            }
            div {
                class: "inline-flex items-center justify-center",
                hr {
                    class: "w-64 h-px border-0 bg-gray-700",
                },
                span {
                    class: "absolute px-3 font-medium -translate-x-1/2 left-1/2 text-white bg-gray-900",
                    {translate!(i18, "or_label")}
                },
            },
            div {
                class: "inline-flex items-center justify-center",
                CreateBoardButton {},
            }
        }
    }
}

#[component]
fn JoinBoardButton() -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("rounded-lg {}", theme.primary_button);
    rsx! {
        button {
            r#type: "submit",
            class: "
                w-full sm:w-auto
                px-5 py-2.5
                text-sm text-center font-medium
                {style}
            ",
            {translate!(i18, "join_board_button_label")}
        }
    }
}

#[component]
fn CreateBoardButton() -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let url = use_context::<Signal<UnloadUrl>>();
    let nav = use_navigator();
    let style = format!("rounded-lg {}", theme.primary_button);
    rsx! {
        button {
            class: "
                w-full sm:w-auto
                px-5 py-2.5
                text-sm text-center font-medium
                {style}
            ",
            onclick: move |_| create_board(url, nav),
            {translate!(i18, "create_new_board_button_label")}
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
    let i18 = use_i18();
    let style = "stroke-red-600";
    rsx! {
        button {
            aria_label: translate!(i18, "remove_board_button_label"),
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
