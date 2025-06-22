use crate::ollama_client::OllamaClient;

#[tokio::test]
async fn stream_prompt_handles_error() {
    let client = OllamaClient::new("127.0.0.1".to_string(), 1); // unlikely port
    client.stream_prompt("test".into()).await; // should not panic even if fails
}
