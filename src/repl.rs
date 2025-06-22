use crate::ollama_client::OllamaClient;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

pub struct Repl {
    editor: DefaultEditor,
    client: OllamaClient,
    history: Vec<String>,
}

impl Repl {
    pub fn new(client: OllamaClient) -> Self {
        Self {
            editor: DefaultEditor::new().unwrap(),
            client,
            history: Vec::new(),
        }
    }

    pub async fn run(&mut self) {
        println!("Type your prompt. Enter an empty line to send. Type :help for help.");
        let mut buffer = String::new();
        loop {
            match self.editor.readline("> ") {
                Ok(line) => {
                    let trimmed = line.trim();
                    if trimmed == ":exit" {
                        break;
                    } else if trimmed == ":help" {
                        println!("Enter your message. Use an empty line to submit. :exit to quit.");
                    } else if trimmed.is_empty() {
                        if !buffer.trim().is_empty() {
                            println!("Sending prompt...");
                            self.history.push(buffer.clone());
                            self.client.stream_prompt(buffer.clone()).await;
                            buffer.clear();
                        }
                    } else {
                        buffer.push_str(&line);
                        buffer.push('\n');
                    }
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
                Err(e) => {
                    eprintln!("REPL error: {e}");
                    break;
                }
            }
        }
    }
}
