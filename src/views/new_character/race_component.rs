#![allow(dead_code)]

use crate::{
    common::{Age, Choice, Height, Language, Range, Size, Speed, Trait, Weight, ASI},
    race::Subrace,
    views::common::Summary,
};

#[derive(Debug)]
pub struct RaceComponent {
    /// The name of the race.
    pub name: String,

    /// The plural form of the race's name.
    pub name_plural: String,

    /// The description of the race.
    pub summary: Summary,

    /// The ability score increases provided by the race.
    pub asi: Option<Choice<ASI>>,

    /// The age info of the race.
    pub age: AgeInfo,

    /// The size info of the race.
    pub size: SizeInfo,

    /// The speed of the race.
    pub speed: Vec<Speed>,

    /// A list of traits provided by the race.
    pub traits: Vec<Trait>,

    /// The languages provided by the race.
    pub languages: Option<Vec<Language>>,

    /// Subraces that a character may choose.
    pub subrace_options: Option<Vec<Subrace>>,
}

/// Represents the defining ages of a race.
#[derive(Debug)]
pub struct AgeInfo {
    /// The age at which a character is considered an adult.
    pub adult: Age,

    /// The average lifespan of a character.
    pub lifespan: Age,
}

/// Represents the size info for a race.
#[derive(Debug)]
pub struct SizeInfo {
    /// The size category.
    pub category: Size,

    /// The height in feet and inches.
    pub height: Option<Range<Height>>,

    /// The weight in pounds (lb).
    pub weight: Option<Range<Weight>>,
}
