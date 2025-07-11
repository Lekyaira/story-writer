use crate::ollama_client::OllamaClient;
use regex::Regex;
use crate::story::{Character, Relationship};
use serde::{ Deserialize, de::DeserializeOwned };
use uuid::Uuid;

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

    pub async fn action_stream(&mut self, prompt: String) -> String {
        let response = self.client.get_response_stream(prompt).await;
        response
    }

    pub async fn parse<T: DeserializeOwned>(&mut self, prompt: &str, attempts: u32, params: &[(&str, &str)]) -> Result<T, String> {
        let mut prompt = params.iter().fold(prompt.to_string(), |acc, (key, value)| acc.replace(key, value));
        let mut error_message = String::new();
        for i in 0..attempts {
            let response = self.action_stream(prompt.clone()).await;
            let json_start = response.find('{').unwrap_or(0);
            let json_str = &response[json_start..];
            let result = serde_json::from_str::<T>(json_str);
            match result {
                Ok(result) => return Ok(result),
                Err(e) => {
                    prompt = format!("{prompt}\n\n{response}\n\nError parsing JSON: {e}");
                    error_message = format!("{e}");
                    println!("Error parsing JSON in attempt {i}: {response}\n\nError: {e}");
                },
            }
        }
        Err(format!("Failed to parse JSON after {attempts} attempts: Error parsing JSON: {error_message}"))
    }
// TODO: Re-implement arrays in schema.
    pub async fn parse_characters(&mut self, idea_contents: String) -> Result<Vec<Character>, String> {
        let prompt = r#"
        You are a structured-data extractor.
        
        # Story
        {idea_contents}

        # Instructions
        1. Identify every **unique, sentient** character in the story.
        2. If the story uses multiple surface names (nicknames, titles, species references like “the fox”), choose one canonical name for `name` and list the rest in `aliases`. Do not use hyphens, slashes, or other punctuation in `name` or `aliases`.
        3. Label each character as Main / Secondary / Supporting.

        Think step-by-step internally, then output **only** valid JSON that satisfies the schema below.

        ## Output Schema
        {
            "characters": [
                {
                    "name": "Canonical Character Name",
                    "aliases": ["Alias 1", "Alias 2"],
                    "character_type": "<Main | Secondary | Supporting>"
                }
            ]
        }
        "#;

        #[derive(Deserialize)]
        struct CharactersWrapper {
            characters: Vec<Character>,
        }
        let mut characters = match self.parse::<CharactersWrapper>(prompt, 3, &[("{idea_contents}", &idea_contents)]).await {
            Ok(wrapper) => Ok(wrapper.characters),
            Err(e) => Err(format!("{e}\n")),
        }?;

        // Generate IDs for the characters
        for character in &mut characters {
            character.id = Uuid::new_v4().to_string();
        }
        Ok(characters)
    }

    /// Parse the main characters from the idea contents using the LLM and return a Vec<Character>.
    /// The LLM is expected to return a JSON object with a "characters" array.
    pub async fn parse_character(&mut self, idea_contents: String, mut character: Character) -> Result<Character, String> {
        // Facts gathering pass
        let prompt = r#"
            You are a CharacterSheet extractor & completer.

            # Story
            {idea_contents}

            # Character (canonical data from DB)
            {character}

            ## Task
            - Gather evidence about this character from the story.
            - If evidence is missing, leave the field blank.

            Think step-by-step *internally*, then output only **valid JSON**.
            Follow the output schema exactly.

            ## Output Schema
            {
                "id": "{character_id}",
                "name": "{character_name}",
                "aliases": ["{aliases}"],
                "character_type": "{character_type}",
                "physical_description": "If unknown, infer a brief visual consistent with genre.",
                "backstory_summary": "One-sentence past or background.",
                "internal_goals": "Emotional or psychological wants",
                "external_goals": "Concrete, plot-facing wants",
                "fears": "Fears that drive the character or might prevent them from achieving their goals",
                "flaws": "Negative personality traits, separated by commas",
                "virtues": "Positive personality traits, separated by commas",
                "arc_stage": "<Setup | Rising | Crisis | Resolution>",
                "voice_rules": "Lexical quirks, tone hints",
                "continuity_notes": "Important facts from the story to keep in mind for consistency"
            }
        "#;

        let character_json = serde_json::to_string(&character).unwrap();
        
        let mut character = match self.parse::<Character>(prompt, 3, &[("{idea_contents}", &idea_contents), ("{character}", &character_json)]).await {
            Ok(character) => Ok(character),
            Err(e) => Err(format!("{e}\n")),
        }?;

        // Embellishment pass
        let prompt = r#"
        You are a CharacterSheet extractor & completer.

            # Story
            {idea_contents}

            # Character (canonical data from DB)
            {character}

            ## Task
            - Use the information from the story, plus the facts gathered about this character, then complete the missing information plausibly.
            - Infer missing information based on the character's data, story themes and genre, and your own intuition.
            - Embellish the character's data to make them more consistent, interesting and engaging.

            Think step-by-step *internally*, then output only **valid JSON**.
            Follow the output schema exactly.

            ## Output Schema
            {
                "id": "{character_id}",
                "name": "{character_name}",
                "aliases": ["{aliases}"],
                "character_type": "{character_type}",
                "physical_description": "If unknown, infer a brief visual consistent with genre.",
                "backstory_summary": "One-sentence past or background.",
                "internal_goals": "Emotional or psychological wants",
                "external_goals": "Concrete, plot-facing wants",
                "fears": "Fears that drive the character or might prevent them from achieving their goals",
                "flaws": "Negative personality traits, separated by commas",
                "virtues": "Positive personality traits, separated by commas",
                "arc_stage": "<Setup | Rising | Crisis | Resolution>",
                "voice_rules": "Lexical quirks, tone hints",
                "continuity_notes": "Important facts from the story to keep in mind for consistency"
            }
        "#;
        
        let mut character = match self.parse::<Character>(prompt, 3, &[("{idea_contents}", &idea_contents), ("{character}", &character_json)]).await {
            Ok(character) => Ok(character),
            Err(e) => Err(format!("{e}\n")),
        }?;

        Ok(character)
    }

    // TODO: Fix this
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