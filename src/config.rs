use std::env;
use std::error::Error;

/// Application configuration loaded from environment variables
pub struct Config {
    pub host: String,
    pub port: u16,
    pub model: String,
}

impl Config {
    /// Load configuration from environment variables using `dotenvy`.
    pub fn load() -> Result<Self, Box<dyn Error>> {
        dotenvy::dotenv().ok();
        let host = env::var("OLLAMA_HOST")?;
        let port: u16 = env::var("OLLAMA_PORT")?.parse()?;
        let model = env::var("OLLAMA_MODEL")?;
        Ok(Self { host, port, model })
    }

    /// Load the configuration or print a helpful message and exit on failure.
    pub fn load_or_exit() -> Self {
        match Self::load() {
            Ok(cfg) => cfg,
            Err(err) => {
                use ansi_term::Colour::Red;
                eprintln!(
                    "{}",
                    Red.paint(format!(
                        "Missing or invalid configuration: {err}.\nSet OLLAMA_HOST, OLLAMA_PORT, and OLLAMA_MODEL in your environment or .env file."
                    ))
                );
                std::process::exit(1);
            }
        }
    }
}
