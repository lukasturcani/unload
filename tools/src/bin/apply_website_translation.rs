use clap::Parser;
use openai_api_rs::v1::api::OpenAIClient;
use serde_json::Value;
use std::{path::PathBuf, str};
use website::translations::Translation;

#[derive(Parser)]
struct Cli {
    batch_id: String,
    write_to: PathBuf,
    openai_api_key: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = OpenAIClient::builder()
        .with_api_key(cli.openai_api_key)
        .build()
        .expect("failed to create client");
    let batch = client.retrieve_batch(cli.batch_id).await.unwrap();
    let file_id = batch.output_file_id.unwrap();
    let content = client.retrieve_file_content(file_id).await.unwrap();
    let content = str::from_utf8(&content).unwrap();
    for value in content
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
    {
        let content = &value["response"]["body"]["choices"][0]["message"]["content"]
            .as_str()
            .unwrap();
        match serde_json::from_str::<Translation<String>>(content) {
            Ok(translation) => {
                std::fs::write(
                    cli.write_to.join(format!("{}.rs", translation.id)),
                    file_content(&translation),
                )
                .unwrap();
            }
            Err(e) => println!("failed: {e}"),
        }
    }
}

fn file_content(translation: &Translation<String>) -> String {
    format!(
        "use super::{{Text, Translation}};\n\npub const {}: Translation<&'static str> = {:#?};\n",
        translation.id.to_uppercase(),
        translation
    )
}
