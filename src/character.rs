use crate::{common::AbilityScores, equipment::Equipment};

use super::{background::BackgroundInfo, class::Class, race::Race};

/// Represents a character.
#[derive(Debug)]
pub struct Character {
    /// The character's ability scores.
    pub ability_scores: AbilityScores,

    /// The character's race.
    pub race: Option<Race>,

    /// The character's class.
    pub classes: Vec<Class>,

    /// The character's background info.
    pub background: Option<BackgroundInfo>,

    /// The character's equipment.
    pub equipment: Option<Equipment>,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            ability_scores: Default::default(),
            race: None,
            classes: vec![],
            background: None,
            equipment: None,
        }
    }
}
