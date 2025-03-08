use crate::{common::AbilityScores, equipment::Equipment};

use super::{background::Background, class::Class, race::Race};

/// Represents a character.
#[derive(Debug)]
pub struct Character {
    /// The character's name.
    pub name: String,

    /// The character's ability scores.
    pub ability_scores: AbilityScores,

    /// The character's race.
    pub race: Race,

    /// The character's class.
    pub classes: Vec<Class>,

    /// The character's background.
    pub background: Background,

    /// The character's equipment.
    pub equipment: Equipment,
}
