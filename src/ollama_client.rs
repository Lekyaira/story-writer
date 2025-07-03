use ansi_term::Colour::Red;
use ollama_rs::Ollama;
use ollama_rs::generation::chat::{ChatMessage, request::ChatMessageRequest};
use tokio_stream::{Stream, StreamExt};

// TODO: Coerce model into reasoning using chain of thought if not a reasoning model.

pub struct OllamaClient {
    inner: Ollama,
    model: String,
    reasoning: bool,
    thinking: bool,
}

impl OllamaClient {
    pub fn new(host: String, port: u16, model: String, reasoning: bool) -> Self {
        let url = format!("http://{host}");
        Self {
            inner: Ollama::new(url, port),
            model,
            reasoning,
            thinking: false,
        }
    }

    /// Send a prompt and stream the model response to stdout.
    pub async fn stream_prompt(&mut self, prompt: String) {
        let req = ChatMessageRequest::new(self.model.clone(), vec![ChatMessage::user(prompt)]);

        match self.inner.send_chat_messages_stream(req).await {
            Ok(mut stream) => {
                while let Some(chunk) = stream.next().await {
                    if let Ok(resp) = chunk {
                        // If the model is thinking, tell the user and don't print <think>
                        // messages.
                        // TODO: Put thinking messages in log so they can be examined.
                        if resp.message.content.contains("<think>") {
                            self.thinking = true;
                            print!("Thinking...")
                        }
                        if self.thinking {
                            if resp.message.content.contains("</think>") {
                                self.thinking = false;
                            }
                        } else {
                            print!("{}", resp.message.content);
                        }
                    }
                }
                println!();
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    Red.paint(format!("Failed to communicate with Ollama: {e}"))
                );
            }
        }
    }

    /// Send a prompt and get a response from the model.
    pub async fn get_response(&mut self, prompt: String) -> String {
        let req = ChatMessageRequest::new(self.model.clone(), vec![ChatMessage::user(prompt)]);
        let resp = self.inner.send_chat_messages(req).await.unwrap();
        resp.message.content
    }

    /// Send a prompt and stream the model response.
    pub async fn get_response_stream(&mut self, prompt: String) -> String {
        let req = ChatMessageRequest::new(self.model.clone(), vec![ChatMessage::user(prompt)]);
        let mut response_str = String::new();
        let mut thinking = false;
        match self.inner.send_chat_messages_stream(req).await {
            Ok(mut stream) => {
                while let Some(chunk) = stream.next().await {
                    if let Ok(resp) = chunk {
                        if resp.message.content.contains("<think>") {
                            thinking = true;
                        } else if resp.message.content.contains("</think>") {
                            thinking = false;
                            println!("");
                        }
                        if thinking {
                            print!("{}", resp.message.content);
                        } else {
                            response_str += &resp.message.content;
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!(
                    "{}",
                    Red.paint(format!("Failed to communicate with Ollama: {e}"))
                );
            }
        }
        response_str
    }
}
