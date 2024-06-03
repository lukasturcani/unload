use dioxus::prelude::*;
use shared_models::BoardName;

use crate::{
    components::nav::NavBar,
    requests::{self, BoardSignals},
    themes::Theme,
};

#[component]
pub fn ThreeColumnBoard(board_name: BoardName) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        text-white stroke-white
        {}
    ",
        theme.bg_color_1
    );
    let dense = use_signal(|| false);
    let mut board_signals = BoardSignals::default();
    if board_signals.model.read().board_name != board_name {
        board_signals.model.write().board_name = board_name.clone();
        board_signals.board.write().board_name = board_name.clone();
    }
    use_future(move || requests::board(board_signals));
    rsx! {
        div {
            class: "flex flex-col h-dvh w-screen {style}",
            div {
                class: "grow flex flex-col gap-2 overflow-y-auto p-4 pb-2",
                div {
                    class: "grow w-full h-full overflow-y-auto",
                    div {
                        class: "w-full h-full grid grid-cols-3 gap-2 overflow-y-auto",
                        if board_signals.model.read().dense_view {
                            DenseToDoColumn {}
                            DenseInProgressColumn {}
                            DenseDoneColumn {}
                        } else {
                            ToDoColumn {}
                            InProgressColumn {}
                            DoneColumn {}
                        }
                    },
                }
                FilterBar {}
            }
            NavBar { board_name }
        }
    }
}

#[component]
fn Column(status: TaskStatus) -> Element {
    rsx! {
        div {

        }
    }
}
