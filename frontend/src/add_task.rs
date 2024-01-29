use std::collections::HashSet;

use crate::color_picker;
use crate::color_picker::ColorPicker;
use crate::requests;
use crate::responsive_layout::ResponsiveLayout;
use crate::route::Route;
use crate::{model::Model, styles};
use chrono::{offset::Local, NaiveDate, NaiveTime, TimeZone};
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use dioxus_router::prelude::Navigator;
use itertools::Itertools;
use reqwest::Client;
use shared_models::{BoardName, TagData, TagId, TaskId, TaskSize, TaskStatus, UserData, UserId};

#[component]
pub fn AddTask(cx: Scope, board_name: BoardName) -> Element {
    cx.render(rsx! {
        AddTaskImpl {
            board_name: board_name.clone(),
            default_status: TaskStatus::ToDo,
        }
    })
}

#[component]
pub fn AddToDoTask(cx: Scope, board_name: BoardName) -> Element {
    cx.render(rsx! {
        AddTaskImpl {
            board_name: board_name.clone(),
            default_status: TaskStatus::ToDo,
        }
    })
}

#[component]
pub fn AddInProgressTask(cx: Scope, board_name: BoardName) -> Element {
    cx.render(rsx! {
        AddTaskImpl {
            board_name: board_name.clone(),
            default_status: TaskStatus::InProgress,
        }
    })
}

#[component]
pub fn AddDoneTask(cx: Scope, board_name: BoardName) -> Element {
    cx.render(rsx! {
        AddTaskImpl {
            board_name: board_name.clone(),
            default_status: TaskStatus::Done,
        }
    })
}

const RADIO_DIV: &str = "
    w-full flex items-center ps-2 border rounded border-gray-700
";
const RADIO_INPUT: &str = "
    w-4 h-4 text-blue-600 focus:ring-blue-600 ring-offset-gray-800 focus:ring-2 bg-gray-700 border-gray-600
";
const RADIO_LABEL: &str = "
    w-full py-4 ms-2 text-sm font-medium text-gray-300
";

