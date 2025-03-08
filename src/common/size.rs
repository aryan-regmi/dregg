/// Represents a height in feet and inches.
#[derive(Debug)]
pub struct Height {
    pub feet: f32,
    pub inches: f32,
}

/// Represents a height in pounds.
#[derive(Debug)]
pub struct Weight(f32);

/// Represents the size of a character.
#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Gargantuan,
}
