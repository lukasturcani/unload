use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use shared_models::{Color, TagData, TagEntry, TagId, TaskStatus, TaskSuggestion, UserId};

use crate::{
    components::{
        form::{CancelButton, ConfirmButton},
        icons::{CancelIcon, ConfirmIcon, EditIcon},
        input::TextInput,
        tooltip::Tooltip,
    },
    description_parser::{parse_blocks, Block, Line},
    model::UnloadUrl,
    pages::board::{
        components::{
            assignee_selection::AssigneeSelection, assignees::Assignees,
            description_input::DescriptionInput, tag_selection::TagSelection, task_tags::TaskTags,
        },
        model::{Board, ChatGptResponse, NumChatGptCalls, Tags, Users},
        requests::{self, BoardSignals},
    },
    themes::Theme,
};

#[component]
pub fn ChatGpt(chat_gpt_response: Signal<Option<ChatGptResponse>>) -> Element {
    rsx! {
        match &*chat_gpt_response.read() {
            Some(ChatGptResponse::Waiting) => rsx! {
                ChatGptWaiting {}
            },
            Some(ChatGptResponse::Suggestions(suggestions)) => rsx! {
                ChatGptSuggestions {
                    suggestions: suggestions.clone(),
                    chat_gpt_response,
                }
            },
            Some(ChatGptResponse::LimitExceeded) => rsx! {
                ChatGptLimitExceeded { chat_gpt_response }
            },
            Some(ChatGptResponse::Error) => rsx! {
                ChatGptError { chat_gpt_response }
            },
            Some(ChatGptResponse::Resolved) => rsx! {},
            None => rsx! {
                ChatGptPromptInput { chat_gpt_response }
            }
        }
    }
}

#[component]
fn ChatGptLimitExceeded(chat_gpt_response: Signal<Option<ChatGptResponse>>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2 items-center justify-center",
            h2 {
                class: "text-xl font-bold",
                "ChatGPT Limit Exceeded"
            },
            p {
                class: "text-sm",
                "You have reached the limit of ChatGPT calls. Please try again later."
            }
        }
    }
}

#[component]
fn ChatGptWaiting() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2 items-center justify-center",
            role: "status",
            p {
                "Talking to ChatGPT..."
            }
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "size-16",
                view_box: "0 0 24 24",
                path {
                    fill: "currentColor",
                    d: "M20.27,4.74a4.93,4.93,0,0,1,1.52,4.61,5.32,5.32,0,0,1-4.1,4.51,5.12,5.12,0,0,1-5.2-1.5,5.53,5.53,0,0,0,6.13-1.48A5.66,5.66,0,0,0,20.27,4.74ZM12.32,11.53a5.49,5.49,0,0,0-1.47-6.2A5.57,5.57,0,0,0,4.71,3.72,5.17,5.17,0,0,1,9.53,2.2,5.52,5.52,0,0,1,13.9,6.45,5.28,5.28,0,0,1,12.32,11.53ZM19.2,20.29a4.92,4.92,0,0,1-4.72,1.49,5.32,5.32,0,0,1-4.34-4.05A5.2,5.2,0,0,1,11.6,12.5a5.6,5.6,0,0,0,1.51,6.13A5.63,5.63,0,0,0,19.2,20.29ZM3.79,19.38A5.18,5.18,0,0,1,2.32,14a5.3,5.3,0,0,1,4.59-4,5,5,0,0,1,4.58,1.61,5.55,5.55,0,0,0-6.32,1.69A5.46,5.46,0,0,0,3.79,19.38ZM12.23,12a5.11,5.11,0,0,0,3.66-5,5.75,5.75,0,0,0-3.18-6,5,5,0,0,1,4.42,2.3,5.21,5.21,0,0,1,.24,5.92A5.4,5.4,0,0,1,12.23,12ZM11.76,12a5.18,5.18,0,0,0-3.68,5.09,5.58,5.58,0,0,0,3.19,5.79c-1,.35-2.9-.46-4-1.68A5.51,5.51,0,0,1,11.76,12ZM23,12.63a5.07,5.07,0,0,1-2.35,4.52,5.23,5.23,0,0,1-5.91.2,5.24,5.24,0,0,1-2.67-4.77,5.51,5.51,0,0,0,5.45,3.33A5.52,5.52,0,0,0,23,12.63ZM1,11.23a5,5,0,0,1,2.49-4.5,5.23,5.23,0,0,1,5.81-.06,5.3,5.3,0,0,1,2.61,4.74A5.56,5.56,0,0,0,6.56,8.06,5.71,5.71,0,0,0,1,11.23Z",
                    animateTransform {
                        attribute_name: "transform",
                        dur: "1.5s",
                        repeat_count: "indefinite",
                        r#type: "rotate",
                        values: "0 12 12;360 12 12"
                    }
                }
            }
        }
    }
}

