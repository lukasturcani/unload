use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use sqlx::SqlitePool;

#[derive(Parser)]
struct Args {
    database_url: String,
    nouns: PathBuf,
    adjectives: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let pool = SqlitePool::connect(&args.database_url).await?;
    let nouns = {
        let mut nouns = String::new();
        File::open(args.nouns)?.read_to_string(&mut nouns)?;
        nouns
    };
    let adjectives = {
        let mut adjectives = String::new();
        File::open(args.adjectives)?.read_to_string(&mut adjectives)?;
        adjectives
    };
    let mut tx = pool.begin().await?;
    for noun in nouns.split_whitespace() {
        sqlx::query("INSERT INTO nouns (noun) VALUES (?)")
            .bind(noun)
            .execute(&mut *tx)
            .await?;
    }
    for adjective in adjectives.split_whitespace() {
        sqlx::query("INSERT INTO adjectives (adjective) VALUES (?)")
            .bind(adjective)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(())
}
