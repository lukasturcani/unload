use dioxus::prelude::*;
use shared_models::BoardName;

#[component]
pub fn Board(board_name: BoardName) -> Element {
    todo!()
    // let layout = ResponsiveLayout::from_window();
    // let eval = use_eval(cx);
    // eval(&format!(r#"document.title = "{board_name}";"#)).unwrap();
    // rsx! {
    //     match layout {
    //         ResponsiveLayout::Narrow => rsx! {
    //             OneColumnBoard {
    //                 board_name: board_name.clone(),
    //             }
    //         },
    //         ResponsiveLayout::Wide => rsx! {
    //             ThreeColumnBoard {
    //                 board_name: board_name.clone(),
    //             }
    //         }
    //     }
    // }
}
