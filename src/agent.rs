use crate::ollama_client::OllamaClient;
use regex::Regex;

pub async fn agent_action(client: &mut OllamaClient, prompt: String) {
    println!("Sending prompt to agent...");
    let response = client.get_response(prompt).await;
    // Remove <think>...</think> and their contents
    let re = Regex::new(r"<think>[\s\S]*?</think>").unwrap();
    let filtered = re.replace_all(&response, "");
    println!("Agent response: {}", filtered.trim());
} 