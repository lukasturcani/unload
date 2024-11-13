use clap::Parser;
use frontend::translations;
use openai_api_rs::v1::{
    api::OpenAIClient,
    batch::CreateBatchRequest,
    chat_completion::{ChatCompletionMessage, ChatCompletionRequest, Content, MessageRole},
    common::GPT4_O,
    file::FileUploadRequest,
};
use serde::Serialize;
use serde_json::json;
use shared_models::{IntoEnumIterator, SupportedLanguage};
use std::io::Write;

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
    let client = OpenAIClient::builder()
        .with_api_key(cli.openai_api_key)
        .build()
        .expect("failed to create OpenAI client");
    let english =
        serde_json::to_string(&translations::Translation::from(SupportedLanguage::English))
            .unwrap();
    let mut file = tempfile::NamedTempFile::new().unwrap();

    let batch_request_content = SupportedLanguage::iter()
        .filter(|&l| l != SupportedLanguage::English)
        .map(|l| {
            (
                l,
                format!(
                    "Translate the following text into {} ({}). \
                        It is for a to-do list app. It consists of tasks which are placed in 3 columns, to do \
                        in progress and done. \
                    \n```json\n{}\n```",
                    l.id(),
                    l.name(),
                    english
                ),
            )
        })
        .map(|(l, s)| BatchFileLine {
            custom_id: format!("frontend-{}-request", l.name()),
            method: "POST".to_string(),
            url: "/v1/chat/completions".into(),
            body: ChatCompletionRequest::new(
                GPT4_O.to_string(),
                vec![ChatCompletionMessage {
                    role: MessageRole::system,
                    content: Content::Text(s),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                }],
            )
            .response_format(json!({"type": "json_object"})),
        })
        .map(|l| serde_json::to_string(&l).unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    file.write_all(batch_request_content.as_bytes()).unwrap();
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
