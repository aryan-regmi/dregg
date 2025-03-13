use std::fmt::Display;

use crate::{
    common::{Age, Height, Language, Range, Size, Speed, Trait, Weight, ASI},
    race::{Race, Subrace},
    views::{common::Summary, Component},
};

use super::dwarf;

#[derive(Clone, Debug)]
pub enum Message {
    NoSubraceSelected,
    SubraceSelected,
}

#[derive(Debug)]
pub enum Command {
    None,
    SubraceSelected(Subrace),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RaceComponent {
    /// The name of the race.
    pub name: String,

    /// The plural form of the race's name.
    pub name_plural: String,

    /// The description of the race.
    pub summary: Summary,

    /// The ability score increases provided by the race.
    pub asi: Option<Vec<ASI>>,

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
    pub subrace_options: Option<Vec<SubraceComponent>>,
}

impl Component for RaceComponent {
    type Message = Message;

    type Context = ();

    type Command = Command;

    fn view(&self, _ctx: Self::Context) -> iced::Element<Self::Message> {
        todo!()
    }

    fn update(&mut self, _message: Self::Message) -> Self::Command {
        todo!()
    }
}

impl Display for RaceComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl From<RaceComponent> for Race {
    fn from(value: RaceComponent) -> Self {
        Self {
            name: value.name,
            asi: value.asi,
            size: value.size.category,
            speed: value.speed,
            languages: value.languages,
            traits: value.traits,
            subrace: None,
        }
    }
}

impl From<Race> for RaceComponent {
    fn from(value: Race) -> Self {
        // NOTE: Keep in sync with each added race!
        match value.name.as_str() {
            "Dwarf" => dwarf::dwarf(),
            _ => unreachable!("Invalid race type"),
        }
    }
}

/// Represents the defining ages of a race.
#[derive(Debug, Clone, PartialEq)]
pub struct AgeInfo {
    /// The age at which a character is considered an adult.
    pub adult: Age,

    /// The average lifespan of a character.
    pub lifespan: Age,
}

/// Represents the size info for a race.
#[derive(Debug, Clone, PartialEq)]
pub struct SizeInfo {
    /// The size category.
    pub category: Size,

    /// The height in feet and inches.
    pub height: Option<Range<Height>>,

    /// The weight in pounds (lb).
    pub weight: Option<Range<Weight>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubraceComponent {
    /// The name of the subrace.
    pub name: String,

    /// The description of the race.
    pub summary: Summary,

    /// The ability score increases provided by the subrace.
    pub asi: Option<Vec<ASI>>,

    /// The languages provided by the subrace.
    pub languages: Option<Vec<Language>>,

    /// The traits provided by the subrace.
    pub traits: Vec<Trait>,
}
