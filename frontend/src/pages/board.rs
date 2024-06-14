use dioxus::prelude::*;

use crate::components::one_column_board::OneColumnBoard;
use crate::model::{Board, QuickAddTasks, TagFilter, Tags, Tasks, UserFilter, Users};
use crate::pages::three_column_board::ThreeColumnBoard;
use crate::responsive_layout::ResponsiveLayout;

use shared_models::BoardName;

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