#[component]
fn ChatGptSuggestions(
    suggestions: Vec<TaskSuggestion>,
    chat_gpt_response: Signal<Option<ChatGptResponse>>,
) -> Element {
    let tags = use_context::<Signal<Tags>>();
    let name_to_entry = use_memo(move || {
        let tags = &tags.read().0;
        tags.iter().fold(HashMap::new(), |mut map, (id, tag)| {
            map.insert(
                tag.name.clone(),
                TagEntry {
                    id: *id,
                    name: tag.name.clone(),
                    color: tag.color,
                },
            );
            map
        })
    });
    let resolved_suggestions = use_signal(HashSet::new);
    let resolved_suggestions_ = &resolved_suggestions.read();
    if resolved_suggestions_.len() == suggestions.len() {
        chat_gpt_response.set(Some(ChatGptResponse::Resolved));
    }
    rsx! {
        div {
            class: "
                flex flex-col gap-2 items-center
                max-h-full overflow-y-auto
            ",
            for (suggestion_id, suggestion) in suggestions
                .into_iter()
                .enumerate()
                .filter(|(id, _)| !resolved_suggestions_.contains(id))
            {
                TaskSuggestionCard {
                    key: "{suggestion.title}",
                    suggestion_id,
                    suggestion,
                    resolved_suggestions,
                    name_to_entry,
                }
            }
        }
    }
}

#[component]
fn TaskSuggestionCard(
    suggestion_id: usize,
    resolved_suggestions: Signal<HashSet<usize>>,
    suggestion: ReadOnlySignal<TaskSuggestion>,
    name_to_entry: ReadOnlySignal<HashMap<String, TagEntry>>,
) -> Element {
    let colors = [
        Color::Black,
        Color::White,
        Color::Gray,
        Color::Silver,
        Color::Maroon,
        Color::Red,
        Color::Purple,
        Color::Fushsia,
        Color::Green,
        Color::Lime,
        Color::Olive,
        Color::Yellow,
        Color::Navy,
        Color::Blue,
        Color::Teal,
        Color::Aqua,
    ];
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        sm:rounded-lg
        sm:shadow
        first:border-t border-b sm:border
        w-full
        {} {}",
        theme.border_color, theme.bg_color_2
    );
    let users = use_context::<Signal<Users>>();
    let all_tags = use_context::<Signal<Tags>>();
    let board_signals = BoardSignals::default();
    let select_assignees = use_signal(|| false);
    let select_tags = use_signal(|| false);
    let mut title = use_signal(String::new);
    use_effect(move || {
        let suggestion = suggestion.read();
        title.set(suggestion.title.clone());
    });
    let mut description = use_signal(String::new);
    use_effect(move || {
        let suggestion = suggestion.read();
        description.set(suggestion.description.clone());
    });
    let mut assignees = use_signal(Vec::new);
    let mut tags = use_signal(|| {
        let name_to_entry = name_to_entry.read();
        suggestion
            .read()
            .tags
            .iter()
            .filter_map(|name| name_to_entry.get(name))
            .map(|entry| entry.id)
            .collect::<Vec<_>>()
    });
    let mut new_tags = use_signal(|| {
        let name_to_entry = name_to_entry.read();
        suggestion
            .read()
            .tags
            .iter()
            .filter_map(|name| match name_to_entry.get(name) {
                Some(_) => None,
                None => Some(TagData {
                    name: name.clone(),
                    color: colors
                        [(name.bytes().fold(0, |acc, byte| acc + (byte as u16)) % 16) as usize],
                }),
            })
            .collect::<Vec<_>>()
    });
    use_effect(move || {
        let all_tags = &all_tags.read().0;
        let name_to_entry = name_to_entry.read();
        let mut tags = tags.write();
        let mut new_tags = new_tags.write();

        tags.retain(|tag| all_tags.contains_key(tag));
        new_tags.retain(|tag| {
            if let Some(entry) = name_to_entry.get(&tag.name) {
                tags.push(entry.id);
                false
            } else {
                true
            }
        });
    });
    rsx! {
        article {
            aria_label: "{title}",
            class: "flex flex-col gap-2 p-2.5 {style}",
            Title { suggestion_id, title }
            Description { suggestion_id description }
            Assignees {
                id: "suggestion-{suggestion_id}-assignees",
                assignees,
                select_assignees,
                on_toggle_selector: move |_| {},
            }
            if select_assignees() {
                AssigneeSelection {
                    id: "suggestion-{suggestion_id}-assignee-selection",
                    assignees,
                    on_assign_user: move |user_id| {
                        assignees.write().push(user_id);
                    },
                    on_unassign_user: move |user_id| {
                        assignees.write().retain(|&id| id != user_id);
                    },
                    on_add_user: move |user_id| {
                        spawn_forever(add_suggestion_assignee(
                            board_signals,
                            users,
                            assignees,
                            user_id,
                        ));
                    },
                }
            }
            TaskTags {
                id: "suggestion-{suggestion_id}-tags",
                tags,
                new_tags,
                select_tags,
                on_unassign_tag: move |tag_id| {
                    tags.write().retain(|&id| id != tag_id);
                },
                on_toggle_selector: move |_| {},
            }
            if select_tags() {
                TagSelection {
                    id: "suggestion-{suggestion_id}-tag-selection",
                    tags,
                    on_assign_tag: move |tag_id| {
                        tags.write().push(tag_id);
                    },
                    on_add_tag: move |tag_id| {
                        spawn_forever(add_suggestion_tag(
                            board_signals,
                            all_tags,
                            tags,
                            tag_id,
                        ));
                    },
                }
            }
            div {
                class: "flex flex-row gap-2 items-center justify-center w-full",
                AddTaskButton {
                    suggestion_id,
                    resolved_suggestions,
                    suggestion: SuggestionSignals {
                        title,
                        description,
                        assignees,
                        tags,
                        new_tags,
                    },
                }
                DeleteTaskButton { suggestion_id, resolved_suggestions }
            }
        }
    }
}

