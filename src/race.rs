use std::fmt::Debug;

use crate::common::{size::Size, Attribute, Language, Speed, Trait};

/// Represents a race.
#[derive(Debug)]
pub struct Race {
    /// The name of the race.
    pub name: String,

    /// The ability score increases provided by the race.
    pub asi: Option<Vec<Attribute>>,

    /// The size of the race.
    pub size: Size,

    /// The speed of the race.
    pub speed: Vec<Speed>,

    /// The languages provided by the race.
    pub languages: Option<Vec<Language>>,

    /// The traits provided by the race.
    pub traits: Vec<Trait>,
}
