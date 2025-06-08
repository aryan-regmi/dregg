use std::fmt::Display;

use crate::utils::Summary;

/// Represents a race.
#[derive(Debug, Clone, PartialEq)]
pub struct Race {
    /// Name of the race.
    pub name: String,

    /// Plural name of the race.
    pub plural_name: Option<String>,

    /// Description of the race.
    pub description: Summary,
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
