use dioxus::prelude::*;

const TEXT_INPUT: &str = "
    border text-sm rounded-lg
    block p-2.5 bg-gray-700
    border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500
    focus:border-blue-500
";

#[component]
pub fn JoinBoard() -> Element {
    rsx! {
        div{ }
    }
}

// async fn create_board(url: Signal<UnloadUrl>, board: Signal<Board>, nav: Navigator) {
//     if let Ok(board_name) = send_create_board_request(url, board).await {
//         nav.push(Route::Board { board_name });
//     }
// }

// async fn send_create_board_request(
//     url: Signal<UnloadUrl>,
//     board: Signal<Board>,
// ) -> Result<BoardName, anyhow::Error> {
//     let request = {
//         let url = &url.read().0;
//         let board = board.read();
//         let client = Client::new();
//         let url = url.join("/api/boards")?;
//         client.post(url).json(&board.board_name)
//     };
//     Ok(request.send().await?.json::<BoardName>().await?)
// }
