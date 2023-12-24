use crate::{model::Model, route::Route, styles};
use dioxus::prelude::*;
use dioxus_router::{hooks::use_navigator, prelude::Navigator};
use reqwest::Client;
use shared_models::{BoardName, Color, UserData, UserId};

#[component]
pub fn AddUser(cx: Scope, board_name: BoardName) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let nav = use_navigator(cx);
    let name = use_state(cx, String::default);
    if &model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    cx.render(rsx! {
        div {
            class: "bg-gray-900 h-screen w-screen",
            form {
                class:"max-w-sm mx-auto",
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
                        // TODO: once future issue is fixed change page
                        // as first thing
                        create_user(
                            model.clone(),
                            UserData{
                                name: name.make_mut().drain(..).collect(),
                                color: Color::Black,
                            },
                            nav.clone(),
                        )
                    },
                    "Submit"
                }
            }
        }
    })
}

async fn create_user(model: UseSharedState<Model>, user_data: UserData, nav: Navigator) {
    if user_data.name.is_empty() {
        return;
    }
    log::info!("sending create user request");
    if let Ok(user_id) = send_create_user_request(&model, &user_data).await {
        log::info!("created user");
        model.write().users.insert(user_id, user_data);
    } else {
        log::info!("failed to create user");
    }
    nav.push(Route::Board {
        board_name: model.read().board_name.clone(),
    });
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
