use std::env;
use std::error::Error;

/// Application configuration loaded from environment variables
pub struct Config {
    pub host: String,
    pub port: u16,
    pub model: String,
    pub reasoning: bool,
}

impl Config {
    /// Load configuration from environment variables using `dotenvy`.
    pub fn load() -> Result<Self, Box<dyn Error>> {
        dotenvy::dotenv().ok();
        let host = env::var("OLLAMA_HOST")?;
        let port: u16 = env::var("OLLAMA_PORT")?.parse()?;
        let model = env::var("OLLAMA_MODEL")?;
        let reasoning = env::var("OLLAMA_REASONING")
            .map_or(false, |v| v == "true");
        Ok(Self { host, port, model, reasoning })
    }

    /// Load configuration, allowing CLI overrides for host, port, and model.
    pub fn load_with_overrides(host: Option<String>, port: Option<u16>, model: Option<String>, reasoning: Option<bool>) -> Self {
        dotenvy::dotenv().ok();
        let host = host.or_else(|| env::var("OLLAMA_HOST").ok())
            .unwrap_or_else(|| Self::exit_with_msg("OLLAMA_HOST"));
        let port = port.or_else(|| env::var("OLLAMA_PORT").ok().and_then(|p| p.parse().ok()))
            .unwrap_or_else(|| Self::exit_with_msg("OLLAMA_PORT"));
        let model = model.or_else(|| env::var("OLLAMA_MODEL").ok())
            .unwrap_or_else(|| Self::exit_with_msg("OLLAMA_MODEL"));
        let reasoning = reasoning.unwrap_or(env::var("OLLAMA_REASONING")
            .map_or(false, |v| v == "true"));
        Self { host, port, model, reasoning }
    }

    fn exit_with_msg(var: &str) -> ! {
        use ansi_term::Colour::Red;
        eprintln!(
            "{}",
            Red.paint(format!(
                "Missing or invalid configuration. Set {var} via CLI or environment variable."
            ))
        );
        std::process::exit(1);
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
