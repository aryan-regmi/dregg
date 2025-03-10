/// Represents an attribute.
#[derive(Debug)]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

/// Represents an ability score increase.
#[derive(Debug)]
pub enum ASI {
    Any(u8),
    Strength(u8),
    Dexterity(u8),
    Constitution(u8),
    Intelligence(u8),
    Wisdom(u8),
    Charisma(u8),
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
