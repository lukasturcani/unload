use clap::Parser;
use frontend::translations;
use openai_api_rs::v1::{
    api::OpenAIClient,
    chat_completion::{ChatCompletionMessage, ChatCompletionRequest, Content, MessageRole},
    common::GPT4_O,
};
use shared_models::SupportedLanguage;

#[derive(Parser)]
struct Cli {
    openai_api_key: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = OpenAIClient::new(cli.openai_api_key);
    let english = translations::Translation::from(SupportedLanguage::English)
        .to_json()
        .to_string();
    let completion_request = ChatCompletionRequest::new(
        GPT4_O.to_string(),
        vec![
            ChatCompletionMessage {
                role: MessageRole::system,
                content: Content::Text(
                    format!("Translate the following text into {}. \
                    It is for a to-do list app. It consists of tasks which are placed in 3 columns, to do \
                    in progress and done. \
                    \n```json\n{english}\n```", SupportedLanguage::Slovak.name()),
                ),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ],
    );
    let choices = client
        .chat_completion(completion_request)
        .await
        .unwrap()
        .choices;
    let choice = choices.first().unwrap();
    let content = choice.message.content.as_ref().unwrap();
    println!("{content}");
}
