use clap::Parser;
use story_writer::cli::Cli;
use story_writer::config;
use story_writer::ollama_client;
use story_writer::agent;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let cfg = config::Config::load_with_overrides(cli.host, cli.port, cli.model);
    let mut client = ollama_client::OllamaClient::new(cfg.host.clone(), cfg.port, cfg.model);
    let response = agent::agent_action(&mut client, "What is the capital of the moon?".to_string()).await;
    println!("{response}");
}
