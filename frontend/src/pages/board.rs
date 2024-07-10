use dioxus::prelude::*;
use dioxus_sdk::storage::*;

use crate::model::SavedBoards;
use crate::pages::board::model::{
    Board, Dense, QuickAddTasks, TagFilter, Tags, Tasks, UserFilter, Users,
};
use crate::pages::board::one_column_board::OneColumnBoard;
use crate::pages::board::requests::BoardSignals;
use crate::pages::board::three_column_board::ThreeColumnBoard;
use crate::responsive_layout::ResponsiveLayout;
use crate::window::use_window_size;

use shared_models::BoardName;

mod components;
mod model;
mod one_column_board;
mod requests;
mod three_column_board;

#[component]
pub fn Board(board_name: BoardName) -> Element {
    let boards =
        use_synced_storage::<LocalStorage, SavedBoards>("boards".to_string(), SavedBoards::default);
    let dense = use_synced_storage::<LocalStorage, Dense>("dense".to_string(), Dense::default);
    use_context_provider(|| dense);
    use_context_provider(|| Signal::new(Board::default()));
    use_context_provider(|| Signal::new(Tasks::default()));
    use_context_provider(|| Signal::new(Users::default()));
    use_context_provider(|| Signal::new(Tags::default()));
    use_context_provider(|| Signal::new(QuickAddTasks::default()));
    use_context_provider(|| Signal::new(UserFilter::default()));
    use_context_provider(|| Signal::new(TagFilter::default()));
    use_context_provider(|| boards);
    let window_size = use_window_size()();
    let layout = ResponsiveLayout::from_window_size(window_size);
    eval(&format!(r#"document.title = "{board_name}";"#));
    let mut board_signals = BoardSignals::default();
    if board_signals.board.read().board_name != board_name {
        board_signals.board.write().board_name = board_name.clone();
    }
    use_future(move || requests::board(board_signals));
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
