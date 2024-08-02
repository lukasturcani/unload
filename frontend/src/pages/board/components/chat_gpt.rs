use dioxus::prelude::*;
use shared_models::TaskSuggestion;

use crate::{
    components::input::TextInput,
    model::UnloadUrl,
    pages::board::{
        model::{Board, ChatGptResponse},
        requests,
    },
    themes::Theme,
};

#[component]
pub fn ChatGpt() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "rounded-lg border {} {}",
        theme.bg_color_2, theme.border_color
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

#[component]
fn ChatGptSuggestions(
    suggestions: Vec<TaskSuggestion>,
    chat_gpt_response: Signal<Option<ChatGptResponse>>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2 items-center justify-center",
            for suggestion in suggestions {
                TaskSuggestionCard { suggestion }
            }
        }
    }
}

#[component]
fn TaskSuggestionCard(suggestion: TaskSuggestion) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "rounded-lg border {} {} {} {}",
        theme.border_color, theme.bg_color_2, theme.text_color, theme.bg_color_1
    );
    rsx! {
        div {
            class: "flex flex-col gap-2 p-2 {style}",
            h2 {
                class: "text-xl font-bold",
                {suggestion.title}
            },
            p {
                class: "text-sm",
                {suggestion.description}
            },
            div {
                class: "flex flex-row gap-2 items-center justify-start",
                for tag in suggestion.tags {
                    div {
                        class: "rounded-full bg-gray-200 px-2 py-1 text-xs",
                        {tag}
                    }
                }
            }
        }
    }
}

#[component]
fn ChatGptError(chat_gpt_response: Signal<Option<ChatGptResponse>>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2 items-center justify-center",
            h2 {
                class: "text-xl font-bold",
                "Chat GPT Error"
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
                "Tell ChatGPT to write some tasks for you, or pick one from the suggestions below:"
            }
            PromptSuggestions { chat_gpt_response }
            form {
                id: "chat-gpt-prompt-form",
                "aria-label": "chat gpt prompt",
                onsubmit: move |event| {
                    let prompt = event.values()["Make tasks for:"].as_value();
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
                        label: "Make tasks for:",
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
                class: "w-full",
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
