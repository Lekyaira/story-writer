#[derive(Debug, Clone)]
pub struct Story {
    major_characters: Vec<u16>,
    characters: Vec<Character>,
    locations: Vec<Location>,
    events: Vec<Event>,
    plot: Plot,
    chapters: Vec<Chapter>,
}

#[derive(Debug, Clone)]
pub struct MajorCharacter {

}
impl MajorCharacter {
    pub fn new(character: Character, mut story: &mut Story) -> Self {
        let mut id: Option<u16> = None;
        if story.characters.iter().any(|c| c.name == character.name) {
            id = Some(story.characters.iter().position(|c| c.name == character.name).unwrap() as u16);
        } else {
            id = Some(story.characters.len() as u16);
            story.characters.push(character);
        }
        Self { id }
    }

    pub fn get(story: &Story) -> Character {
        story.characters[self.id]
    }
}
    

#[derive(Debug, Clone)]
pub struct Character {
    id: u16,
    name: String,
    description: String,
    backstory: String,
    goals: Vec<String>,
    fears: Vec<String>,
    motivations: Vec<String>,
    relationships: Vec<Relationship>,
}

#[derive(Debug, Clone)]
pub struct Relationship {
    character: Character,
    relationship: String,
}