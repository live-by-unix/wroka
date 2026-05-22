mod engine;
mod plugins;
mod yaml;

use anyhow::Result;
use colored::*;
use engine::Engine;
use std::env;
use yaml::load_yaml;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "--version" {
        println!("WROKA 1.0.0");
        return Ok(());
    }

    if args.len() != 3 || args[1] != "deploy" {
        eprintln!("{}", "Usage: wroka deploy <file.yml>".red());
        std::process::exit(1);
    }

    let config = load_yaml(&args[2]).await?;
    let mut engine = Engine::new();
    engine.run(config).await?;

    Ok(())
}
