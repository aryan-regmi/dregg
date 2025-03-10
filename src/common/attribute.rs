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
