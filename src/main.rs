use clap::{Parser, ValueEnum};
mod models;
mod render_cmd;
mod update_cmd;
use std::path::Path;
use tokio::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(value_enum)]
    command: Commands,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Commands {
    Update,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = Args::parse();
    // Glob through all .toml files in data folder
    let data_path = Path::new("data/data.toml");
    let example_path = Path::new("doc/example.toml");
    let path = if data_path.exists() {
        data_path
    } else {
        example_path
    };
    // Update all the entries to give them an id
    let database = update_cmd::assign_ids(path).await.unwrap();
    let result = render_cmd::render_database(database).await;
    // Write to result to dist target folder
    fs::write("target/graph.txt", result.as_bytes()).await?;
    println!("{}", result);
    Ok(())
}