#[component]
fn AddTaskImpl(cx: Scope, board_name: BoardName, default_status: TaskStatus) -> Element {
    let model = use_shared_state::<Model>(cx).unwrap();
    let nav = use_navigator(cx);
    let title = use_state(cx, String::default);
    let tags = use_ref(cx, Vec::new);
    let description = use_state(cx, String::default);
    let size = use_state(cx, || TaskSize::Small);
    let status = use_state(cx, || *default_status);
    let blocked_by = use_ref(cx, Vec::new);
    let blocks = use_ref(cx, Vec::new);
    let assigned_to = use_ref(cx, Vec::new);
    let due_date = use_state(cx, || None::<NaiveDate>);
    let due_time = use_state(cx, || NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    let layout = ResponsiveLayout::from_window();
    let has_focus = use_state(cx, || false);
    if &model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    use_future(cx, (), |_| requests::board(model.clone()));
    cx.render(rsx! {
        div {
            class: "
                h-screen w-screen
                bg-gray-900
                flex flex-col
            ",
            div {
                class: "
                    grow w-full p-4 overflow-y-scroll
                    flex flex-col items-center
                ",
                form {
                    class: "flex flex-col gap-5 items-left w-full max-w-lg",
                    onfocusin: |_| has_focus.set(true),
                    onfocusout: |_| has_focus.set(false),
                    div {
                        class: "flex flex-col gap-1",
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
                    TagSearch {
                        on_select_tag: |tag_id| tags.write().push(tag_id),
                        on_remove_tag: |tag_id| tags.write().retain(|&value| value != tag_id),
                    }
                    div {
                        class: "flex flex-col gap-1",
                        label {
                            r#for: "status",
                            class: styles::TEXT_INPUT_LABEL,
                            "Status"
                        },
                        div {
                            class: "flex flex-row w-full gap-x-2",
                            div {
                                class: RADIO_DIV,
                                input {
                                    class: RADIO_INPUT,
                                    id: "status_to_do",
                                    r#type: "radio",
                                    value: "To do",
                                    name: "status",
                                    checked: *status == TaskStatus::ToDo,
                                    oninput: |_| status.set(TaskStatus::ToDo),
                                },
                                label {
                                    r#for: "status_to_do",
                                    class: RADIO_LABEL,
                                    "To do",
                                },
                            },
                            div {
                                class: RADIO_DIV,
                                input {
                                    class: RADIO_INPUT,
                                    id: "status_in_progress",
                                    r#type: "radio",
                                    value: "In progress",
                                    name: "status",
                                    checked: *status == TaskStatus::InProgress,
                                    oninput: |_| status.set(TaskStatus::InProgress),
                                },
                                label {
                                    r#for: "status_in_progress",
                                    class: RADIO_LABEL,
                                    "In progress",
                                },
                            },
                            div {
                                class: RADIO_DIV,
                                input {
                                    class: RADIO_INPUT,
                                    id: "status_done",
                                    r#type: "radio",
                                    value: "Done",
                                    name: "status",
                                    checked: *status == TaskStatus::Done,
                                    oninput: |_| status.set(TaskStatus::Done),
                                },
                                label {
                                    r#for: "status_done",
                                    class: RADIO_LABEL,
                                    "Done",
                                },
                            },
                        },
                    },
                    div {
                        class: "flex flex-col gap-1",
                        label {
                            r#for: "size",
                            class: styles::TEXT_INPUT_LABEL,
                            "Size"
                        },
                        div {
                            class: "flex flex-row w-full gap-x-2",
                            div {
                                class: RADIO_DIV,
                                input {
                                    class: RADIO_INPUT,
                                    id: "size_small",
                                    r#type: "radio",
                                    value: "Small",
                                    name: "size",
                                    checked: true,
                                    oninput: |_| size.set(TaskSize::Small),
                                },
                                label {
                                    r#for: "size_small",
                                    class: RADIO_LABEL,
                                    "Small",
                                },
                            },
                            div {
                                class: RADIO_DIV,
                                input {
                                    class: RADIO_INPUT,
                                    id: "size_medium",
                                    r#type: "radio",
                                    value: "Medium",
                                    name: "size",
                                    oninput: |_| size.set(TaskSize::Medium),
                                },
                                label {
                                    r#for: "size_medium",
                                    class: RADIO_LABEL,
                                    "Medium",
                                },
                            },
                            div {
                                class: RADIO_DIV,
                                input {
                                    class: RADIO_INPUT,
                                    id: "size_large",
                                    r#type: "radio",
                                    value: "Large",
                                    name: "size",
                                    oninput: |_| size.set(TaskSize::Large),
                                },
                                label {
                                    r#for: "size_large",
                                    class: RADIO_LABEL,
                                    "Large",
                                },
                            },

                        },
                    },
                    UserSearch {
                        on_select_user: |user_id| assigned_to.write().push(user_id),
                        on_remove_user: |user_id| assigned_to.write().retain(|&value| value != user_id),
                    }
                    div {
                        class: "flex flex-col gap-1",
                        label {
                            r#for: "task_description" ,
                            class: "text-sm font-medium text-white",
                            "Description"
                        },
                        textarea {
                            r#id: "task_description",
                            rows: "4",
                            class: "
                                p-2.5 w-full text-base rounded-lg border bg-gray-700 border-gray-600
                                placeholder-gray-400 text-white
                                focus:ring-blue-500 focus:border-blue-500
                            ",
                            placeholder: "Give a description...",
                            oninput: |event| {
                                description.set(event.value.clone())
                            },
                        },
                    },
                    div {
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
                            on_search_focus_in: |_| has_focus.set(true),
                            on_search_focus_out: |_| has_focus.set(false),
                        },
                    }
                    NewTaskSearch {
                        title: "Blocks",
                        on_select_task: |task_id| blocks.write().push(task_id),
                        on_remove_task: |task_id| {
                            blocks
                            .write()
                            .retain(|&value| value != task_id)
                        },
                        }
                    div {
                        class: "flex flex-col gap-1",
                        label {
                            r#for: "task_due",
                            class: styles::TEXT_INPUT_LABEL,
                            "Due"
                        }
                        div {
                            id: "task_due",
                            input {
                                id: "task_due_date",
                                class: "mb-2 border text-sm rounded-lg block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500",
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
                                    class: "border text-sm rounded-lg block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500",
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
                        }
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
                                    tags: tags.write().drain(..).collect(),
                                },
                                nav.clone(),
                            )
                        },
                        "Submit"
                    }
                }
            }
            if (layout == ResponsiveLayout::Wide) || (!has_focus && layout == ResponsiveLayout::Narrow) {rsx! {
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
            }}
        },
    })
}

