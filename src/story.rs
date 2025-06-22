use id_derive::HasId;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Story {
    title: String,
    elevator_pitch: String,
    genre: String,
    intended_audience: String,
    core_themes: Vec<String>,
    tone: String,
    voice: String,
    pov_scheme: String,
    tense: String,
    style_guide: String,
    target_length_chapters: u16,
    target_length_words_per_chapter: u16,
    characters: Vec<Character>,
    setting: Setting,
    plot: Plot,
    chapters: Vec<Chapter>,
}

impl Story {
    pub fn get_major_characters(&self) -> Vec<Character> {
        self.characters.iter().filter(|c| c.major).cloned().collect()
    }
}

#[derive(Debug, Clone, HasId, Serialize)]
pub struct Character {
    id: String,
    major: bool,
    name: String,
    description: String,
    backstory_summary: String,
    internal_goals: Vec<String>,
    external_goals: Vec<String>,
    immediate_goal: String,
    fears: Vec<String>,
    motivations: Vec<String>,
    relationships: Vec<Relationship>,
    flaws: Vec<String>,
    virtues: Vec<String>,
    arc_stage: String,
    voice_rules: String,
    continuity_notes: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Relationship {
    character_id: String,
    relationship_type: String,
    current_status: String,
}

#[derive(Debug, Clone)]
pub struct Setting {
    features: Vec<String>,
    locations: Vec<Location>,
    events: Vec<Event>,
    factions: Vec<Faction>,
    artifacts: Vec<Artifact>,
    cultures: Vec<Culture>,
    glossary: Glossary,
}

#[derive(Debug, Clone, HasId, Serialize)]
pub struct Faction {
    id: String,
    name: String,
    description: String,
    members: Vec<String>,
    goals: Vec<String>,
    resources: Vec<String>,
}

#[derive(Debug, Clone, HasId, Serialize)]
pub struct Artifact {
    id: String,
    name: String,
    physical_description: String,
    lore_snippet: String,
    rules: String,
}

#[derive(Debug, Clone, HasId, Serialize)]
pub struct Culture {
    id: String,
    name: String,
    customs: String,
    taboos: String,
    idioms: String,
}

#[derive(Debug, Clone, HasId, Serialize)]
pub struct Event {
    id: String,
    description: String,
    characters: Vec<String>,
    locations: Vec<String>,
    time: String,
    consequences: Vec<String>,
}

#[derive(Debug, Clone, HasId, Serialize)]
pub struct Location {
    id: String,
    name: String,
    visual_cues: String,
    description: String,
    details: Vec<String>,
}

#[derive(Debug, Clone, HasId, Serialize)]
pub struct Chapter {
    id: String,
    title: String,
    short_description: String,
    text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Plot {
    acts: Vec<Act>,
    timeline: Vec<TimelineEvent>,
    subplots: Vec<Subplot>,
}

#[derive(Debug, Clone, HasId, Serialize)]
pub struct Subplot {
    id: String,
    title: String,
    description: String,
    relevance_to_main_plot: String,
    scene_ids: Vec<String>,
    status: String,
}

#[derive(Debug, Clone, HasId, Serialize)]
pub struct Act {
    id: String,
    act_number: u16,
    title: String,
    purpose: String,
    expected_length_chapters: u16,
}

#[derive(Debug, Clone, HasId, Serialize)]
pub struct Scene {
    id: String,
    act_id: String,
    title: String,
    pov_character: String,
    location_id: String,
    time_in_story: String,
    goal: String,
    conflict: String,
    outcome: String,
    foreshadow_payoff: String,
    text: String,
}

#[derive(Debug, Clone, HasId, Serialize)]
pub struct TimelineEvent {
    id: String,
    time: String,
    event: String,
}

#[derive(Debug, Clone)]
pub struct Glossary {}

// TODO: Create a draft->reviewed->final workflow for the story
// TODO: Functionaly to analyze text (story piece, finished story, idea draft, etc.) and create a continuation