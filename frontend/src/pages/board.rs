use dioxus::prelude::*;
use dioxus_sdk::i18n::use_i18;
use dioxus_sdk::storage::*;
use model::NumChatGptCalls;

use crate::model::{BoardLanguage, SavedBoards};
use crate::pages::board::model::{Board, Dense, TagFilter, Tags, Tasks, UserFilter, Users};
use crate::pages::board::one_column_board::OneColumnBoard;
use crate::pages::board::requests::BoardSignals;
use crate::pages::board::three_column_board::ThreeColumnBoard;
use crate::responsive_layout::ResponsiveLayout;
use crate::route::Route;
use crate::window::use_window_size;

use shared_models::{BoardName, SavedBoard};

mod components;
mod model;
mod one_column_board;
mod requests;
mod three_column_board;

#[component]
pub fn LanguageBoard(language: ReadOnlySignal<String>, board_name: BoardName) -> Element {
    let mut board_language = use_context::<Signal<BoardLanguage>>();
    board_language.set(BoardLanguage(language.read().clone()));
    let mut i18 = use_i18();
    i18.set_language(language.read().parse().unwrap());
    let nav = use_navigator();
    nav.push(Route::Board { board_name });
    rsx! {}
}

#[component]
pub fn Board(board_name: BoardName) -> Element {
    let mut saved_boards =
        use_synced_storage::<LocalStorage, SavedBoards>("boards".to_string(), SavedBoards::default);
    let dense = use_synced_storage::<LocalStorage, Dense>("dense".to_string(), Dense::default);
    use_context_provider(|| dense);
    let board = use_context_provider(|| Signal::new(Board::default()));
    if saved_boards.read().0.iter().all(|b| b.name != board_name) {
        let mut saved_boards = saved_boards.write();
        saved_boards.0.push(SavedBoard {
            name: board_name.clone(),
            title: board.read().title.clone(),
        });
    }
    use_context_provider(|| Signal::new(NumChatGptCalls::default()));
    use_context_provider(|| Signal::new(Tasks::default()));
    use_context_provider(|| Signal::new(Users::default()));
    use_context_provider(|| Signal::new(Tags::default()));
    use_context_provider(|| Signal::new(UserFilter::default()));
    use_context_provider(|| Signal::new(TagFilter::default()));
    use_context_provider(|| saved_boards);
    let window_size = use_window_size()();
    let layout = ResponsiveLayout::from_window_size(window_size);
    eval(&format!(r#"document.title = "{board_name}";"#));
    let mut board_signals = BoardSignals::default();
    let mut board_future = use_future(move || requests::board(board_signals));
    if board_signals.board.read().board_name != board_name {
        board_signals.board.write().board_name = board_name.clone();
        board_future.restart();
    }
    rsx! {
        match layout {
            ResponsiveLayout::Narrow => rsx! {
                OneColumnBoard {
                    board_name: board_name.clone(),
                }
            },
            ResponsiveLayout::Wide => rsx! {
                ThreeColumnBoard {
                    board_name: board_name.clone(),
                }
            }
        }
    }
}