#[component]
fn Title(suggestion_id: usize, title: Signal<String>) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            TitleInput { suggestion_id, editing, title }
        } else {
            TitleShow { editing, title }
        }
    }
}

#[component]
fn TitleShow(editing: Signal<bool>, title: String) -> Element {
    rsx! {
        div {
            class: "flex flex-row gap-2 pr-2 items-center",
            h3 {
                class: "
                    text-lg sm:text-xl
                    font-bold tracking-tight
                ",
                {title}
            }
            EditButton { tooltip: "Edit Title", editing }
        }
    }
}

#[component]
fn EditButton(tooltip: &'static str, editing: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "edit title",
                class: "block size-5",
                onclick: move |_| editing.set(true),
                EditIcon {}
            }
            Tooltip {
                content: tooltip,
                position: "",
            }
        }
    }
}

#[component]
fn TitleInput(suggestion_id: usize, editing: Signal<bool>, title: Signal<String>) -> Element {
    let read_only_title = ReadOnlySignal::from(title);
    rsx! {
        form {
            "aria-label": "update title",
            class: "flex flex-row gap-2 justify-start items-center",
            onsubmit: move |event| {
                title.set(event.values()["Title"].as_value());
                editing.set(false);
            },
            div {
                class: "flex flex-row gap-1 items-center",
                TextInput {
                    id: "suggestion-{suggestion_id}-title-input",
                    label: "Title",
                    value: read_only_title,
                }
            }
            div {
                class: "flex flex-row gap-1 items-center",
                ConfirmButton { label: "set title" }
                CancelButton { label: "cancel title update", editing }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct SuggestionSignals {
    title: Signal<String>,
    description: Signal<String>,
    assignees: Signal<Vec<UserId>>,
    tags: Signal<Vec<TagId>>,
    new_tags: Signal<Vec<TagData>>,
}

#[component]
fn AddTaskButton(
    suggestion_id: usize,
    resolved_suggestions: Signal<HashSet<usize>>,
    suggestion: SuggestionSignals,
) -> Element {
    let style = "
        rounded-md
        border border-green-500
        stroke-green-500
        active:bg-green-500
        sm:hover:bg-green-500 sm:hover:stroke-white
    ";
    let board_signals = BoardSignals::default();
    rsx! {
        button {
            aria_label: "add task",
            class: "size-7 {style}",
            onclick: move |_| {
                let mut resolved_suggestions = resolved_suggestions.write();
                resolved_suggestions.insert(suggestion_id);
                spawn_forever(create_task(board_signals, suggestion));
            },
            ConfirmIcon {}
        }
    }
}

async fn create_task(signals: BoardSignals, suggestion: SuggestionSignals) {
    let task_data = shared_models::NewTaskData {
        title: suggestion.title.read().clone(),
        description: suggestion.description.read().clone(),
        due: None,
        status: TaskStatus::ToDo,
        assignees: suggestion.assignees.read().clone(),
        tags: suggestion.tags.read().clone(),
        new_tags: suggestion.new_tags.read().clone(),
    };
    if let Ok(task_id) = requests::create_task(signals.url, signals.board, task_data).await {
        log::info!("created task: {task_id}");
        requests::board(signals).await;
    }
}

#[component]
fn DeleteTaskButton(suggestion_id: usize, resolved_suggestions: Signal<HashSet<usize>>) -> Element {
    let style = "
        rounded-md
        border border-red-600
        stroke-red-600
        active:bg-red-600
        sm:hover:bg-red-600 sm:hover:stroke-white
    ";
    rsx! {
        button {
            aria_label: "delete task",
            class: "size-7 {style}",
            onclick: move |_| {
                let mut resolved_suggestions = resolved_suggestions.write();
                resolved_suggestions.insert(suggestion_id);
            },
            CancelIcon {}
        }
    }
}

#[component]
fn TagIcon(name: String, color: Color) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let color = match color {
        Color::Black => theme.color1_button,
        Color::White => theme.color2_button,
        Color::Gray => theme.color3_button,
        Color::Silver => theme.color4_button,
        Color::Maroon => theme.color5_button,
        Color::Red => theme.color6_button,
        Color::Purple => theme.color7_button,
        Color::Fushsia => theme.color8_button,
        Color::Green => theme.color9_button,
        Color::Lime => theme.color10_button,
        Color::Olive => theme.color11_button,
        Color::Yellow => theme.color12_button,
        Color::Navy => theme.color13_button,
        Color::Blue => theme.color14_button,
        Color::Teal => theme.color15_button,
        Color::Aqua => theme.color16_button,
    };
    let style = "rounded border-2";
    rsx! {
        div {
            class: "
                group
                flex flex-row items-center
                px-1.5 py-0.5
                {style} {color}
            ",
            {name}
        }
    }
}

#[component]
fn Description(suggestion_id: usize, description: Signal<String>) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            DescriptionForm { suggestion_id, editing, description }
        } else {
            DescriptionShow { editing, description }
        }
    }
}

