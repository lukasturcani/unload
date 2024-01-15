use dioxus::prelude::*;
use shared_models::{UserData, UserId};

use crate::{color_picker::ColorPicker, model::Model, requests, styles};

#[component]
pub fn UserSearch<'a>(
    cx: Scope<'a>,
    id: &'a str,
    on_select_user: EventHandler<'a, UserId>,
    on_remove_user: EventHandler<'a, UserId>,
    initial_users: Option<Vec<UserId>>,
    always_show_suggestions: Option<bool>,
) -> Element<'a> {
    let always_show_suggestions = always_show_suggestions.unwrap_or(false);
    let model = use_shared_state::<Model>(cx).unwrap();
    let show_color_picker = use_state(cx, || false);
    let has_input_focus = use_state(cx, || false);
    let search_input = use_state(cx, String::default);
    let selected = {
        let model = model.read();
        use_ref(cx, || {
            if let Some(users) = initial_users {
                users
                    .iter()
                    .map(|id| (*id, model.users[id].name.clone()))
                    .collect()
            } else {
                Vec::new()
            }
        })
    };
    if model.read().user_search_created_user.is_some() {
        if let Some(user) = model.write().user_search_created_user.take() {
            on_select_user.call(user.0);
            selected.write().push(user);
        }
    }
    let user_data = if (**has_input_focus || always_show_suggestions) && !**show_color_picker {
        let model = model.read();
        let selected = selected.read();
        let users: Vec<_> = model
            .users
            .iter()
            .filter(|(id, user)| {
                user.name.contains(&**search_input)
                    && selected.iter().all(|(selected_id, _)| selected_id != *id)
            })
            .map(|(id, user)| (*id, user.name.clone()))
            .collect();
        let show_add_user_button = !search_input.is_empty()
            && model
                .users
                .iter()
                .all(|(_, user)| user.name != search_input.trim());
        Some((users, show_add_user_button))
    } else {
        None
    };
    cx.render(rsx! {
        label {
            r#for: *id,
            class: styles::TEXT_INPUT_LABEL,
            "Assigned to"
        },
        div {
            class: "relative",
            div {
                class: "absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none",
                svg {
                    class: "w-4 h-4 text-gray-500 dark:text-gray-400",
                    "aria-hidden": "true",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none" ,
                    "viewBox": "0 0 20 20",
                    path {
                        d: "m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z",
                        stroke: "currentColor",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        "stroke-width", "2",
                    },
                },
            },
            input {
                r#type: "search",
                id: *id,
                class: "{styles::TEXT_INPUT} ps-10",
                placeholder: "Search",
                autocomplete: "off",
                value: "{search_input}",
                onfocusin: |_| has_input_focus.set(true),
                onfocusout: |_| has_input_focus.set(false),
                oninput: |event| search_input.set(event.data.value.clone())
            },
        },
        if **show_color_picker {rsx!{
            div {
                class: "mt-2 z-10 bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-700 p-4",
                ColorPicker {
                    on_pick_color: |color| {
                        show_color_picker.set(false);
                        cx.spawn(create_user(
                            model.clone(),
                            UserData {
                                name: search_input.make_mut().drain(..).collect(),
                                color
                            },
                        ));
                    },
                }
            }
        }}
        if let Some((users, show_add_user_button)) = user_data {rsx!{
            if !users.is_empty() || show_add_user_button {rsx!{
                div {
                    class: "mt-2 z-10 bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-700",
                    ul {
                        class: "py-2 text-sm text-gray-700 dark:text-gray-200",
                        rsx!{
                            for user in users {rsx!{
                                li {
                                    key: "{user.0}",
                                    button {
                                        r#type: "button",
                                        class: "block text-left w-full px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white focus:border-blue-500",
                                        prevent_default: "onmousedown",
                                        onmousedown: |_| {},
                                        onclick: move |_| {
                                            search_input.set(String::new());
                                            selected.write().push(user.clone());
                                            on_select_user.call(user.0);
                                        },
                                        user.1.clone(),
                                    }
                                },
                            }}
                        }
                        if show_add_user_button {rsx!{
                            li {
                                key: "add user",
                                button {
                                    r#type: "button",
                                    class: "block text-left w-full px-4 py-2
                                        hover:bg-gray-100 dark:hover:bg-gray-600
                                        font-medium text-blue-600 dark:text-blue-500 hover:underline",
                                    prevent_default: "onmousedown",
                                    onmousedown: |_| {},
                                    onclick: |_| show_color_picker.set(true),
                                    "Add User"
                                }
                            },
                        }}
                    }
                }
            }}
            else {rsx!{
                div {
                    class: "mt-2 z-10 bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-700 focus:border-blue-500",
                    ul {
                        class: "py-2 text-sm text-gray-700 dark:text-gray-200 focus:border-blue-500",
                        li {
                            class: "italic text-gray-500 dark:text-gray-400 block text-left w-full px-4 py-2",
                            prevent_default: "onmousedown",
                            onmousedown: |_| {},
                            "No matches"
                        },
                    }
                }
            }}
        }}
        div {
            class: "mt-2",
            for user in selected.read().iter().map(|x| x.clone()) {rsx!{
                span {
                    class: "inline-flex items-center px-2 py-1 me-2 mt-2 text-sm font-medium text-gray-800 bg-gray-100 rounded dark:bg-gray-700 dark:text-gray-300",
                    user.1.clone(),
                    button {
                        r#type: "button",
                        class: "inline-flex items-center p-1 ms-2 text-sm text-gray-400 bg-transparent rounded-sm hover:bg-gray-200 hover:text-gray-900 dark:hover:bg-gray-600 dark:hover:text-gray-300",
                        "aria-label": "Remove",
                        onclick: move |_| {
                            selected.write().retain(|this| this.0 != user.0);
                            on_remove_user.call(user.0);
                        },
                        svg {
                            class: "w-2 h-2",
                            "aria-hidden": "true",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 14 14",
                            path {
                                stroke: "currentColor",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                "stroke-width": "2",
                                d: "m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6",
                            },
                        },
                        span {
                            class: "sr-only",
                            "Remove badge",
                        },
                    },
                },
            }},
        },
    })
}

async fn create_user(model: UseSharedState<Model>, user_data: UserData) {
    if let Ok(user_data) = requests::create_user(model.clone(), user_data).await {
        requests::board(model.clone()).await;
        model.write().user_search_created_user = Some(user_data);
    }
}
