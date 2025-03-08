use super::Range;

/// Represents the size info for a character.
#[derive(Debug)]
pub struct Size {
    /// The size category.
    pub category: SizeCategory,

    /// The height in feet and inches.
    pub height: Option<Range<Height>>,

    /// The weight in pounds (lb).
    pub weight: Option<Range<Weight>>,
}

/// Represents a height in feet and inches.
#[derive(Debug)]
pub struct Height {
    pub feet: f32,
    pub inches: f32,
}

/// Represents a height in pounds.
#[derive(Debug)]
pub struct Weight(f32);


/// Represents the size category of a character.
#[derive(Debug, Clone, PartialEq)]
pub enum SizeCategory {
    Tiny,
    Small,
    Medium,
    Large,
    Gargantuan,
}
