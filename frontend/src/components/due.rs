use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use shared_models::TaskId;

#[component]
pub fn Due(task_id: TaskId, due: Option<DateTime<Utc>>) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            EditingDue { task_id, due, editing }
        } else {
            ShowDue { task_id, due, editing }
        }
    }
}

#[component]
fn EditingDue(task_id: TaskId, due: Option<DateTime<Utc>>, editing: Signal<bool>) -> Element {
    rsx! {}
}

#[component]
fn ShowDue(task_id: TaskId, due: Option<DateTime<Utc>>, editing: Signal<bool>) -> Element {
    rsx! {}
}
