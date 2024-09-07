use dioxus::prelude::*;
use dioxus_sdk::{i18n::use_i18, translate};
use shared_models::TaskStatus;

use crate::{
    commands::ScrollTarget,
    components::{
        form::{CancelButton, ConfirmButton},
        input::TextInput,
    },
    pages::board::requests::{self, BoardSignals},
    themes::Theme,
};

#[component]
pub fn NewTaskForm(status: TaskStatus, adding_task: Signal<bool>) -> Element {
    let i18 = use_i18();
    let scroll_target = use_context::<Signal<ScrollTarget>>();
    let board_signals = BoardSignals::default();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        sm:border
        sm:rounded-lg
        sm:shadow
        {} {}
        ",
        theme.border_color, theme.bg_color_2,
    );
    rsx! {
        form {
            aria_label: translate!(i18, "new_task_form_label"),
            class: "flex flex-row gap-1 p-2.5 items-center {style}",
            onsubmit: move |event| {
                let title = event.values()["Title"].as_value();
                spawn_forever(create_task(board_signals, title, status, scroll_target));
                adding_task.set(false);
            },
            TextInput {
                id: "new-{status:#?}-task-title-input",
                label: "Title",
            }
            ConfirmButton { label: "add task" }
            CancelButton { label: "cancel adding task", editing: adding_task }
        }
    }
}

async fn create_task(
    signals: BoardSignals,
    title: String,
    status: TaskStatus,
    mut scroll_target: Signal<ScrollTarget>,
) {
    if let Ok(task_id) = requests::create_task(
        signals.url,
        signals.board,
        shared_models::NewTaskData {
            title,
            description: String::new(),
            due: None,
            status,
            assignees: Vec::new(),
            tags: Vec::new(),
            new_tags: Vec::new(),
        },
    )
    .await
    {
        log::info!("created task: {task_id}");
        requests::board(signals).await;
        scroll_target.set(ScrollTarget(Some(format!("task-{task_id}-article"))));
    }
}
