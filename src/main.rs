use clap::{Parser, ValueEnum};
mod models;
use std::fs::File;
use std::io::Read;

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
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", format!("{:?}", args.command));
    }
    let mut file = File::open("data/example.toml")?;

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the TOML string into a MyConfig struct
    let config: models::TomlFile = toml::from_str(&contents)?;

    // Print the config details
    println!("{:#?}", config);
    Ok(())
}
