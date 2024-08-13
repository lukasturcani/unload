use std::collections::HashMap;

use dioxus::prelude::*;
use shared_models::{Color, TagData, TaskSuggestion};

use crate::{
    components::{
        icons::{CancelIcon, ConfirmIcon},
        input::TextInput,
    },
    description_parser::{parse_blocks, Block, Line},
    model::UnloadUrl,
    pages::board::{
        model::{Board, ChatGptResponse, Tags},
        requests,
    },
    themes::Theme,
};

#[component]
pub fn ChatGpt() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "rounded-lg border max-h-full overflow-y-auto {} {}",
        theme.bg_color_1, theme.border_color
    );
    let chat_gpt_response = use_signal(|| None);
    rsx! {
        div {
            class: "p-5 {style}",
            onclick: |event| event.stop_propagation(),
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
                Some(ChatGptResponse::Error) => rsx! {
                    ChatGptError { chat_gpt_response }
                },
                None => rsx! {
                    ChatGptPromptInput { chat_gpt_response }
                }
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct ProcessedTaskSuggestion {
    title: String,
    description: String,
    tags: Vec<TagData>,
    new_tags: Vec<TagData>,
}

#[component]
fn ChatGptSuggestions(
    suggestions: Vec<TaskSuggestion>,
    chat_gpt_response: Signal<Option<ChatGptResponse>>,
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
    let tags = use_context::<Signal<Tags>>();
    let tags = &tags.read().0;
    let name_to_color = tags.values().fold(HashMap::new(), |mut map, tag| {
        map.insert(&tag.name, tag.color);
        map
    });
    let mut processed_suggestions = Vec::with_capacity(suggestions.len());
    for suggestion in suggestions {
        let mut tags = Vec::new();
        let mut new_tags = Vec::new();
        for name in suggestion.tags {
            match name_to_color.get(&name) {
                Some(color) => tags.push(TagData {
                    name,
                    color: *color,
                }),
                None => {
                    let color =
                        colors[(name.bytes().fold(0, |acc, x| acc + (x as u16)) % 16) as usize];
                    new_tags.push(TagData { name, color });
                }
            }
        }
        processed_suggestions.push(ProcessedTaskSuggestion {
            title: suggestion.title,
            description: suggestion.description,
            tags,
            new_tags,
        });
    }
    rsx! {
        div {
            class: "
                flex flex-col gap-2 items-center
                max-h-full overflow-y-auto
            ",
            for suggestion in processed_suggestions {
                TaskSuggestionCard {
                    key: "{suggestion.title}",
                    suggestion
                }
            }
        }
    }
}

#[component]
fn TaskSuggestionCard(suggestion: ProcessedTaskSuggestion) -> Element {
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
    let label = suggestion.title.clone();

    rsx! {
        article {
            aria_label: label,
            class: "flex flex-col gap-2 p-2.5 {style}",
            div {
                class: "flex flex-row gap-2 items-center justify-start",
                h3 {
                    class: "
                        text-lg sm:text-xl
                        font-bold tracking-tight
                    ",
                    {suggestion.title}
                },
                AddTaskButton {}
                DeleteTaskButton {}
            }
            Description {
                description: suggestion.description,
            },
            div {
                class: "flex flex-row gap-2 items-center justify-start",
                for tag in suggestion.tags {
                    TagIcon {
                        name: tag.name,
                        color: tag.color
                    }
                }
                for tag in suggestion.new_tags {
                    TagIcon {
                        name: tag.name,
                        color: tag.color
                    }
                }
            }
        }
    }
}

#[component]
fn AddTaskButton() -> Element {
    let style = "
        rounded-md
        border border-green-500
        stroke-green-500
        active:bg-green-500
        sm:hover:bg-green-500 sm:hover:stroke-white
    ";
    rsx! {
        button {
            aria_label: "add task",
            class: "size-7 {style}",
            ConfirmIcon {}
        }
    }
}

#[component]
fn DeleteTaskButton() -> Element {
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
fn Description(description: String) -> Element {
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
    rsx! {
        div {
            class: "flex flex-col gap-2 items-center justify-center {style}",
            p {
                class: "text-xl font-bold",
                "Use ChatGPT"
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
                    ));
                },
                {prompt}
            }
        }
    }
}
