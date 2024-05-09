use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use shared_models::{TagId, TaskId, UserId};

#[component]
pub fn Task(
    task_id: TaskId,
    title: String,
    description: String,
    assignees: Vec<UserId>,
    tags: Vec<TagId>,
    due: Option<DateTime<Utc>>,
) -> Element {
    let style = "
        border border-gray-700
        rounded-lg
        shadow
        bg-gray-800 sm:hover:bg-gray-700
    ";
    let expanded = use_signal(|| false);
    rsx! {
        div {
            class: "flex flex-col gap-2 p-3 {style}",
            div {
                class: "",
                Title { task_id, title }
                StatusButtons { task_id }
            }
            div {
                class: "",
                Users { task_id, users: assignees }
                TaskActions { task_id }
            }
            Tags { task_id, tags }
            if expanded() {
                Due { task_id, due }
                Description { task_id, description }
                SpecialActions { task_id }
            }
            ToggleExpanded {
                expanded
            }
        }
    }
}

#[component]
fn Title(task_id: TaskId, title: String) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            TitleInput { task_id, editing }
        } else {
            TitleShow { editing, title }
        }
    }
}

#[component]
fn TitleInput(task_id: TaskId, editing: Signal<bool>) -> Element {
    let input_style = "";
    let confirm_button_style = "";
    let cancel_button_style = "";
    rsx! {
        div {
            input {
                class: input_style,

            }
            button {
                class: confirm_button_style,
                onclick: move |_| {
                    editing.set(false);
                }
            }
            button {
                class: cancel_button_style,
                onclick: move |_| {
                    editing.set(false);
                }
            }
        }
    }
}

#[component]
fn TitleShow(editing: Signal<bool>, title: String) -> Element {
    rsx! {
        div {
            class: "flex items-center gap-2",
            {title}
            button {
                class: "",
                onclick: move |_| {
                    editing.set(true);
                }
            }
        }
    }
}

#[component]
fn Description(task_id: TaskId, description: String) -> Element {
    todo!()
}

#[component]
fn SpecialActions(task_id: TaskId) -> Element {
    todo!()
}

#[component]
fn StatusButtons(task_id: TaskId) -> Element {
    let style = "";
    rsx! {
        div {
            class: "",
            button {
                class: style,
                BoltIcon {}
            }
            button {
                class: style,
                CopyIcon {}
            }
            button {
                class: style,
                ArchiveIcon {}
            }
        }
    }
}

#[component]
fn BoltIcon() -> Element {
    todo!()
}

#[component]
fn CopyIcon() -> Element {
    todo!()
}

#[component]
fn ArchiveIcon() -> Element {
    todo!()
}

#[component]
fn Users(task_id: TaskId, users: Vec<UserId>) -> Element {
    todo!()
}

#[component]
fn Tags(task_id: TaskId, tags: Vec<TagId>) -> Element {
    todo!()
}

#[component]
fn TaskActions(task_id: TaskId) -> Element {
    todo!()
}

#[component]
fn Due(task_id: TaskId, due: Option<DateTime<Utc>>) -> Element {
    todo!()
}

#[component]
fn ToggleExpanded(expanded: Signal<bool>) -> Element {
    todo!()
}
