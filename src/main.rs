mod config;
mod ollama_client;
mod repl;

#[tokio::main]
async fn main() {
    let cfg = config::Config::load_or_exit();
    let client = ollama_client::OllamaClient::new(cfg.host.clone(), cfg.port, cfg.model);
    let mut repl = repl::Repl::new(client);
    repl.run().await;
}
