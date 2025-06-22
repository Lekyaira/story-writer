mod config;
mod ollama_client;
mod repl;
mod cli;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let cfg = config::Config::load_with_overrides(cli.host, cli.port, cli.model);
    let client = ollama_client::OllamaClient::new(cfg.host.clone(), cfg.port, cfg.model);
    let mut repl = repl::Repl::new(client);
    repl.run().await;
}
