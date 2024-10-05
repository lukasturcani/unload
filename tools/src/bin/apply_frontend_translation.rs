use clap::Parser;
use openai_api_rs::v1::api::OpenAIClient;
use serde_json::Value;
use std::str;

#[derive(Parser)]
struct Cli {
    batch_id: String,
    openai_api_key: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = OpenAIClient::new(cli.openai_api_key);
    let batch = client.retrieve_batch(cli.batch_id).await.unwrap();
    let file_id = batch.output_file_id.unwrap();
    let content = client.retrieve_file_content(file_id).await.unwrap();
    let content = str::from_utf8(&content).unwrap();
    let values = content
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect::<Vec<Value>>();
    println!("{:?}", values);
}
