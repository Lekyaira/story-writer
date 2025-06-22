use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let cfg = config::Config::load_with_overrides(cli.host, cli.port, cli.model);
    let client = ollama_client::OllamaClient::new(cfg.host.clone(), cfg.port, cfg.model);
    agent::agent_action(&mut client, "What is the capital of the moon?".to_string()).await;
}
