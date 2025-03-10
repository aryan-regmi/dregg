use std::fmt::Debug;

use crate::common::{size::Size, Language, Speed, Trait, ASI};

/// Represents a race.
#[derive(Debug)]
pub struct Race {
    /// The name of the race.
    pub name: String,

    /// The ability score increases provided by the race.
    pub asi: Option<Vec<ASI>>,

    /// The size of the race.
    pub size: Size,

    /// The speed of the race.
    pub speed: Vec<Speed>,

    /// The languages provided by the race.
    pub languages: Option<Vec<Language>>,

    /// The traits provided by the race.
    pub traits: Vec<Trait>,

    /// The selected subrace, if one exists.
    pub subrace: Option<Subrace>,
}

#[derive(Debug)]
pub struct Subrace {
    /// The name of the subrace.
    pub name: String,

    /// The ability score increases provided by the subrace.
    pub asi: Option<Vec<ASI>>,

    /// The languages provided by the subrace.
    pub languages: Option<Vec<Language>>,

    /// The traits provided by the subrace.
    pub traits: Vec<Trait>,
}
