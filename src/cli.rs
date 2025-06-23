use clap::Parser;
use std::path::PathBuf;

/// CLI arguments for overriding configuration
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Ollama host (overrides OLLAMA_HOST)
    #[arg(long)]
    pub host: Option<String>,
    /// Ollama port (overrides OLLAMA_PORT)
    #[arg(long)]
    pub port: Option<u16>,
    /// Ollama model (overrides OLLAMA_MODEL)
    #[arg(long)]
    pub model: Option<String>,
    /// Path to an idea file to load
    #[arg(long)]
    pub idea: Option<PathBuf>,
} 