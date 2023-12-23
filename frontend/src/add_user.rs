use crate::{model::Model, styles};
use dioxus::prelude::*;
use reqwest::Client;
use shared_models::{Color, UserData, UserId};

#[component]
fn AddUser(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let name = use_state(cx, String::default);
    cx.render(rsx! {
        div {
            class: "bg-gray-900 h-screen w-screen",
            form { class:"max-w-sm mx-auto",
                div {
                    class: "mb-5",
                    label {
                        r#for: "user_name",
                        class: styles::TEXT_INPUT_LABEL,
                        "Name"
                    },
                    input {
                        class: styles::TEXT_INPUT,
                        r#type: "text",
                        id: "user_name",
                        required: true,
                        value: "{name}",
                        oninput: |event| {
                            name.set(event.value.clone())
                        },
                    },
                }
                button {
                    class: styles::BUTTON,
                    r#type: "submit",
                    onclick: |_| {
                        create_user(
                            model.clone(),
                            UserData{
                                name: name.make_mut().drain(..).collect(),
                                color: Color::Black,
                            },
                        )
                    },
                    "Submit"
                }
            }
        }
    })
}

async fn create_user(model: UseSharedState<Model>, user_data: UserData) {
    if let Ok(user_id) = send_create_user_request(&model, &user_data).await {
        model.write().users.insert(user_id, user_data);
    }
}

async fn send_create_user_request(
    model: &UseSharedState<Model>,
    user_data: &UserData,
) -> Result<UserId, anyhow::Error> {
    let url = {
        let model = model.read();
        model
            .url
            .join(&format!("/api/boards/{}/users", model.board_name))?
    };
    Ok(Client::new()
        .post(url)
        .json(user_data)
        .send()
        .await?
        .json::<UserId>()
        .await?)
}
