/// Represents an attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attribute {
    Any,
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Any => f.write_str("Any"),
            Attribute::Strength => f.write_str("Strength"),
            Attribute::Dexterity => f.write_str("Dexterity"),
            Attribute::Constitution => f.write_str("Constitution"),
            Attribute::Intelligence => f.write_str("Intelligence"),
            Attribute::Wisdom => f.write_str("Wisdom"),
            Attribute::Charisma => f.write_str("Charisma"),
        }
    }
}

/// All the attributes.
pub const ATTRIBUTES: [Attribute; 7] = [
    Attribute::Any,
    Attribute::Strength,
    Attribute::Dexterity,
    Attribute::Constitution,
    Attribute::Intelligence,
    Attribute::Wisdom,
    Attribute::Charisma,
];

/// Represents an ability score increase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASI {
    pub attribute: Attribute,
    pub value: i8,
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
