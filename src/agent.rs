use crate::ollama_client::OllamaClient;
use regex::Regex;
use crate::story::Character;
use serde::Deserialize;

pub struct Agent {
    client: OllamaClient,
}

impl Agent {
    pub fn new(client: OllamaClient) -> Self {
        Self { client }
    }
}
impl Agent {
    pub async fn action(&mut self, prompt: String) -> String {
        let response = self.client.get_response(prompt).await;
        // Remove <think>...</think> and their contents
        let re = Regex::new(r"<think>[\s\S]*?</think>").unwrap();
        let filtered = re.replace_all(&response, "");
        filtered.trim().to_string()
    } 

    /// Parse the main characters from the idea contents using the LLM and return a Vec<Character>.
    /// The LLM is expected to return a JSON object with a "characters" array.
    pub async fn parse_characters(&mut self, idea_contents: String) -> Result<Vec<Character>, String> {
        let prompt = r#"
        # Story
        {idea_contents}

        # Instructions
        Find the main characters in this story. For each character, provide the following information:
        - Name
        - Is this a main character?
        - Physical description
        - Backstory summary
        - Internal goals
        - External goals
        - Immediate goal
        - Fears
        - Motivations
        - Flaws
        - Virtues
        - What stage of their character arc are they in?
        - Rules for character voice, tone, and style
        - Notes from the story thus far to maintain consistency and continuity

        List the information in JSON format.
        ## Example Output
        {
            "characters": [
                {
                    "name": "Character Name",
                    "main": true,
                    "physical_description": "Character Physical Description",
                    "backstory_summary": "Character Backstory Summary",
                    "internal_goals": ["Internal Goal 1", "Internal Goal 2"],
                    "external_goals": ["External Goal 1", "External Goal 2"],
                    "immediate_goal": "Immediate Goal",
                    "fears": ["Fear 1", "Fear 2"],
                    "motivations": ["Motivation 1", "Motivation 2"],
                    "flaws": ["Flaw 1", "Flaw 2"],
                    "virtues": ["Virtue 1", "Virtue 2"],
                    "arc_stage": "Character Arc Stage",
                    "voice_rules": "Character Voice Rules",
                    "continuity_notes": "Continuity Notes"
                }
            ]
        }
        "#;
        let prompt = prompt.replace("{idea_contents}", &idea_contents);
        let response = self.action(prompt.to_string()).await;
        #[derive(Deserialize)]
        struct CharactersWrapper {
            characters: Vec<Character>,
        }
        let json_start = response.find('{').unwrap_or(0);
        let json_str = &response[json_start..];
        match serde_json::from_str::<CharactersWrapper>(json_str) {
            Ok(wrapper) => Ok(wrapper.characters),
            Err(e) => Err(format!("Failed to parse characters JSON: {e}\nResponse: {json_str}")),
        }
    }
}