use std::collections::HashSet;

use crate::color_picker;
use crate::color_picker::ColorPicker;
use crate::requests::{self, BoardSignals};
use crate::responsive_layout::ResponsiveLayout;
use crate::route::Route;
use crate::{model::Model, styles};
use chrono::{offset::Local, NaiveDate, NaiveTime, TimeZone};
use dioxus::prelude::*;
use itertools::Itertools;
use shared_models::{BoardName, TagData, TagId, TaskSize, TaskStatus, UserData, UserId};

#[component]
pub fn AddTask(board_name: BoardName) -> Element {
    rsx! {
        AddTaskImpl {
            board_name: board_name.clone(),
            default_status: TaskStatus::ToDo,
        }
    }
}

#[component]
pub fn AddToDoTask(board_name: BoardName) -> Element {
    rsx! {
        AddTaskImpl {
            board_name: board_name.clone(),
            default_status: TaskStatus::ToDo,
        }
    }
}

#[component]
pub fn AddInProgressTask(board_name: BoardName) -> Element {
    rsx! {
        AddTaskImpl {
            board_name: board_name.clone(),
            default_status: TaskStatus::InProgress,
        }
    }
}

#[component]
pub fn AddDoneTask(board_name: BoardName) -> Element {
    rsx! {
        AddTaskImpl {
            board_name: board_name.clone(),
            default_status: TaskStatus::Done,
        }
    }
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
fn AddTaskImpl(board_name: BoardName, default_status: TaskStatus) -> Element {
    let board_signals = BoardSignals::default();
    let mut model = use_context::<Signal<Model>>();
    let nav = use_navigator();
    let mut title = use_signal(String::new);
    let mut tags = use_signal(Vec::new);
    let mut description = use_signal(String::new);

    let mut size_signal = use_signal(|| TaskSize::Small);
    let size = size_signal();

    let mut status_signal = use_signal(|| default_status);
    let status = status_signal();

    let mut assigned_to = use_signal(Vec::new);

    let mut due_date_signal = use_signal(|| None::<NaiveDate>);
    let due_date = due_date_signal();

    let mut due_time_signal = use_signal(|| NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    let due_time = due_time_signal();

    let layout = ResponsiveLayout::from_window();

    let mut has_focus_signal = use_signal(|| false);
    let has_focus = has_focus_signal();

    if model.read().board_name != board_name {
        model.write().board_name = board_name.clone()
    }
    use_future(move || requests::board(board_signals));
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
                    class: "flex flex-col gap-5 items-left w-full max-w-lg",
                    onfocusin: move |_| has_focus_signal.set(true),
                    onfocusout: move |_| has_focus_signal.set(false),
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
                            oninput: move |event| title.set(event.value()),
                        },
                    }
                    TagSearch {
                        on_select_tag: move |tag_id| tags.write().push(tag_id),
                        on_remove_tag: move |tag_id| tags.write().retain(|&value| value != tag_id),
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
                                    checked: status == TaskStatus::ToDo,
                                    oninput: move |_| status_signal.set(TaskStatus::ToDo),
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
                                    checked: status == TaskStatus::InProgress,
                                    oninput: move |_| status_signal.set(TaskStatus::InProgress),
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
                                    checked: status == TaskStatus::Done,
                                    oninput: move |_| status_signal.set(TaskStatus::Done),
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
                                    oninput: move |_| size_signal.set(TaskSize::Small),
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
                                    oninput: move |_| size_signal.set(TaskSize::Medium),
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
                                    oninput: move |_| size_signal.set(TaskSize::Large),
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
                        on_select_user: move |user_id| assigned_to.write().push(user_id),
                        on_remove_user: move |user_id| assigned_to.write().retain(|&value| value != user_id),
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
                            oninput: move |event| description.set(event.value()),
                        },
                    },
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
                                oninput: move |event| {
                                    let event_value = event.value();
                                    if event_value.is_empty() {
                                        due_date_signal.set(None)
                                    } else if let Ok(date) = NaiveDate::parse_from_str(&event_value, "%Y-%m-%d") {
                                        due_date_signal.set(Some(date))
                                    }
                                },
                            },
                            if due_date.is_some() {
                                select {
                                    id: "task_due_time",
                                    class: "border text-sm rounded-lg block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500",
                                    onchange: move |event| {
                                        if let Ok(time) = NaiveTime::parse_from_str(&event.value(), "%H:%M") {
                                            due_time_signal.set(time)
                                        }
                                    },
                                    for hour in 0..24 {
                                        for minute in [0, 15, 30, 45] {
                                            option {
                                                value: "{hour:02}:{minute:02}",
                                                "{hour:02}:{minute:02}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    button {
                        class: styles::BUTTON,
                        r#type: "submit",
                        prevent_default: "onclick",
                        onclick: move |_| {
                            // TODO: once future issue is fixed change page
                            // as first thing
                            create_task(
                                board_signals,
                                shared_models::TaskData {
                                    title:
                                        title
                                        .write()
                                        .drain(..)
                                        .collect::<String>()
                                        .trim()
                                        .to_string(),
                                    description: description.write().drain(..).collect(),
                                    due: due_date.map(|date| {
                                        Local.from_local_datetime(&date.and_time(due_time))
                                        .unwrap()
                                        .into()
                                    }),
                                    size,
                                    status,
                                    assignees: assigned_to.write().drain(..).collect(),
                                    tags: tags.write().drain(..).collect(),
                                },
                                nav,
                            )
                        },
                        "Submit"
                    }
                }
            }
            if (layout == ResponsiveLayout::Wide) || (!has_focus && layout == ResponsiveLayout::Narrow) {
                div {
                    class: styles::BOTTOM_BAR,
                    button {
                        r#type: "button" ,
                        class: styles::BOTTOM_BAR_BUTTON,
                        onclick: move |_| {
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
        },
    }
}

#[component]
fn TagSearch(on_select_tag: EventHandler<TagId>, on_remove_tag: EventHandler<TagId>) -> Element {
    let board_signals = BoardSignals::default();
    let mut model = use_context::<Signal<Model>>();
    let mut selected = use_signal(HashSet::new);
    if model.read().tag_search_created_tag.is_some() {
        if let Some((tag_id, _)) = model.write().tag_search_created_tag.take() {
            selected.write().insert(tag_id);
            on_select_tag.call(tag_id);
        }
    }
    let read_model = model.read();
    let read_selected = selected.read();

    let mut show_add_tag_button_signal = use_signal(|| true);
    let show_add_tag_button = show_add_tag_button_signal();

    let mut new_tag = use_signal(String::new);
    rsx! {
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
                {
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
                }
            }
            ul {
                class: "
                    text-sm text-gray-200 rounded-lg
                    border border-gray-700 divide-y divide-gray-700
                ",
                for (tag_id, tag) in read_model
                    .tags
                    .iter()
                    .filter(|(id, _)| !read_selected.contains(id))
                    .sorted_by_key(|(_, tag)| tag.name.to_lowercase())
                {
                    li {
                        key: "{tag_id}",
                        button {
                            r#type: "button",
                            class: "
                                text-left w-full px-4 py-2
                                active:bg-gray-800 active:text-white
                                sm:hover:bg-gray-800 sm:hover:text-white
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
                            "{tag.name}"
                        }
                    }
                }
                li {
                    key: "{\"add tag\"}",
                    if show_add_tag_button {
                        button {
                            r#type: "button",
                            class: "
                                text-left w-full px-4 py-2
                                active:bg-gray-800 active:underline
                                sm:hover:bg-gray-800 sm:hover:underline
                                font-medium text-blue-500
                            ",
                            prevent_default: "onmousedown",
                            onmousedown: |_| {},
                            onclick: move |_| {
                                show_add_tag_button_signal.set(false);
                            },
                            "Add Tag"
                        }
                    } else {
                        div {
                            class: "p-2",
                            div {
                                class: "flex flex-col gap-2 p-2",
                                input {
                                    class: styles::TEXT_INPUT,
                                    r#type: "text",
                                    placeholder: "Tag",
                                    value: "{new_tag}",
                                    oninput: move |event| {
                                        new_tag.set(event.value())
                                    },
                                }
                                ColorPicker {
                                    on_pick_color: move |color| {
                                        show_add_tag_button_signal.set(true);
                                        let mut new_tag = new_tag.write();
                                        if new_tag.trim().is_empty() {
                                            return;
                                        }
                                        spawn(create_tag(
                                            board_signals,
                                            TagData {
                                                name: new_tag.drain(..).collect(),
                                                color
                                            },
                                        ));
                                    },
                                }
                            }
                        }
                    }
                }

            }
        }
    }
}

#[component]
fn UserSearch(
    on_select_user: EventHandler<UserId>,
    on_remove_user: EventHandler<UserId>,
) -> Element {
    let board_signals = BoardSignals::default();
    let mut model = use_context::<Signal<Model>>();
    let mut selected = use_signal(HashSet::new);
    if model.read().user_search_created_user.is_some() {
        if let Some((user_id, _)) = model.write().user_search_created_user.take() {
            selected.write().insert(user_id);
            on_select_user.call(user_id);
        }
    }
    let read_model = model.read();
    let read_selected = selected.read();

    let mut show_add_user_button_signal = use_signal(|| true);
    let show_add_user_button = show_add_user_button_signal();

    let mut new_user = use_signal(String::new);
    rsx! {
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
                {
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
                }
            }
            ul {
                class: "
                    text-sm text-gray-200 rounded-lg
                    border border-gray-700 divide-y divide-gray-700
                ",
                for (user_id, user) in read_model
                    .users
                    .iter()
                    .filter(|(id, _)| !read_selected.contains(id))
                    .sorted_by_key(|(_, user)| user.name.to_lowercase())
                {
                    li {
                        key: "{user_id}",
                        button {
                            r#type: "button",
                            class: "
                                text-left w-full px-4 py-2
                                active:bg-gray-800 active:text-white
                                sm:hover:bg-gray-800 sm:hover:text-white
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
                            "{user.name}"
                        }
                    }
                }
                li {
                    key: "{\"add user\"}",
                    if show_add_user_button {
                        button {
                            r#type: "button",
                            class: "
                                text-left w-full px-4 py-2
                                active:bg-gray-800 active:underline
                                sm:hover:bg-gray-800 sm:hover:underline
                                font-medium text-blue-500
                            ",
                            prevent_default: "onmousedown",
                            onmousedown: |_| {},
                            onclick: move |_| {
                                show_add_user_button_signal.set(false);
                            },
                            "Add User"
                        }
                    } else {
                        div {
                            class: "p-2",
                            div {
                                class: "flex flex-col gap-2 p-2",
                                input {
                                    class: styles::TEXT_INPUT,
                                    r#type: "text",
                                    placeholder: "Name",
                                    value: "{new_user}",
                                    oninput: move |event| {
                                        new_user.set(event.value())
                                    },
                                }
                                ColorPicker {
                                    on_pick_color: move |color| {
                                        show_add_user_button_signal.set(true);
                                        let mut new_user = new_user.write();
                                        if new_user.trim().is_empty() {
                                            return;
                                        }
                                        spawn(create_user(
                                            board_signals,
                                            UserData {
                                                name: new_user.drain(..).collect(),
                                                color
                                            },
                                        ));
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn create_task(signals: BoardSignals, task_data: shared_models::TaskData, nav: Navigator) {
    if task_data.title.is_empty() {
        log::info!("empty task title, doing nothing");
        return;
    }
    if let Ok(task_id) = requests::create_task(signals.board, &task_data).await {
        log::info!("created task: {task_id}");
    }
    nav.push(Route::Board {
        board_name: signals.board.read().board_name.clone(),
    });
}

async fn create_tag(mut signals: BoardSignals, tag_data: TagData) {
    if let Ok(tag_data) = requests::create_tag(signals.board, tag_data).await {
        requests::board(signals).await;
        signals.model.write().tag_search_created_tag = Some(tag_data);
    }
}

async fn create_user(mut signals: BoardSignals, user_data: UserData) {
    if let Ok(user_data) = requests::create_user(signals.board, user_data).await {
        requests::board(signals).await;
        signals.model.write().user_search_created_user = Some(user_data);
    }
}
