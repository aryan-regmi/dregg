#![allow(dead_code)]

use crate::{
    common::{Age, Trait},
    race::{Race, Subrace},
};

#[derive(Debug)]
pub struct RaceComponent {
    /// The state of the component.
    pub state: Race,

    /// The plural form of the race's name.
    pub name_plural: String,

    /// The description of the race.
    pub summary: Summary,

    /// The age info of the race.
    pub age: AgeInfo,

    /// Subraces that a character may choose.
    pub subrace_options: Vec<Subrace>,

    /// A list of traits provided by the race.
    pub traits: Vec<Trait>,
}

#[derive(Debug)]
pub struct Summary {}

#[derive(Debug)]
pub struct AgeInfo {
    pub adult: Age,
    pub lifespan: Age,
}
