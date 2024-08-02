use dioxus::prelude::*;
use shared_models::TaskSuggestion;

use crate::{
    components::input::TextInput,
    model::UnloadUrl,
    pages::board::{model::ChatGptResponse, requests},
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
                    spawn_forever(requests::send_chat_gpt_prompt(url, prompt, chat_gpt_response));
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
    rsx! {
        li {
            button {
                class: "w-full",
                onclick: move |_| {
                    spawn_forever(requests::send_chat_gpt_prompt(url, p.clone(), chat_gpt_response));
                },
                {prompt}
            }
        }
    }
}
