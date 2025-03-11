/// Represents a height in feet and inches.
#[derive(Debug, Clone, Copy)]
pub struct Height {
    pub feet: f32,
    pub inches: f32,
}

/// Represents a height in pounds.
#[derive(Debug, Clone, Copy)]
pub struct Weight(pub f32);

/// Represents the size of a character.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Size {
    Tiny,
    Small,
    #[default]
    Medium,
    Large,
    Gargantuan,
}
