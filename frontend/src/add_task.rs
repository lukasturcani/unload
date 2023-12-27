use crate::color_picker::ColorPicker;
use crate::requests;
use crate::route::Route;
use crate::{model::Model, styles};
use chrono::{offset::Local, NaiveDate, NaiveTime, TimeZone};
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use dioxus_router::prelude::Navigator;
use reqwest::Client;
use shared_models::{BoardName, TaskId, TaskSize, TaskStatus, UserData, UserId};

#[component]
pub fn AddTask(cx: Scope, board_name: BoardName) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let nav = use_navigator(cx);
    let title = use_state(cx, String::default);
    let description = use_state(cx, String::default);
    let size = use_state(cx, || TaskSize::Small);
    let status = use_state(cx, || TaskStatus::ToDo);
    let blocked_by = use_ref(cx, Vec::new);
    let blocks = use_ref(cx, Vec::new);
    let assigned_to = use_ref(cx, Vec::new);
    let due_date = use_state(cx, || None::<NaiveDate>);
    let due_time = use_state(cx, || NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    if &model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    use_future(cx, (), |_| requests::board(model.clone()));
    cx.render(rsx! {
        div {
            class: "bg-gray-900 min-h-screen min-w-screen",
            form { class:"max-w-sm mx-auto",
                div {
                    class: "mb-5",
                    label {
                        r#for: "task_title",
                        class: styles::TEXT_INPUT_LABEL,
                        "Title"
                    },
                    input {
                        class: styles::TEXT_INPUT,
                        r#type: "text",
                        id: "task_title",
                        value: "{title}",
                        oninput: |event| {
                            title.set(event.value.clone())
                        },
                    },
                }
                div {
                    class: "mb-5",
                    label {
                        r#for: "status",
                        class: styles::TEXT_INPUT_LABEL,
                        "Status"
                    },
                    div {
                        class: "flex flex-row w-full gap-x-2",
                        div {
                            class: "w-full flex items-center ps-2 border border-gray-200 rounded dark:border-gray-700",
                            input {
                                class: "w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600",
                                id: "status_to_do",
                                r#type: "radio",
                                value: "To do",
                                name: "status",
                                checked: true,
                                oninput: |_| status.set(TaskStatus::ToDo),
                            },
                            label {
                                r#for: "status_to_do",
                                class: "w-full py-4 ms-2 text-sm font-medium text-gray-900 dark:text-gray-300",
                                "To do",
                            },
                        },
                        div {
                            class: "w-full flex items-center ps-2 border border-gray-200 rounded dark:border-gray-700",
                            input {
                                class: "w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600",
                                id: "status_in_progress",
                                r#type: "radio",
                                value: "In progress",
                                name: "status",
                                oninput: |_| status.set(TaskStatus::InProgress),
                            },
                            label {
                                r#for: "status_in_progress",
                                class: "w-full py-4 ms-2 text-sm font-medium text-gray-900 dark:text-gray-300",
                                "In progress",
                            },
                        },
                        div {
                            class: "w-full flex items-center ps-2 border border-gray-200 rounded dark:border-gray-700",
                            input {
                                class: "w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600",
                                id: "status_done",
                                r#type: "radio",
                                value: "Done",
                                name: "status",
                                oninput: |_| status.set(TaskStatus::Done),
                            },
                            label {
                                r#for: "status_done",
                                class: "w-full py-4 ms-2 text-sm font-medium text-gray-900 dark:text-gray-300",
                                "Done",
                            },
                        },
                    },
                },
                div {
                    class: "mb-5",
                    label {
                        r#for: "size",
                        class: styles::TEXT_INPUT_LABEL,
                        "Size"
                    },
                    div {
                        class: "flex flex-row w-full gap-x-2",
                        div {
                            class: "w-full flex items-center ps-2 border border-gray-200 rounded dark:border-gray-700",
                            input {
                                class: "w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600",
                                id: "size_small",
                                r#type: "radio",
                                value: "Small",
                                name: "size",
                                checked: true,
                                oninput: |_| size.set(TaskSize::Small),
                            },
                            label {
                                r#for: "size_small",
                                class: "w-full py-4 ms-2 text-sm font-medium text-gray-900 dark:text-gray-300",
                                "Small",
                            },
                        },
                        div {
                            class: "w-full flex items-center ps-2 border border-gray-200 rounded dark:border-gray-700",
                            input {
                                class: "w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600",
                                id: "size_medium",
                                r#type: "radio",
                                value: "Medium",
                                name: "size",
                                oninput: |_| size.set(TaskSize::Medium),
                            },
                            label {
                                r#for: "size_medium",
                                class: "w-full py-4 ms-2 text-sm font-medium text-gray-900 dark:text-gray-300",
                                "Medium",
                            },
                        },
                        div {
                            class: "w-full flex items-center ps-2 border border-gray-200 rounded dark:border-gray-700",
                            input {
                                class: "w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600",
                                id: "size_large",
                                r#type: "radio",
                                value: "Large",
                                name: "size",
                                oninput: |_| size.set(TaskSize::Large),
                            },
                            label {
                                r#for: "size_large",
                                class: "w-full py-4 ms-2 text-sm font-medium text-gray-900 dark:text-gray-300",
                                "Large",
                            },
                        },

                    },
                },
                div {
                    class: "mb-5",
                    UserSearch{
                        id: "user_search",
                        on_select_user: |user_id| assigned_to.write().push(user_id),
                        on_remove_user: |user_id| assigned_to.write().retain(|&value| value != user_id),
                    },
                },
                div {
                    class: "mb-5",
                    label {
                        r#for: "task_description" ,
                        class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                        "Description"
                    },
                    textarea {
                        r#id: "task_description",
                        rows: "4",
                        class: "block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                        placeholder: "Give a description...",
                        oninput: |event| {
                            description.set(event.value.clone())
                        },
                    },
                },
                div {
                    class: "mb-5",
                    TaskSearch{
                        id: "blocked_by_search",
                        title: "Blocked by",
                        banned: blocks.read().clone(),
                        on_select_task: |task_id| blocked_by.write().push(task_id),
                        on_remove_task: |task_id| {
                            blocked_by
                            .write()
                            .retain(|&value| value != task_id)
                        },
                    },
                }
                div {
                    class: "mb-5",
                    TaskSearch{
                        id: "blocks_search",
                        title: "Blocks",
                        banned: blocked_by.read().clone(),
                        on_select_task: |task_id| blocks.write().push(task_id),
                        on_remove_task: |task_id| {
                            blocks
                            .write()
                            .retain(|&value| value != task_id)
                        },
                    },
                }
                div {
                    class: "mb-5",
                    label {
                        r#for: "task_due",
                        class: styles::TEXT_INPUT_LABEL,
                        "Due"
                    },
                    div {
                        id: "task_due",
                        input {
                            id: "task_due_date",
                            class: "mb-2 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                            r#type: "date",
                            oninput: |event| {
                                if event.value.is_empty() {
                                    due_date.set(None)
                                } else if let Ok(date) = NaiveDate::parse_from_str(&event.value, "%Y-%m-%d") {
                                    due_date.set(Some(date))
                                }
                            },
                        },
                        if due_date.is_some() {rsx!{
                            select {
                                id: "task_due_time",
                                class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                onchange: |event| {
                                    if let Ok(time) = NaiveTime::parse_from_str(&event.value, "%H:%M") {
                                        due_time.set(time)
                                    }
                                },
                                for hour in 0..24 {
                                    for minute in [0, 15, 30, 45] {
                                        rsx!{
                                            option {
                                                value: "{hour:02}:{minute:02}",
                                                "{hour:02}:{minute:02}"
                                            },
                                        }
                                    }
                                }
                            },
                        }}
                    },
                }
                button {
                    class: styles::BUTTON,
                    r#type: "submit",
                    prevent_default: "onclick",
                    onclick: |_| {
                        // TODO: once future issue is fixed change page
                        // as first thing
                        create_task(
                            model.clone(),
                            shared_models::TaskData {
                                title:
                                    title
                                    .make_mut()
                                    .drain(..)
                                    .collect::<String>()
                                    .trim()
                                    .to_string(),
                                description: description.make_mut().drain(..).collect(),
                                due: due_date.map(|date| {
                                    Local.from_local_datetime(&date.and_time(**due_time))
                                    .unwrap()
                                    .into()
                                }),
                                size: **size,
                                status: **status,
                                assignees: assigned_to.write().drain(..).collect(),
                                blocks: blocks.write().drain(..).collect(),
                                blocked_by: blocked_by.write().drain(..).collect(),
                            },
                            nav.clone(),
                        )
                    },
                    "Submit"
                }
            }
        },
    })
}