#[component]
fn DescriptionForm(
    suggestion_id: usize,
    editing: Signal<bool>,
    description: Signal<String>,
) -> Element {
    rsx! {
        form {
            "aria-label": "update description",
            class: "flex flex-col gap-2",
            onsubmit: move |event| {
                description.set(event.values()["Description"].as_value());
                editing.set(false);
            },
            DescriptionInput  {
                id: "suggestion-{suggestion_id}-description-input",
                editing,
                description,
            },
            div {
                class: "flex flex-row gap-2 items-center justify-center",
                ConfirmButton { label: "set description" }
                CancelButton { label: "cancel description update", editing }
            }
        }
    }
}

#[component]
fn DescriptionShow(editing: Signal<bool>, description: String) -> Element {
    rsx! {
        section {
            "aria-label": "description",
            class: "flex flex-col gap-2",
            DescriptionContent { description }
            div {
                class: "flex flex-row justify-center",
                EditDescriptionButton { editing }
            }
        }
    }
}

#[component]
fn EditDescriptionButton(editing: Signal<bool>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("rounded border {}", theme.button);
    rsx! {
        button {
            "aria-label": "edit description",
            class: "
                group
                flex flex-row justify-center items-center
                py-1 px-6
                {style}
            ",
            onclick: move |_| editing.set(true),
            div {
                class: "relative",
                div { class: "size-5", EditIcon {} }
                Tooltip { content: "Edit Description", position: "-top-12 -left-10" }
            }
        }
    }
}

