use clap::Parser;
use openai_api_rs::v1::{
    api::OpenAIClient,
    batch::CreateBatchRequest,
    chat_completion::{ChatCompletionMessage, ChatCompletionRequest, Content, MessageRole},
    common::GPT4_O,
    file::FileUploadRequest,
};
use serde::Serialize;
use shared_models::{IntoEnumIterator, SupportedLanguage};
use std::io::Write;
use website::translations;

#[derive(Parser)]
struct Cli {
    openai_api_key: String,
}

#[derive(Serialize)]
struct BatchFileLine {
    custom_id: String,
    method: String,
    url: String,
    body: ChatCompletionRequest,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = OpenAIClient::new(cli.openai_api_key);
    let english = translations::Translation::from(SupportedLanguage::English)
        .to_json()
        .to_string();
    let mut file = tempfile::NamedTempFile::new().unwrap();

    let batch_request_content = SupportedLanguage::iter()
        .filter(|&l| l != SupportedLanguage::English)
        .map(|l| {
            (
                l,
                format!(
                    "Translate the following text into {}. \
                        It is for landing page of website of a to-do list app. \
                        It consists of tasks which are placed in 3 columns, to do \
                        in progress and done. \
                    \n```json\n{}\n```",
                    l.name(),
                    english
                ),
            )
        })
        .map(|(l, s)| BatchFileLine {
            custom_id: format!("website-{}-request", l.name()),
            method: "POST".to_string(),
            url: format!("/v1/chat/completions"),
            body: ChatCompletionRequest::new(
                GPT4_O.to_string(),
                vec![ChatCompletionMessage {
                    role: MessageRole::system,
                    content: Content::Text(s),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                }],
            ),
        })
        .map(|l| serde_json::to_string(&l).unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    file.write(batch_request_content.as_bytes()).unwrap();
    let file_upload_request = FileUploadRequest::new(
        file.path().to_str().unwrap().to_string(),
        "batch".to_string(),
    );
    let file_upload_result = client.upload_file(file_upload_request).await.unwrap();
    println!("File id: {:?}", file_upload_result.id);

    let input_file_id = file_upload_result.id;
    let batch_request = CreateBatchRequest::new(
        input_file_id,
        "/v1/chat/completions".to_string(),
        "24h".to_string(),
    );
    let batch_result = client.create_batch(batch_request).await.unwrap();
    let batch_id = batch_result.id;
    println!("Batch id: {:?}", batch_id);
}
