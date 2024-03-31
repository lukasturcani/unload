use crate::model::Model;
use crate::route::Route;
use crate::styles;
use dioxus::prelude::*;
use reqwest::Client;
use shared_models::BoardName;

const TEXT_INPUT: &str = "
    border text-sm rounded-lg
    block p-2.5 bg-gray-700
    border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500
    focus:border-blue-500
";

#[component]
pub fn JoinBoard() -> Element {
    let mut model = use_context::<Signal<Model>>();
    let nav = use_navigator();
    rsx! {
        div{
            class: "bg-gray-900 h-dvh w-screen",
            form {
                class:"max-w-sm mx-auto",
                div {
                    class: "w-full inline-flex items-center justify-center gap-3 py-5",
                    input {
                        class: TEXT_INPUT,
                        r#type: "text",
                        required: true,
                        placeholder: "Board Name",
                        value: "{model.read().board_name}",
                        oninput: move |event| {
                            model.write().board_name = event.data.value().into()
                        },
                    },
                    button {
                        class: styles::BUTTON,
                        r#type: "submit",
                        onclick: move |_| {
                            nav.push(Route::Board {
                                board_name: model.read().board_name.clone(),
                            });
                        },
                        "Join"
                    },
                },
            },
            div {
                class: "inline-flex items-center justify-center w-full",
                hr {
                    class: "w-64 h-px my-8 border-0 bg-gray-700",
                },
                span {
                    class: "absolute px-3 font-medium -translate-x-1/2 left-1/2 text-white bg-gray-900",
                    "or"
                },
            },
            div {
                class: "inline-flex items-center justify-center w-full py-5",
                button {
                    class: styles::BUTTON,
                    onclick: move |_| create_board(model, nav),
                    "Create New Board",
                },
            },
        }
    }
}

async fn create_board(model: Signal<Model>, nav: Navigator) {
    if let Ok(board_name) = send_create_board_request(model).await {
        nav.push(Route::Board { board_name });
    }
}

async fn send_create_board_request(model: Signal<Model>) -> Result<BoardName, anyhow::Error> {
    let request = {
        let model = model.read();
        let client = Client::new();
        let url = model.url.join("/api/boards")?;
        client.post(url).json(&model.board_name)
    };
    Ok(request.send().await?.json::<BoardName>().await?)
}