#[component]
fn DescriptionContent(description: String) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "p-4 rounded border whitespace-pre-wrap break-words {} {}",
        theme.bg_color_1, theme.border_color
    );
    rsx! {
        div {
            class: style,
            for block in parse_blocks(&description) {
                match block {
                    Block::Text(text) => rsx!{
                        p { {text} }
                    },
                    Block::Bullet(lines) => rsx!{
                        ul {
                            class:" list-disc list-inside",
                            for line in lines {
                                Bullet { line }
                            }
                        }
                    },
                    Block::Checkbox(lines) => rsx!{
                        ul {
                            for line in lines {
                                Checkbox { line }
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn Checkbox(line: Line) -> Element {
    line.content.drain(..5);
    rsx! {
        li {
            label {
                input {
                    disabled: true,
                    checked: false,
                    r#type: "checkbox",
                }
                {line.content}
            }
        }
    }
}

#[component]
fn Bullet(line: String) -> Element {
    line.drain(..2);
    rsx! { li { {line} } }
}

#[component]
fn ChatGptError(chat_gpt_response: Signal<Option<ChatGptResponse>>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2 items-center justify-center",
            h2 {
                class: "text-xl font-bold",
                "ChatGPT Error"
            },
            p {
                class: "text-sm",
                "An error occurred while trying to connect to Chat GPT. Please try again later."
            }
        }
    }
}

#[component]
fn ChatGptPromptInput(chat_gpt_response: Signal<Option<ChatGptResponse>>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = theme.text_color;
    let url = use_context::<Signal<UnloadUrl>>();
    let board = use_context::<Signal<Board>>();
    let num_calls_left = use_context::<Signal<NumChatGptCalls>>();
    let num_calls_left_ = num_calls_left.read().0;
    rsx! {
        div {
            class: "flex flex-col gap-2 items-center justify-center {style}",
            p {
                class: "text-xl font-bold",
                "Use ChatGPT ({num_calls_left_} daily attempts left)"
            }
            p {
                class: "text-sm",
                "or pick one from the suggestions below:"
            }
            PromptSuggestions { chat_gpt_response }
            form {
                id: "chat-gpt-prompt-form",
                "aria-label": "chat gpt prompt",
                onsubmit: move |event| {
                    let prompt = event.values()["Prompt:"].as_value();
                    spawn_forever(requests::send_chat_gpt_prompt(
                        board.read().board_name.clone(),
                        url,
                        prompt,
                        chat_gpt_response,
                        num_calls_left,
                    ));
                },
                div {
                    class: "flex flex-row gap-2 items-center justify-start",
                    TextInput {
                        id: "chat-gpt-prompt" ,
                        label: "Prompt:",
                    }
                }
            }
        }
    }
}

#[component]
fn PromptSuggestions(chat_gpt_response: Signal<Option<ChatGptResponse>>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "rounded-lg border divide-y {} {}",
        theme.border_color, theme.divide_color
    );
    rsx! {
        ul {
            class: "w-full {style}",
            PromptSuggestion { prompt: "suggest cupcake recipe", chat_gpt_response }
            PromptSuggestion { prompt: "paint bedroom", chat_gpt_response }
            PromptSuggestion { prompt: "friends over for BBQ", chat_gpt_response }
            PromptSuggestion { prompt: "prepare for Rome vacation", chat_gpt_response }
            PromptSuggestion { prompt: "house tidy", chat_gpt_response }
            PromptSuggestion { prompt: "fix fence", chat_gpt_response }
        }
    }
}

#[component]
fn PromptSuggestion(prompt: String, chat_gpt_response: Signal<Option<ChatGptResponse>>) -> Element {
    let url = use_context::<Signal<UnloadUrl>>();
    let p = prompt.clone();
    let board = use_context::<Signal<Board>>();
    let num_calls_left = use_context::<Signal<NumChatGptCalls>>();
    rsx! {
        li {
            button {
                class: "w-full text-lg",
                onclick: move |_| {
                    spawn_forever(requests::send_chat_gpt_prompt(
                        board.read().board_name.clone(),
                        url,
                        p.clone(),
                        chat_gpt_response,
                        num_calls_left,
                    ));
                },
                {prompt}
            }
        }
    }
}

async fn add_suggestion_assignee(
    signals: BoardSignals,
    users: Signal<Users>,
    mut assignees: Signal<Vec<UserId>>,
    user_id: UserId,
) {
    requests::board(signals).await;
    if users.read().0.contains_key(&user_id) {
        assignees.write().push(user_id);
    }
}

async fn add_suggestion_tag(
    signals: BoardSignals,
    tags: Signal<Tags>,
    mut assigned_tags: Signal<Vec<TagId>>,
    tag_id: TagId,
) {
    requests::board(signals).await;
    if tags.read().0.contains_key(&tag_id) {
        assigned_tags.write().push(tag_id);
    }
}