#[component]
fn TaskSearch<'a>(
    cx: Scope<'a>,
    id: &'a str,
    title: &'a str,
    banned: Vec<TaskId>,
    on_select_task: EventHandler<'a, TaskId>,
    on_remove_task: EventHandler<'a, TaskId>,
) -> Element<'a> {
    let model = use_shared_state::<Model>(cx).unwrap();
    let has_input_focus = use_state(cx, || false);
    let search_input = use_state(cx, String::default);
    let selected = use_ref(cx, Vec::<(TaskId, String)>::new);
    let dropdown_data: Option<Vec<_>> = has_input_focus.then(|| {
        let model = model.read();
        let selected = selected.read();
        if search_input.is_empty() {
            let mut data = model
                .tasks
                .iter()
                .filter(|(id1, _)| {
                    selected.iter().all(|(id2, _)| *id1 != id2)
                        && banned.iter().all(|id2| *id1 != id2)
                })
                .collect::<Vec<_>>();
            data.sort_by(|(_, a), (_, b)| a.updated.cmp(&b.updated));
            data.truncate(5);
            data.into_iter()
                .map(|(id, task)| (*id, task.title.clone()))
                .collect()
        } else {
            model
                .tasks
                .iter()
                .filter(|(id, task)| {
                    (task.title.contains(&**search_input)
                        || task.description.contains(&**search_input))
                        && selected.iter().all(|(selected_id, _)| *id != selected_id)
                })
                .map(|(id, task)| (*id, task.title.clone()))
                .collect()
        }
    });
    cx.render(rsx! {
        label {
            r#for: *id,
            class: styles::TEXT_INPUT_LABEL,
            title,
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
                class: "block w-full p-4 ps-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                placeholder: "Search",
                autocomplete: "off",
                onfocusin: |_| has_input_focus.set(true),
                onfocusout: |_| has_input_focus.set(false),
                oninput: |event| search_input.set(event.data.value.clone())
            },
        },
        if let Some(suggestions) = dropdown_data {
            if suggestions.is_empty() {rsx!{
                div {
                    class: "mt-2 z-10 bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-700 focus:border-blue-500",
                    ul {
                        class: "py-2 text-sm text-gray-700 dark:text-gray-200",
                        li {
                            class: "italic text-gray-500 dark:text-gray-400 block text-left w-full px-4 py-2",
                            prevent_default: "onmousedown",
                            onmousedown: |_| {},
                            "No matches"
                        },
                    }
                }
            }} else {rsx!{
                div {
                    class: "mt-2 z-10 bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-700 focus:border-blue-500",
                    ul {
                        class: "py-2 text-sm text-gray-700 dark:text-gray-200",
                        rsx!{
                            for task in suggestions {rsx!{
                                li {
                                    key: "{task.0}",
                                    button {
                                        r#type: "button",
                                        class: "block text-left w-full px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white focus:border-blue-500",
                                        prevent_default: "onmousedown",
                                        onmousedown: |_| {},
                                        onclick: move |_| {
                                            selected.write().push(task.clone());
                                            on_select_task.call(task.0);
                                        },
                                        task.1.clone(),
                                    }
                                },
                            }}
                        }
                    }
                }}
            }
        }
        div {
            class: "mt-2",
            for task in selected.read().iter().map(|x| x.clone()) {rsx!{
                span {
                    class: "inline-flex items-center px-2 py-1 me-2 mt-2 text-sm font-medium text-gray-800 bg-gray-100 rounded dark:bg-gray-700 dark:text-gray-300",
                    task.1.clone(),
                    button {
                        r#type: "button",
                        class: "inline-flex items-center p-1 ms-2 text-sm text-gray-400 bg-transparent rounded-sm hover:bg-gray-200 hover:text-gray-900 dark:hover:bg-gray-600 dark:hover:text-gray-300",
                        "aria-label": "Remove",
                        onclick: move |_| {
                            selected.write().retain(|this| this.0 != task.0);
                            on_remove_task.call(task.0);
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

#[component]
fn UserSearch<'a>(
    cx: Scope<'a>,
    id: &'a str,
    on_select_user: EventHandler<'a, UserId>,
    on_remove_user: EventHandler<'a, UserId>,
) -> Element<'a> {
    let model = use_shared_state::<Model>(cx).unwrap();
    let show_color_picker = use_state(cx, || false);
    let has_input_focus = use_state(cx, || false);
    let search_input = use_state(cx, String::default);
    let selected = use_ref(cx, Vec::<(UserId, String)>::new);
    if model.read().user_search_created_user.is_some() {
        if let Some(user) = model.write().user_search_created_user.take() {
            on_select_user.call(user.0);
            selected.write().push(user);
        }
    }
    let user_data = if **has_input_focus && !**show_color_picker {
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
                class: "block w-full p-4 ps-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
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
                                    // TODO: Add user needs to open a model for selecting color
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

async fn create_task(
    model: UseSharedState<Model>,
    task_data: shared_models::TaskData,
    nav: Navigator,
) {
    if task_data.title.is_empty() {
        log::info!("empty task title, doing nothing");
        return;
    }
    if let Ok(task_id) = send_create_task_request(&model, &task_data).await {
        log::info!("created task: {task_id}");
    }
    nav.push(Route::Board {
        board_name: model.read().board_name.clone(),
    });
}

async fn send_create_task_request(
    model: &UseSharedState<Model>,
    task_data: &shared_models::TaskData,
) -> Result<TaskId, anyhow::Error> {
    let url = {
        let model = model.read();
        model
            .url
            .join(&format!("/api/boards/{}/tasks", model.board_name))?
    };
    Ok(Client::new()
        .post(url)
        .json(task_data)
        .send()
        .await?
        .json::<TaskId>()
        .await?)
}

async fn create_user(model: UseSharedState<Model>, user_data: UserData) {
    if let Ok(user_data) = requests::create_user(model.clone(), user_data).await {
        requests::board(model.clone()).await;
        model.write().user_search_created_user = Some(user_data);
    }
}
