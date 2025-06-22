# PRD: CLI AI Chat Application Using Ollama

## 1. Introduction / Overview

This document outlines the requirements for a command-line interface (CLI) AI chat application written in Rust, using the `ollama-rs` crate to interact with a locally running Ollama model server. The tool will launch a REPL session, allowing the user to input messages and receive streamed AI-generated responses. Chat history will persist only in memory during the session and be lost afterward.

The primary goal is to create a lightweight and minimal REPL-based AI chat tool for personal learning, experimentation, and future extensibility.

## 2. Goals

- Create a functional CLI chat REPL using Rust and `ollama-rs`.
- Stream model responses in real time.
- Use `.env` file or environment variables for configuration.
- Maintain in-memory chat history during each session.
- Provide basic but informative error handling.

## 3. User Stories

- As a developer, I want to run a CLI tool that opens a chat session with a local AI model so that I can experiment and test ideas quickly.
- As a user, I want streamed model output so that I can start reading the response immediately without waiting for the full answer.
- As a user, I want the application to guide me if required environment variables are missing so that I can fix the issue without checking documentation.
- As a user, I want to write messages across multiple lines in the REPL so that I can submit code or detailed prompts.

## 4. Functional Requirements

1. The application must launch an interactive REPL session when run.
2. The REPL must support multi-line user input, submitted with an explicit delimiter (e.g., double newline or special command).
3. The application must interface with the Ollama model server via the `ollama-rs` crate.
4. The application must retrieve the Ollama server host and port from environment variables using `dotenvy`.
5. If the required environment variables are missing, the application must prompt the user to supply them via environment variables or `.env` file.
6. The REPL must stream the model's output to the terminal as it is received.
7. Chat history must be stored in memory only and not persisted to disk.
8. The application must print error messages to `stderr` for network or model errors.

## 5. Non-Goals (Out of Scope)

- Persistent chat history or session saving/loading
- Support for multiple models or model switching
- Plugin system or extensibility
- Fancy UI or input editing features (beyond multi-line support)
- Authentication or remote inference

## 6. Design Considerations (Optional)

- Consider using `rustyline` or a similar library to handle REPL input with editing/multi-line support.
- Consider printing a message when the session starts (e.g., "Type your prompt and press Enter. Use CTRL+D or `:exit` to quit.").
- Allowing a command like `:help` or `:env` could make development debugging easier.

## 7. Technical Considerations (Optional)

- Use the `dotenvy` crate to load `.env` files.
- Use the `ollama-rs` crate for model communication.
- Stream output should be handled via async streaming, possibly with `tokio`.
- Error messages should be colored (e.g., using `ansi_term`) for visibility in `stderr`.

## 8. Success Metrics

- The application launches and connects to the Ollama model using environment configuration.
- The REPL works as expected for multiple inputs and provides streamed responses.
- Missing or incorrect configuration is caught and reported helpfully.
- No panics or unhandled errors in standard use cases.

## 9. Open Questions

- What syntax or command should end multi-line input in the REPL? (`Ctrl+D`, `:end`, double newline, etc.)
- Should future versions consider adding support for saving history or configuration profiles?
