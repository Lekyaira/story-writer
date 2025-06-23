use crate::ollama_client::OllamaClient;
use regex::Regex;
use crate::story::{Character, Relationship};
use serde::Deserialize;
use crate::id::HasId;

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

        List the information in **JSON format only**.
        ## Example Output
        {
            "characters": [
                {
                    "name": "Character Name",
                    "main_character": true,
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
        println!("Parsing characters...");
        let response = self.action(prompt.to_string()).await;
        #[derive(Deserialize)]
        struct CharactersWrapper {
            characters: Vec<Character>,
        }
        let json_start = response.find('{').unwrap_or(0);
        let json_str = &response[json_start..];
        let characters = match serde_json::from_str::<CharactersWrapper>(json_str) {
            Ok(mut wrapper) => {
                for character in &mut wrapper.characters {
                    character.id = character.generate_id();
                }
                Ok(wrapper.characters)
            },
            Err(e) => Err(format!("Failed to parse characters JSON: {e}\nResponse: {json_str}")),
        }?;
        let characters = self.parse_relationships(idea_contents, characters).await?;
        Ok(characters)
    }

    pub async fn parse_relationships(&mut self, idea_contents: String, mut characters: Vec<Character>) -> Result<Vec<Character>, String> {
        let character_data = characters.iter().map(|c| format!("Name: {}\nId: {}\n", c.name.clone(), c.id.clone())).collect::<Vec<_>>().join("\n");

        let prompt = r#"
        # Story
        {idea_contents}

        # Characters
        {character_data}
        
        # Instructions
        For **each** character, determine their relationships with **each other** character, if applicable.
        For each relationship, provide the following information:
        - ID of character
        - ID of character relationship is with
        - Relationship type
        - Current status

        List the information in **JSON format only**.
        ## Example Output
        {
            "relationships": [
                {
                    "character_id": "ts456neist4654",
                    "related_character_id": "ier654stne55s",
                    "relationship_type": "Relationship Type",
                    "current_status": "Current Status"
                },
                {
                    "character_id": "ier654stne55s",
                    "related_character_id": "ts456neist4654",
                    "relationship_type": "Relationship Type",
                    "current_status": "Current Status"
                }
            ]
        }
        "#;
        let prompt = prompt.replace("{idea_contents}", &idea_contents);
        let prompt = prompt.replace("{character_data}", &character_data);

        println!("Prompt: {}", prompt);

        println!("Parsing relationships...");
        let response = self.action(prompt.to_string()).await;
        #[derive(Deserialize)   ]
        struct RelationshipData {
            character_id: String,
            related_character_id: String,
            relationship_type: String,
            current_status: String,
        }
        #[derive(Deserialize)]
        struct RelationshipsWrapper {
            relationships: Vec<RelationshipData>,
        }
        let json_start = response.find('{').unwrap_or(0);
        let json_str = &response[json_start..];
        let relationships = match serde_json::from_str::<RelationshipsWrapper>(json_str) {
            Ok(wrapper) => {
                Ok(wrapper.relationships)
            },
            Err(e) => Err(format!("Failed to parse relationships JSON: {e}\nResponse: {json_str}")),
        }?;
        
        let characters = characters.iter().map(|c| {
            // Search RelationshipData for this character ID
            let mut character = c.clone();
            for r in &relationships {
                if r.character_id == c.id {
                    character.relationships.push(Relationship {
                        character_id: r.related_character_id.clone(),
                        relationship_type: r.relationship_type.clone(),
                        current_status: r.current_status.clone(),
                    });
                }
            }
            character
        }).collect::<Vec<_>>();
        Ok(characters)
    }
}