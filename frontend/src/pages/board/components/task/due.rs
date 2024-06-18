use chrono::{DateTime, Local, NaiveDate, NaiveTime, TimeZone, Utc};
use dioxus::prelude::*;
use reqwest::Client;
use shared_models::TaskId;

use crate::{
    components::{
        form::{CancelButton, ConfirmButton},
        icons::{ClockIcon, EditIcon},
        input::DateInput,
        tooltip::Tooltip,
    },
    datetime,
    pages::board::requests::{self, BoardSignals},
    themes::Theme,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DueOptions {
    pub due: DateTime<Utc>,
    pub show_time_left: bool,
    pub is_late: bool,
}

#[component]
pub fn Due(task_id: TaskId, due: Option<DueOptions>) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            EditingDue {
                task_id,
                due: due.map(|d| d.due),
                editing
            }
        } else {
            ShowDue { task_id, due, editing }
        }
    }
}

#[component]
fn EditingDue(task_id: TaskId, due: Option<DateTime<Utc>>, editing: Signal<bool>) -> Element {
    let board_signals = BoardSignals::default();
    let mut has_due = use_signal(|| due.is_some());
    rsx! {
        form {
            "aria-label": "set due date",
            class: "flex flex-row flex-wrap gap-1 items-center",
            onsubmit: move |event| {
                let values = event.values();
                let due_string = values["Due"].as_value();
                let due = if due_string.is_empty() {
                    None
                } else {
                    let due = NaiveDate::parse_from_str(&due_string, "%Y-%m-%d").unwrap();
                    let  time = NaiveTime::parse_from_str(
                        &format!(
                            "{}:{} {}",
                            values["Hour"].as_value(),
                            values["Minute"].as_value(),
                            values["AM/PM"].as_value(),
                        ),
                        "%I:%M %p",
                    ).unwrap();
                    Some(Local.from_local_datetime(&due.and_time(time)).unwrap().into())
                };
                spawn_forever(set_task_due(board_signals, task_id, due));
                editing.set(false);
            },
            div { class: "size-6", ClockIcon {} }
            DateInput {
                id: "task-{task_id}-due-input",
                label: "Due",
                value: due.map(|d| d.format("%Y-%m-%d").to_string()),
                oninput: move |event: FormEvent| has_due.set(!event.value().is_empty()),
            }
            if has_due() {
                TimeSelect {}
            }
            ConfirmButton { label: "set due" }
            CancelButton { label: "cancel due update", editing }
        }
    }
}

#[component]
fn TimeSelect() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "border rounded-lg {} {}",
        theme.bg_color_2, theme.border_color
    );
    rsx! {
        div {
            class: "flex flex-row items-center gap-0.5",
            select {
                name: "Hour",
                class: "text-base p-2.5 {style}",
                for hour in 1..=12 {
                    option { value: hour, "{hour:02}" }
                }
            }
            p { ":" }
            select {
                name: "Minute",
                class: "text-base p-2.5 {style}",
                for minute in [0, 15, 30, 45] {
                    option { value: minute, "{minute:02}" }
                }
            }
            select {
                name: "AM/PM",
                class: "text-base p-2.5 {style}",
                option { value: "AM", "AM" }
                option { value: "PM", "PM" }
            }
        }
    }
}

#[component]
fn ShowDue(task_id: TaskId, due: Option<DueOptions>, editing: Signal<bool>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let icon_style = if let Some(DueOptions {
        is_late: true,
        show_time_left: true,
        ..
    }) = due
    {
        theme.late_text_color
    } else {
        ""
    };
    rsx! {
        section {
            "aria-label": "due date",
            class: "flex flex-row gap-2 items-center text-sm",
            div { class: "size-6 {icon_style}", ClockIcon {} }
            if let Some(DueOptions { due, show_time_left, is_late }) = due {
                HasDue { due, show_time_left, is_late }
                EditButton { task_id, editing, dir: "rtl" }
            } else {
                EditButton { task_id, editing, dir: "" }
            }
        }
    }
}

#[component]
fn HasDue(due: DateTime<Utc>, show_time_left: bool, is_late: bool) -> Element {
    let now = Utc::now();
    let time_delta = datetime::time_delta(&now, &due);
    rsx! {
        div {
            class: "flex flex-col gap-0.5",
            p {
                "Due: {datetime::format(datetime::utc_to_local(&due))}"
            }
            if show_time_left {
                if time_delta.is_negative() {
                    p {
                        "Late: {datetime::time_delta(&now, &due)}"
                    }
                } else {
                    p {
                        "Left: {datetime::time_delta(&now, &due)}"
                    }
                }
            }
        }
    }
}

#[component]
fn EditButton(task_id: TaskId, editing: Signal<bool>, dir: &'static str) -> Element {
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "edit due date",
                class: "block size-5",
                onclick: move |_| editing.set(true),
                EditIcon {}
            }
            Tooltip {
                content: "Edit Due Date",
                position: "",
                dir,
            }
        }
    }
}

async fn set_task_due(signals: BoardSignals, task_id: TaskId, due: Option<DateTime<Utc>>) {
    if send_set_task_due_request(signals, task_id, due)
        .await
        .is_ok()
    {
        requests::board(signals).await;
    }
}

async fn send_set_task_due_request(
    signals: BoardSignals,
    task_id: TaskId,
    due: Option<DateTime<Utc>>,
) -> Result<(), anyhow::Error> {
    let url = {
        let url = &signals.url.read().0;
        let board = signals.board.read();
        url.join(&format!(
            "/api/boards/{}/tasks/{}/due",
            board.board_name, task_id
        ))?
    };
    Ok(Client::new()
        .put(url)
        .json(&due)
        .send()
        .await?
        .json::<()>()
        .await?)
}