#[component]
fn NewTaskSearch<'a>(
    cx: Scope<'a>,
    title: &'static str,
    on_select_task: EventHandler<'a, TaskId>,
    on_remove_task: EventHandler<'a, TaskId>,
) -> Element<'a> {
    let model = use_shared_state::<Model>(cx).unwrap();
    let selected = use_ref(cx, HashSet::new);
    let search_input = use_state(cx, String::default);
    let read_selected = selected.read();
    let read_model = model.read();
    cx.render(rsx! {
        div {
            class: "flex flex-col gap-1",
            p {
                class: styles::TEXT_INPUT_LABEL,
                title
            },
            input {
                r#type: "text",
                class: styles::TEXT_INPUT,
                placeholder: "Search",
                autocomplete: "off",
                value: "{search_input}",
                oninput: |event| search_input.set(event.data.value.clone())
            }
            div {
                class: "flex flex-row gap-2 flex-wrap",
                for (task_id, task) in selected
                    .read()
                    .iter()
                    .map(|task_id| (task_id, &read_model.tasks[task_id]))
                {rsx!{
                    span {
                        class: "
                            text-sm font-medium text-white
                            px-2.5 py-0.5 rounded
                            cursor-pointer
                            border-2 border-white
                            flex flex-row gap-2
                        ",
                        "{&task.title}"
                        button {
                            r#type: "button",
                            class: "{styles::TAG_BADGE_BUTTON}",
                            onclick: {
                                let tag_id = *task_id;
                                move |_| {
                                    selected.write().retain(|&this| this != tag_id);
                                    on_remove_task.call(tag_id);
                                }
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
                                    d: "m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
                                }
                            }
                        }
                    }
                }}
            }
            ul {
                class: "
                    text-sm text-gray-200 rounded-lg
                    border border-gray-700 divide-y divide-gray-700
                    h-64 overflow-y-scroll
                ",
                rsx!{
                    for (task_id, task) in read_model
                        .tasks
                        .iter()
                        .filter(|(id, task)| {
                            !read_selected.contains(id)
                            && task.title.to_lowercase().contains(&search_input.to_lowercase())
                        })
                        .sorted_by_key(|(_, task)| task.title.to_lowercase())
                    {rsx!{
                        li {
                            key: "{task_id}",
                            button {
                                r#type: "button",
                                class: "
                                    text-left w-full px-4 py-2
                                    hover:bg-gray-800 hover:text-white
                                ",
                                prevent_default: "onmousedown",
                                onmousedown: |_| {},
                                onclick: {
                                    let task_id = *task_id;
                                    move |event| {
                                        event.stop_propagation();
                                        selected.write().insert(task_id);
                                        on_select_task.call(task_id);
                                    }
                                },
                                task.title.clone(),
                            }
                        },
                    }}
                }
            }
        }
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
    on_search_focus_in: EventHandler<'a>,
    on_search_focus_out: EventHandler<'a>,
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
                    class: "w-4 h-4 text-gray-400",
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
                onfocusin: |_| {
                    on_search_focus_in.call(());
                    has_input_focus.set(true);
                },
                onfocusout: |_| {
                    on_search_focus_out.call(());
                    has_input_focus.set(true);
                    has_input_focus.set(false);
                },
                oninput: |event| search_input.set(event.data.value.clone())
            },
        },
        if let Some(suggestions) = dropdown_data {
            if suggestions.is_empty() {rsx!{
                div {
                    class: "mt-2 z-10 divide-y divide-gray-100 rounded-lg shadow bg-gray-700 focus:border-blue-500",
                    ul {
                        class: "py-2 text-sm text-gray-200",
                        li {
                            class: "italic text-gray-400 block text-left w-full px-4 py-2",
                            "No matches"
                        },
                    }
                }
            }} else {rsx!{
                div {
                    class: "mt-2 z-10 divide-y divide-gray-100 rounded-lg shadow bg-gray-700 focus:border-blue-500",
                    ul {
                        class: "py-2 text-sm text-gray-200",
                        rsx!{
                            for task in suggestions {rsx!{
                                li {
                                    key: "{task.0}",
                                    button {
                                        r#type: "button",
                                        class: "block text-left w-full px-4 py-2 hover:bg-gray-600 hover:text-white focus:border-blue-500",
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
                    class: "inline-flex items-center px-2 py-1 me-2 mt-2 text-sm font-medium rounded bg-gray-700 text-gray-300",
                    task.1.clone(),
                    button {
                        r#type: "button",
                        class: "inline-flex items-center p-1 ms-2 text-sm text-gray-400 bg-transparent rounded-sm hover:bg-gray-600 hover:text-gray-300",
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
fn TagSearch<'a>(
    cx: Scope,
    on_select_tag: EventHandler<'a, TagId>,
    on_remove_tag: EventHandler<'a, TagId>,
) -> Element<'a> {
    let model = use_shared_state::<Model>(cx).unwrap();
    let selected = use_ref(cx, HashSet::new);
    if model.read().tag_search_created_tag.is_some() {
        if let Some((tag_id, _)) = model.write().tag_search_created_tag.take() {
            selected.write().insert(tag_id);
        }
    }
    let read_model = model.read();
    let read_selected = selected.read();
    let show_add_tag_button = use_state(cx, || true);
    let new_tag = use_state(cx, String::new);
    cx.render(rsx! {
        div {
            class: "flex flex-col gap-1",
            p {
                class: styles::TEXT_INPUT_LABEL,
                "Tags"
            },
            div {
                class: "flex flex-row gap-2 flex-wrap",
                for (tag_id, tag) in selected
                    .read()
                    .iter()
                    .map(|tag_id| (tag_id, &read_model.tags[tag_id]))
                {rsx!{
                    span {
                        class: "{styles::TAG_BADGE_SPAN} {color_picker::border_class(&tag.color)}",
                        "# {&tag.name}"
                        button {
                            r#type: "button",
                            class: "{styles::TAG_BADGE_BUTTON}",
                            onclick: {
                                let tag_id = *tag_id;
                                move |_| {
                                    selected.write().retain(|&this| this != tag_id);
                                    on_remove_tag.call(tag_id);
                                }
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
                                    d: "m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
                                }
                            }
                        }
                    }
                }}
            }
            ul {
                class: "
                    text-sm text-gray-200 rounded-lg
                    border border-gray-700 divide-y divide-gray-700
                ",
                rsx!{
                    for (tag_id, tag) in read_model
                        .tags
                        .iter()
                        .filter(|(id, _)| !read_selected.contains(id))
                        .sorted_by_key(|(_, tag)| tag.name.to_lowercase())
                    {rsx!{
                        li {
                            key: "{tag_id}",
                            button {
                                r#type: "button",
                                class: "
                                    text-left w-full px-4 py-2
                                    hover:bg-gray-800 hover:text-white
                                ",
                                prevent_default: "onmousedown",
                                onmousedown: |_| {},
                                onclick: {
                                    let tag_id = *tag_id;
                                    move |event| {
                                        event.stop_propagation();
                                        selected.write().insert(tag_id);
                                        on_select_tag.call(tag_id);
                                    }
                                },
                                tag.name.clone(),
                            }
                        },
                    }}
                }
                li {
                    key: "add tag",
                    if **show_add_tag_button {rsx! {
                        button {
                            r#type: "button",
                            class: "
                                text-left w-full px-4 py-2
                                hover:bg-gray-800
                                font-medium text-blue-500 hover:underline
                            ",
                            prevent_default: "onmousedown",
                            onmousedown: |_| {},
                            onclick: |_| {
                                show_add_tag_button.set(false);
                            },
                            "Add Tag"
                        }
                    }} else {rsx! {
                        div {
                            class: "p-2",
                            div {
                                class: "flex flex-col gap-2 p-2",
                                input {
                                    class: styles::TEXT_INPUT,
                                    r#type: "text",
                                    placeholder: "Tag",
                                    value: "{new_tag}",
                                    oninput: |event| {
                                        new_tag.set(event.value.clone())
                                    },
                                }
                                ColorPicker {
                                    on_pick_color: |color| {
                                        show_add_tag_button.set(true);
                                        if new_tag.trim().is_empty() {
                                            return;
                                        }
                                        cx.spawn(create_tag(
                                            model.clone(),
                                            TagData {
                                                name: new_tag.make_mut().drain(..).collect(),
                                                color
                                            },
                                        ));
                                    },
                                }
                            }
                        }
                    }}
                }

            }
        }
    })
}

#[component]
fn UserSearch<'a>(
    cx: Scope,
    on_select_user: EventHandler<'a, UserId>,
    on_remove_user: EventHandler<'a, UserId>,
) -> Element<'a> {
    let model = use_shared_state::<Model>(cx).unwrap();
    let selected = use_ref(cx, HashSet::new);
    if model.read().user_search_created_user.is_some() {
        if let Some((user_id, _)) = model.write().user_search_created_user.take() {
            selected.write().insert(user_id);
        }
    }
    let read_model = model.read();
    let read_selected = selected.read();
    let show_add_user_button = use_state(cx, || true);
    let new_user = use_state(cx, String::new);
    cx.render(rsx! {
        div {
            class: "flex flex-col gap-1",
            p {
                class: styles::TEXT_INPUT_LABEL,
                "Users"
            },
            div {
                class: "flex flex-row gap-2 flex-wrap",
                for (user_id, user) in selected
                    .read()
                    .iter()
                    .map(|user_id| (user_id, &read_model.users[user_id]))
                {rsx!{
                    span {
                        class: "
                            text-sm font-medium text-white
                            px-2.5 py-0.5 rounded
                            cursor-pointer
                            border-2
                            flex flex-row gap-2
                            {color_picker::border_class(&user.color)}
                        ",
                        "{&user.name}"
                        button {
                            r#type: "button",
                            class: "
                                border border-transparent sm:hover:border-white
                                inline-flex items-center p-1 font-medium rounded
                            ",
                            onclick: {
                                let user_id = *user_id;
                                move |_| {
                                    selected.write().retain(|&this| this != user_id);
                                    on_remove_user.call(user_id);
                                }
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
                                    d: "m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
                                }
                            }
                        }
                    }
                }}
            }
            ul {
                class: "
                    text-sm text-gray-200 rounded-lg
                    border border-gray-700 divide-y divide-gray-700
                ",
                rsx!{
                    for (user_id, user) in read_model
                        .users
                        .iter()
                        .filter(|(id, _)| !read_selected.contains(id))
                        .sorted_by_key(|(_, user)| user.name.to_lowercase())
                    {rsx!{
                        li {
                            key: "{user_id}",
                            button {
                                r#type: "button",
                                class: "
                                    text-left w-full px-4 py-2
                                    hover:bg-gray-800 hover:text-white
                                ",
                                prevent_default: "onmousedown",
                                onmousedown: |_| {},
                                onclick: {
                                    let user_id = *user_id;
                                    move |event| {
                                        event.stop_propagation();
                                        selected.write().insert(user_id);
                                        on_select_user.call(user_id);
                                    }
                                },
                                user.name.clone(),
                            }
                        },
                    }}
                }
                li {
                    key: "add user",
                    if **show_add_user_button {rsx! {
                        button {
                            r#type: "button",
                            class: "
                                text-left w-full px-4 py-2
                                hover:bg-gray-800
                                font-medium text-blue-500 hover:underline
                            ",
                            prevent_default: "onmousedown",
                            onmousedown: |_| {},
                            onclick: |_| {
                                show_add_user_button.set(false);
                            },
                            "Add User"
                        }
                    }} else {rsx! {
                        div {
                            class: "p-2",
                            div {
                                class: "flex flex-col gap-2 p-2",
                                input {
                                    class: styles::TEXT_INPUT,
                                    r#type: "text",
                                    placeholder: "Name",
                                    value: "{new_user}",
                                    oninput: |event| {
                                        new_user.set(event.value.clone())
                                    },
                                }
                                ColorPicker {
                                    on_pick_color: |color| {
                                        show_add_user_button.set(true);
                                        if new_user.trim().is_empty() {
                                            return;
                                        }
                                        cx.spawn(create_user(
                                            model.clone(),
                                            UserData {
                                                name: new_user.make_mut().drain(..).collect(),
                                                color
                                            },
                                        ));
                                    },
                                }
                            }
                        }
                    }}
                }

            }
        }
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

async fn create_tag(model: UseSharedState<Model>, tag_data: TagData) {
    if let Ok(tag_data) = requests::create_tag(model.clone(), tag_data).await {
        requests::board(model.clone()).await;
        model.write().tag_search_created_tag = Some(tag_data);
    }
}

async fn create_user(model: UseSharedState<Model>, user_data: UserData) {
    if let Ok(user_data) = requests::create_user(model.clone(), user_data).await {
        requests::board(model.clone()).await;
        model.write().user_search_created_user = Some(user_data);
    }
}
