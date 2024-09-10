use clap::{Parser, ValueEnum};
mod update_cmd;
mod models;
use std::path::Path;

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

    // Update all the entries to give them an id
    let _ = update_cmd::assign_ids(Path::new("data/example.toml")).await;

    Ok(())
}
