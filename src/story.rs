#[derive(Debug, Clone)]
pub struct Story {
    characters: Vec<Character>,
    setting: Setting,
    plot: Plot,
    chapters: Vec<Chapter>,
}

impl Story {
    pub fn get_major_characters(&self) -> Vec<Character> {
        self.characters.iter().filter(|c| c.major).collect()
    }
}

#[derive(Debug, Clone, HasId)]
pub struct Character {
    id: String,
    major: bool,
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

#[derive(Debug, Clone)]
pub struct Setting {
    features: Vec<String>,
    locations: Vec<Location>,
    events: Vec<Event>,
}

#[derive(Debug, Clone)]
pub struct Plot {
    events: Vec<PlotEvent>,
}

#[derive(Debug, Clone)]
pub struct PlotEvent {
    description: String,
    reasoning: String,
    event: Option<Event>,
    sub_events: Vec<PlotEvent>,
}

#[derive(Debug, Clone, HasId)]
pub struct Event {
    id: String,
    description: String,
    characters: Vec<String>,
    locations: Vec<String>,
    time: String,
    consequences: Vec<String>,
}

#[derive(Debug, Clone, HasId)]
pub struct Location {
    id: String,
    name: String,
    description: String,
    details: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Chapter {
    title: String,
    short_description: String,
    text: String,
}