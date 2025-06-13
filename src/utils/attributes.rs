/// Represents an attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attribute {
    Any,
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

/// Represents an ability score increase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASI {
    pub attribute: Attribute,
    pub value: u8,
}

impl ASI {
    /// Returns the ASI as a string.
    pub fn to_text(&self) -> String {
        match self.attribute {
            Attribute::Any => format!("increase any ability score by {}.", self.value),
            Attribute::Strength => format!("Strength score increases by {}.", self.value),
            Attribute::Dexterity => format!("Dexterity score increases by {}.", self.value),
            Attribute::Constitution => format!("Constitution score increases by {}.", self.value),
            Attribute::Intelligence => format!("Intelligence score increases by {}.", self.value),
            Attribute::Wisdom => format!("Wisdom score increases by {}.", self.value),
            Attribute::Charisma => format!("Charisma score increases by {}.", self.value),
        }
    }
}
