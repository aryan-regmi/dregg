/// Represents an attribute.
#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

/// Represents an ability score increase.
#[derive(Debug, Clone, PartialEq)]
pub enum ASI {
    Any(u8),
    Strength(u8),
    Dexterity(u8),
    Constitution(u8),
    Intelligence(u8),
    Wisdom(u8),
    Charisma(u8),
}

impl ASI {
    pub fn text(&self) -> String {
        match self {
            ASI::Any(amt) => format!("increase any ability score by {amt}. "),
            ASI::Strength(amt) => format!("Strength score increases by {amt}. "),
            ASI::Dexterity(amt) => format!("Dexterity score increases by {amt}. "),
            ASI::Constitution(amt) => format!("Constitution score increases by {amt}. "),
            ASI::Intelligence(amt) => format!("Intelligence score increases by {amt}. "),
            ASI::Wisdom(amt) => format!("Wisdom score increases by {amt}. "),
            ASI::Charisma(amt) => format!("Charisma score increases by {amt}. "),
        }
    }
}

/// Represents ability scores.
#[derive(Debug)]
pub struct AbilityScores {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

impl Default for AbilityScores {
    fn default() -> Self {
        Self {
            strength: 8,
            dexterity: 8,
            constitution: 8,
            intelligence: 8,
            wisdom: 8,
            charisma: 8,
        }
    }
}
