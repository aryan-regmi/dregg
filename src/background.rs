use crate::common::Age;

// TODO: Add rest
//
/// Represents a character's background information (such as name, sex, age, background, etc.).
#[derive(Debug, Default)]
pub struct BackgroundInfo {
    /// The character's name.
    pub name: String,

    /// The character's sex.
    pub sex: Sex,

    /// The character's age.
    pub age: Age,

    /// The character's background.
    pub background: Background,
}

/// Represents a sex.
#[derive(Debug, Default)]
pub enum Sex {
    #[default]
    Male,
    Female,
    Other,
}

/// Represents a background.
#[derive(Debug, Default)]
pub struct Background {
    // TODO: Implement!
}
