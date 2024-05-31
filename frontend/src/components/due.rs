use std::fmt::Display;

use chrono::{DateTime, Local, Utc};
use dioxus::prelude::*;
use shared_models::TaskId;

use crate::{
    components::{
        form::{CancelButton, ConfirmButton},
        icons::{CalendarIcon, EditIcon},
        input::DateInput,
        tooltip::Tooltip,
    },
    themes::Theme,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DueOptions {
    pub due: DateTime<Utc>,
    pub show_time_left: bool,
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
    let mut has_due = use_signal(|| due.is_some());
    rsx! {
        form {
            "aria-label": "set due date",
            class: "flex flex-row gap-1 items-center",
            div { class: "size-8", CalendarIcon {} }
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
                class: "text-base p-2.5 {style}",
                for hour in 1..=12 {
                    option { value: hour, "{hour:02}" }
                }
            }
            p { ":" }
            select {
                class: "text-base p-2.5 {style}",
                for minute in [0, 15, 30, 45] {
                    option { value: minute, "{minute:02}" }
                }
            }
            select {
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
    let now = Utc::now();
    rsx! {
        section {
            "aria-label": "due date",
            class: "flex flex-row gap-2 items-center",
            div { class: "size-8", CalendarIcon {} }
            if let Some(DueOptions { due: due_value, show_time_left }) = due {
                p {
                    class: theme.late_text_color,
                    if show_time_left {
                        "{format_datetime(utc_to_local(&due_value))} ({time_delta(&now, &due_value)})"
                    } else {
                        "{format_datetime(utc_to_local(&due_value))}"
                    }
                }
            }
            EditButton { task_id, editing }
        }
    }
}

struct TimeDelta {
    days: i32,
    hours: i8,
    minutes: i8,
}

impl Display for TimeDelta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d {}h {}m", self.days, self.hours, self.minutes)
    }
}

fn time_delta(start: &DateTime<Utc>, stop: &DateTime<Utc>) -> TimeDelta {
    let duration = stop.naive_utc() - start.naive_utc();
    let days = duration.num_days();
    let hours = duration.num_hours() - duration.num_days() * 24;
    let minutes = duration.num_minutes() - (days * 24 * 60) - (hours * 60);
    TimeDelta {
        days: days as i32,
        hours: hours as i8,
        minutes: minutes as i8,
    }
}

fn utc_to_local(time: &DateTime<Utc>) -> DateTime<Local> {
    chrono::DateTime::<chrono::offset::Local>::from_naive_utc_and_offset(
        time.naive_utc(),
        *chrono::offset::Local::now().offset(),
    )
}

fn format_datetime(time: DateTime<Local>) -> String {
    format!("{}", time.format("%d %B %Y %I:%M %p"))
}

#[component]
fn EditButton(task_id: TaskId, editing: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "relative",
            button {
                "aria-label": "edit due date",
                class: "peer size-5",
                onclick: move |_| editing.set(true),
                EditIcon {}
            }
            Tooltip {
                content: "Edit Due Date",
                position: ""
            }
        }
    }
}
