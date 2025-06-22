# AI Chat CLI

A simple command-line chat application that connects to a local Ollama model server.

## Setup

1. Copy `.env.example` to `.env` and set `OLLAMA_HOST` and `OLLAMA_PORT` for your Ollama server.
2. Build and run the application using Cargo:
   ```bash
   cargo run
   ```

During startup the application loads configuration from the environment and starts an interactive REPL. Enter your prompt across multiple lines and submit with an empty line. Use `:exit` to quit and `:help` for instructions.
