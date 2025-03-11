use crate::{common::AbilityScores, equipment::Equipment};

use super::{background::BackgroundInfo, class::Class, race::Race};

/// Represents a character.
#[derive(Debug, Default)]
pub struct Character {
    /// The character's ability scores.
    pub ability_scores: AbilityScores,

    /// The character's race.
    pub race: Race,

    /// The character's class.
    pub classes: Vec<Class>,

    /// The character's background info.
    pub background: BackgroundInfo,

    /// The character's equipment.
    pub equipment: Equipment,
}
