# Story Writer

A Rust CLI tool for AI-assisted story writing, character extraction, and story analysis using a local Ollama model server.

## Features

- **CLI interface** with argument parsing via [clap]
- **Configurable** via CLI or environment variables (`.env` supported)
- **Ollama integration** for AI-powered story and character analysis
- **Character extraction**: Parse main characters from a story idea file (JSON output)
- **Custom derive macros** for unique ID generation (`HasId`)
- **SQLite scaffolding** for future persistence
- **Extensible agent module** for prompt/response workflows

## Setup

### General Instructions

1. **Install Rust** (if not already):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Clone the repository:**
   ```sh
   git clone <repo-url>
   cd story-writer
   ```
3. **Set up environment variables:**
   - Copy `.env.example` to `.env` and set `OLLAMA_HOST`, `OLLAMA_PORT`, and `OLLAMA_MODEL` for your local Ollama server.

4. **Build the project:**
   ```sh
   cargo build
   ```

### Nix Setup Instructions

1. **Clone the repository:**
    ```sh
    git clone <repo-url>
    cd story-writer
    ```
2. **Use the Nix shell:**
    ```sh
    nix-shell
    ```
    *or*
    ```sh
    echo "use nix" > .envrc && direnv allow
    ```
3. **Build the project:**
    ```sh
    cargo build
    ```

## Usage

### Extract Characters from a Story Idea

Given a text file (e.g., `story_idea.txt`) containing your story idea, snippet, or draft:

```sh
cargo run -- --idea story_idea.txt
```

- The tool will parse the file, send it to the model, and print a JSON array of main characters.
- Each character is assigned a unique hash-based ID after parsing.

### CLI Options

```
story-writer [OPTIONS]

Options:
      --host <HOST>    Ollama host (overrides OLLAMA_HOST)
      --port <PORT>    Ollama port (overrides OLLAMA_PORT)
      --model <MODEL>  Ollama model (overrides OLLAMA_MODEL)
      --idea <FILE>    Path to a story idea file to analyze
  -h, --help           Print help
  -V, --version        Print version
```

### Example Output

```json
[
  {
    "id": "...",
    "main": true,
    "name": "Alice",
    "physical_description": "Tall, brown hair...",
    ...
  },
  ...
]
```

## Requirements

- Rust (2024 edition)
- Local [Ollama](https://ollama.com/) server running
- Host, port, and model specified in `.env`, environment variables, or via CLI

## Future Features

- Create a full story structure, including data for characters, setting, and plot.
- Plan story arcs and scenes, from loose ideas, story snippet, or draft.
- Edit the structure and plan.
- Query the agent about story structure and plans.
- Produce test prose, outlines, story bible, etc..
- Autonomously write the story based on the generated structure.
- Edit, revise, and prompt changes to the draft. Iterate through revisions.