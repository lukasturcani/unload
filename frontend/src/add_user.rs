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
            class: "flex flex-col bg-gray-900 min-h-screen min-w-screen",
            div {
                class: "grow p-4 max-w-sm mx-auto",
                form {
                    class: "flex flex-col gap-5",
                    div {
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
                        class: "flex justify-center",
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
            div {
                class: styles::BOTTOM_BAR,
                button {
                    r#type: "button" ,
                    class: styles::BOTTOM_BAR_BUTTON,
                    onclick: |_| {
                        nav.go_back();
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "
                            w-6 h-6 text-gray-400
                            group-active:text-blue-500
                            sm:group-hover:text-blue-500
                        ",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M15.75 19.5 8.25 12l7.5-7.5",
                        }
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
        Ok((user_id, _)) => {
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
