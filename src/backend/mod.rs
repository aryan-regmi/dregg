#![allow(dead_code)]

pub mod background;
pub mod character;
pub mod class;
pub mod equipment;
pub mod race;

/// Represents ability scores.
#[derive(Debug)]
pub struct AbilityScores {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}

/// Represents a summary/description.
#[derive(Debug)]
pub struct Summary {
    pub main: String,
    pub subsections: Vec<(String, String)>,
}
