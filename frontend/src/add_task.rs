use crate::{model::Model, styles};
use chrono::{offset::Local, NaiveDate, NaiveTime, TimeZone};
use dioxus::prelude::*;
use reqwest::Client;
use shared_models::{TaskEntry, TaskId, TaskSize, TaskStatus, UserId};

#[component]
fn AddTask(cx: Scope) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let title = use_state(cx, String::default);
    let description = use_state(cx, String::default);
    let size = use_state(cx, || TaskSize::Small);
    let status = use_state(cx, || TaskStatus::ToDo);
    let blocked_by = use_ref(cx, Vec::new);
    let blocks = use_ref(cx, Vec::new);
    let assigned_to = use_ref(cx, Vec::new);
    let due_date = use_state(cx, || None::<NaiveDate>);
    let due_time = use_state(cx, || NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    cx.render(rsx! {
        div {
            class: "bg-gray-900 h-screen w-screen",
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
                        required: true,
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
                    onclick: |_| {
                        let create_task = create_task(
                            model.clone(),
                            shared_models::TaskData {
                                title: title.make_mut().drain(..).collect(),
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
                            }
                        );
                        create_task
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
    on_select_task: EventHandler<'a, TaskId>,
    on_remove_task: EventHandler<'a, TaskId>,
) -> Element<'a> {
    // TODO: multiple clicks on task do not add extra buttons
    let model = use_shared_state::<Model>(cx).unwrap();
    let has_input_focus = use_state(cx, || false);
    let search_input = use_state(cx, String::default);
    let selected = use_ref(cx, Vec::<(TaskId, String)>::new);
    // TODO: Can this holad a Vec<(TaskId, &String)>?
    let dropdown_data = has_input_focus.then(|| {
        if search_input.is_empty() {
            model.read().most_recent_titles()
        } else {
            model.read().find_titles(search_input)
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
                        class: "py-2 text-sm text-gray-700 dark:text-gray-200 focus:border-blue-500",
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
                        class: "py-2 text-sm text-gray-700 dark:text-gray-200 focus:border-blue-500",
                        rsx!{
                            for task in suggestions {rsx!{
                                li {
                                    class: "focus:border-blue-500",
                                    // TODO: check key have correct value
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
    // TODO: multiple clicks on user do not add extra buttons
    let model = use_shared_state::<Model>(cx).unwrap();
    let has_input_focus = use_state(cx, || false);
    let search_input = use_state(cx, String::default);
    let selected = use_ref(cx, Vec::<(UserId, String)>::new);
    // TODO: Can this holad a Vec<(TaskId, &String)>?
    let users: Vec<_> = model
        .read()
        .users
        .iter()
        .filter(|(_, user)| user.name.find(&**search_input).is_some())
        .map(|(id, user)| (*id, user.name.clone()))
        .collect();
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
                onfocusin: |_| has_input_focus.set(true),
                onfocusout: |_| has_input_focus.set(false),
                oninput: |event| search_input.set(event.data.value.clone())
            },
        },
        if **has_input_focus && (!users.is_empty() || !search_input.is_empty()) {rsx!{
            div {
                class: "mt-2 z-10 bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-700 focus:border-blue-500",
                ul {
                    class: "py-2 text-sm text-gray-700 dark:text-gray-200 focus:border-blue-500",
                    rsx!{
                        for user in users {rsx!{
                            li {
                                class: "focus:border-blue-500",
                                // TODO: check key have correct value
                                key: "{user.0}",
                                button {
                                    r#type: "button",
                                    class: "block text-left w-full px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white focus:border-blue-500",
                                    prevent_default: "onmousedown",
                                    onmousedown: |_| {},
                                    onclick: move |_| {
                                        selected.write().push(user.clone());
                                        on_select_user.call(user.0);
                                    },
                                    user.1.clone(),
                                }
                            },
                        }}
                    }
                    if !search_input.is_empty() {rsx!{
                        li {
                            key: "add user",
                            class: "focus:border-blue-500",
                            button {
                                r#type: "button",
                                // TODO: Change color of Add user text
                                // TODO: Add user needs to open a model for selecting color
                                class: "block text-left w-full px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white focus:border-blue-500",
                                prevent_default: "onmousedown",
                                onmousedown: |_| {},
                                onclick: move |_| {},
                                "Add user"
                            }

                        },
                    }}
                }
            }
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

async fn create_task(model: UseSharedState<Model>, task_data: shared_models::TaskData) {
    if let Ok(task_entry) = send_create_task_request(&model, &task_data).await {
        let mut model = model.write();
        match task_entry.status {
            TaskStatus::ToDo => model.to_do.push(task_entry.id),
            TaskStatus::InProgress => model.in_progress.push(task_entry.id),
            TaskStatus::Done => model.done.push(task_entry.id),
        }
        model.tasks.insert(task_entry.id, task_entry.into());
    }
}

async fn send_create_task_request(
    model: &UseSharedState<Model>,
    task_data: &shared_models::TaskData,
) -> Result<TaskEntry, anyhow::Error> {
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
        .json::<TaskEntry>()
        .await?)
}