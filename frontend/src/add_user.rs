use crate::{
    color_picker::SelectingColorPicker, model::Model, requests,
    responsive_layout::ResponsiveLayout, route::Route, styles,
};
use dioxus::prelude::*;
use shared_models::{BoardName, Color, UserData};

#[component]
pub fn AddUser(board_name: BoardName) -> Element {
    let mut model = use_context::<Signal<Model>>();
    let nav = use_navigator();
    let name = use_signal(String::new);
    let default_color = Color::Black;

    let color_signal = use_signal(|| default_color);
    let color = color_signal();

    let layout = ResponsiveLayout::from_window();

    let has_focus_signal = use_signal(|| false);
    let has_focus = has_focus_signal();

    if model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    rsx! {
        div {
            class: "
                h-dvh w-screen
                bg-gray-900
                flex flex-col
            ",
            div {
                class: "
                    grow w-full p-4 overflow-y-scroll
                    flex flex-col items-center
                ",
                form {
                    class: "flex flex-col gap-5 items-center w-full max-w-lg",
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
                            oninput: move |event| name.set(event.value()),
                            onfocusin: move |_| has_focus_signal.set(true),
                            onfocusout: move |_| has_focus_signal.set(false),
                        },
                    }
                    div {
                        class: "flex justify-center",
                        SelectingColorPicker{
                            default_color: default_color,
                            on_pick_color: move |c| color_signal.set(c),
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
                                color_signal.set(default_color);
                                create_user(
                                    model,
                                    UserData{
                                        name:
                                            name
                                            .write()
                                            .drain(..)
                                            .collect(),
                                        color,
                                    },
                                    nav,
                                )
                            },
                            "Submit"
                        }
                    }
                }
            }
            if (layout == ResponsiveLayout::Wide) || (!has_focus && layout == ResponsiveLayout::Narrow) {
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
        }
    }
}

async fn create_user(model: Signal<Model>, user_data: UserData, nav: Navigator) {
    if user_data.name.is_empty() {
        log::info!("empty user name, doing nothing");
        return;
    }
    log::info!("sending create user request");
    match requests::create_user(model, user_data).await {
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
