#![allow(dead_code)]

// TODO: Add rest
//
/// Represents a character's background information (such as name, sex, age, background, etc.).
#[derive(Debug)]
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
#[derive(Debug)]
pub enum Sex {
    Male,
    Female,
    Other,
}

/// Represents an age (in years).
#[derive(Debug)]
pub struct Age(usize);

/// Represents a background.
#[derive(Debug)]
pub struct Background {
    // TODO: Implement!
}
