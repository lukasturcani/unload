use crate::model::Model;
use crate::styles;
use dioxus::prelude::*;
use reqwest::Client;
use shared_models::BoardName;

const TEXT_INPUT: &str = "
    bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg
    focus:ring-blue-500 focus:border-blue-500 block p-2.5 dark:bg-gray-700
    dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500
    dark:focus:border-blue-500
";

#[component]
pub fn JoinBoard(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    cx.render(rsx! {
        div{
            class: "bg-gray-900 h-screen w-screen",
            form {
                class:"max-w-sm mx-auto",
                div {
                    class: "w-full inline-flex items-center justify-center gap-4 py-5",
                    input {
                        class: TEXT_INPUT,
                        r#type: "text",
                        required: true,
                        placeholder: "Board Name",
                        value: "{model.read().board_name}",
                        oninput: |event| {
                            model.write().board_name = event.value.clone().into()
                        },
                    },
                    button {
                        class: styles::BUTTON,
                        r#type: "submit",
                        "Join"
                    },
                },
            },
            div {
                class: "inline-flex items-center justify-center w-full",
                hr {
                    class: "w-64 h-px my-8 bg-gray-200 border-0 dark:bg-gray-700",
                },
                span {
                    class: "absolute px-3 font-medium text-gray-900 -translate-x-1/2 bg-white left-1/2 dark:text-white dark:bg-gray-900",
                    "or"
                },
            },
            div {
                class: "inline-flex items-center justify-center w-full py-5",
                button {
                    class: styles::BUTTON,
                    onclick: |_| create_board(model.clone()),
                    "Create New Board",
                },
            },
        }
    })
}

async fn create_board(model: UseSharedState<Model>) {
    if let Ok(board_name) = send_create_board_request(&model).await {
        model.write().board_name = board_name;
    }
}

async fn send_create_board_request(
    model: &UseSharedState<Model>,
) -> Result<BoardName, anyhow::Error> {
    let request = {
        let model = model.read();
        let client = Client::new();
        let url = model.url.join("/api/boards")?;
        client.post(url).json(&model.board_name)
    };
    Ok(request.send().await?.json::<BoardName>().await?)
}
