use dioxus::prelude::*;
use shared_models::{Color, TagData, TagId, TaskId};

use crate::{
    components::icons::CancelIcon,
    model::TagFilter,
    requests::{self, BoardSignals},
};

#[component]
pub fn FilterBarTagIcon(tag_id: TagId, tag_data: TagData) -> Element {
    let mut tag_filter = use_context::<Signal<TagFilter>>();
    let color = match tag_data.color {
        Color::Black => "border-black aria-pressed:bg-black",
        Color::White => "border-white aria-pressed:bg-white",
        Color::Gray => "border-gray-400 aria-pressed:bg-gray-400",
        Color::Silver => "border-slate-500 aria-pressed:bg-slate-500",
        Color::Maroon => "border-rose-400 aria-pressed:bg-rose-400",
        Color::Red => "border-red-600 aria-pressed:bg-red-600",
        Color::Purple => "border-purple-600 aria-pressed:bg-purple-600",
        Color::Fushsia => "border-fuchsia-400 aria-pressed:bg-fuchsia-400",
        Color::Green => "border-emerald-500 aria-pressed:bg-emerald-500",
        Color::Lime => "border-lime-500 aria-pressed:bg-lime-500",
        Color::Olive => "border-indigo-400 aria-pressed:bg-indigo-400",
        Color::Yellow => "border-yellow-400 aria-pressed:bg-yellow-400",
        Color::Navy => "border-amber-200 aria-pressed:bg-amber-200",
        Color::Blue => "border-blue-400 aria-pressed:bg-blue-400",
        Color::Teal => "border-teal-300 aria-pressed:bg-teal-300",
        Color::Aqua => "border-cyan-500 aria-pressed:bg-cyan-500",
    };
    let style = "rounded border-2";
    let pressed = tag_filter.read().0.contains(&tag_id);
    rsx! {
        div {
            class: "
                group
                flex flex-row items-center
                px-1.5 py-0.5
                {style} {color}
            ",
            "aria-pressed": pressed,
            button {
                class: "text-sm pr-1",
                "aria-label": "toggle {tag_data.name} filter",
                "aria-pressed": pressed,
                onclick: move |_| {
                    let mut tag_filter = tag_filter.write();
                    if tag_filter.0.contains(&tag_id) {
                        tag_filter.0.remove(&tag_id);
                    } else {
                        tag_filter.0.insert(tag_id);
                    }
                },
                "# {tag_data.name}"
            }
        }
    }
}

#[component]
pub fn TaskTagIcon(task_id: TaskId, tag_id: TagId, tag_data: TagData) -> Element {
    let board_signals = BoardSignals::default();
    let mut tag_filter = use_context::<Signal<TagFilter>>();
    let color = match tag_data.color {
        Color::Black => "border-black aria-pressed:bg-black",
        Color::White => "border-white aria-pressed:bg-white",
        Color::Gray => "border-gray-400 aria-pressed:bg-gray-400",
        Color::Silver => "border-slate-500 aria-pressed:bg-slate-500",
        Color::Maroon => "border-rose-400 aria-pressed:bg-rose-400",
        Color::Red => "border-red-600 aria-pressed:bg-red-600",
        Color::Purple => "border-purple-600 aria-pressed:bg-purple-600",
        Color::Fushsia => "border-fuchsia-400 aria-pressed:bg-fuchsia-400",
        Color::Green => "border-emerald-500 aria-pressed:bg-emerald-500",
        Color::Lime => "border-lime-500 aria-pressed:bg-lime-500",
        Color::Olive => "border-indigo-400 aria-pressed:bg-indigo-400",
        Color::Yellow => "border-yellow-400 aria-pressed:bg-yellow-400",
        Color::Navy => "border-amber-200 aria-pressed:bg-amber-200",
        Color::Blue => "border-blue-400 aria-pressed:bg-blue-400",
        Color::Teal => "border-teal-300 aria-pressed:bg-teal-300",
        Color::Aqua => "border-cyan-500 aria-pressed:bg-cyan-500",
    };
    let style = "rounded border-2";
    let delete_tag_button_style = "rounded active:border sm:hover:border";
    let pressed = tag_filter.read().0.contains(&tag_id);
    rsx! {
        div {
            class: "
                group
                flex flex-row items-center
                px-1.5 py-0.5
                {style} {color}
            ",
            "aria-pressed": pressed,
            button {
                class: "text-sm pr-1",
                "aria-label": "toggle {tag_data.name} filter",
                "aria-pressed": pressed,
                onclick: move |_| {
                    let mut tag_filter = tag_filter.write();
                    if tag_filter.0.contains(&tag_id) {
                        tag_filter.0.remove(&tag_id);
                    } else {
                        tag_filter.0.insert(tag_id);
                    }
                },
                "# {tag_data.name}"
            }
            button {
                "aria-label": "remove tag {tag_data.name} from task",
                class: "size-5 p-0.5 {delete_tag_button_style}",
                onclick: move |_| {
                    spawn_forever(delete_task_tag(board_signals, task_id, tag_id));
                },
                CancelIcon {}
            }
        }
    }
}

async fn delete_task_tag(signals: BoardSignals, task_id: TaskId, tag_id: TagId) {
    if send_delete_task_tag_request(signals, task_id, tag_id)
        .await
        .is_ok()
    {
        requests::board(signals).await;
    }
}

async fn send_delete_task_tag_request(
    signals: BoardSignals,
    task_id: TaskId,
    tag_id: TagId,
) -> Result<(), anyhow::Error> {
    let url = {
        let url = &signals.url.read().0;
        let board = signals.board.read();
        url.join(&format!(
            "/api/boards/{}/tasks/{}/tags/{}",
            board.board_name, task_id, tag_id
        ))?
    };
    Ok(reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await?)
}
