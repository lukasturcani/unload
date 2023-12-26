use crate::{color_picker::ColorPicker, model::Model, requests, route::Route, styles};
use dioxus::prelude::*;
use dioxus_router::{hooks::use_navigator, prelude::Navigator};
use shared_models::{BoardName, Color, UserData};

#[component]
pub fn AddUser(cx: Scope, board_name: BoardName) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let nav = use_navigator(cx);
    let name = use_state(cx, String::default);
    let default_color = Color::Black;
    let color = use_state(cx, || default_color);
    if &model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    cx.render(rsx! {
        div {
            class: "bg-gray-900 min-h-screen min-w-screen",
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
                        value: "{name}",
                        oninput: |event| name.set(event.value.clone()),
                    },
                }
                div {
                    class: "mb-5 flex justify-center",
                    ColorPicker{
                        default_color: default_color,
                        on_pick_color: |c| color.set(c),
                    },
                }
                div {
                    class: "flex justify-center",
                    button {
                        class: styles::BUTTON,
                        r#type: "submit",
                        prevent_default: "onclick",
                        onclick: move |_| {
                            // TODO: once future issue is fixed change page
                            // as first thing
                            let user_color = **color;
                            color.set(default_color);
                            create_user(
                                model.clone(),
                                UserData{
                                    name:
                                        name
                                        .make_mut()
                                        .drain(..)
                                        .collect(),
                                    color: user_color,
                                },
                                nav.clone(),
                            )
                        },
                        "Submit"
                    }
                }
            }
        }
    })
}

async fn create_user(model: UseSharedState<Model>, user_data: UserData, nav: Navigator) {
    if user_data.name.is_empty() {
        log::info!("empty user name, doing nothing");
        return;
    }
    log::info!("sending create user request");
    match requests::create_user(model.clone(), user_data).await {
        Ok(user_id) => {
            log::info!("created user: {user_id}");
        }
        Err(e) => {
            log::info!("failed to create user: {}", e);
        }
    }
    nav.push(Route::Board {
        board_name: model.read().board_name.clone(),
    });
}
