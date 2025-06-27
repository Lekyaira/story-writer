use clap::Parser;
use story_writer::cli::Cli;
use story_writer::config;
use story_writer::ollama_client;
use story_writer::agent;
use std::fs;
use std::path::PathBuf;
use serde_json;

pub fn read_idea(path: Option<PathBuf>) -> Result<Option<String>, String> {
    if let Some(path) = path {
        match fs::read_to_string(&path) {
            Ok(contents) => Ok(Some(contents)),
            Err(e) => Err(format!("Failed to read idea file {}: {}", path.display(), e)),
        }
    } else {
        Ok(None)
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    // Parse CLI arguments
    let cli = Cli::parse();
    let idea_contents = read_idea(cli.idea).unwrap();

    // Load configuration
    let cfg = config::Config::load_with_overrides(cli.host, cli.port, cli.model);

    // Initialize Ollama client
    let mut client = ollama_client::OllamaClient::new(cfg.host.clone(), cfg.port, cfg.model);

    // Initialize agent
    let mut agent = agent::Agent::new(client);

    // If there is an idea, use it to generate a story   
    if let Some(idea_contents) = idea_contents {
        // Pull all characters from the idea
        println!("Parsing characters...");
        let mut characters = agent.parse_characters(idea_contents.clone()).await?;
        // Fill out each character's information
        for character in characters.iter_mut() {
            println!("Parsing character: {}", character.name);
            let filled_character = agent.parse_character(idea_contents.clone(), character.clone()).await?;
            *character = filled_character;
        }
        println!("{}", serde_json::to_string_pretty(&characters).unwrap());
    } else {
        // Otherwise, generate a story from scratch
        // TODO: Implement this
        todo!()
    }
    Ok(())
}
