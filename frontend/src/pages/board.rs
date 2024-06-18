use dioxus::prelude::*;

use crate::pages::board::model::{Board, QuickAddTasks, TagFilter, Tags, Tasks, UserFilter, Users};
use crate::pages::board::one_column_board::OneColumnBoard;
use crate::pages::board::requests::BoardSignals;
use crate::pages::board::three_column_board::ThreeColumnBoard;
use crate::responsive_layout::ResponsiveLayout;

use shared_models::BoardName;

mod components;
mod model;
mod one_column_board;
mod requests;
mod three_column_board;

#[component]
pub fn Board(board_name: BoardName) -> Element {
    use_context_provider(|| Signal::new(Board::default()));
    use_context_provider(|| Signal::new(Tasks::default()));
    use_context_provider(|| Signal::new(Users::default()));
    use_context_provider(|| Signal::new(Tags::default()));
    use_context_provider(|| Signal::new(QuickAddTasks::default()));
    use_context_provider(|| Signal::new(UserFilter::default()));
    use_context_provider(|| Signal::new(TagFilter::default()));
    let layout = ResponsiveLayout::from_window();
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
