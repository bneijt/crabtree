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
    #[arg(value_enum, default_value_t = Commands::Update)]
    command: Commands,

    /// Number of times to greet
    #[arg(short, long, default_value_t = String::from("data/data.toml"))]
    database: String,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Commands {
    Update,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // Glob through all .toml files in data folder
    let data_path = Path::new(args.database.as_str());
    // Update all the entries to give them an id
    _ = update_cmd::assign_ids(data_path).await.unwrap();
    let database = update_cmd::load_database(data_path).await.unwrap();
    let result = render_cmd::render_database(database).await;
    // Write to result to dist target folder
    fs::write("target/graph.txt", result.as_bytes()).await?;
    println!("{}", result);
    Ok(())
}
