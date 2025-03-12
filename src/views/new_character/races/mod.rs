use std::fmt::Display;

use crate::race::Race;

use super::RaceComponent;

pub mod dwarf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Races {
    Dwarf,
    Count,
}

impl Races {
    // NOTE: Must be kept in-sync with each added race.
    //
    /// A list of all availabe races.
    pub const ALL: [Self; Self::Count as usize] = [Races::Dwarf];
}

impl From<Races> for RaceComponent {
    fn from(value: Races) -> Self {
        match value {
            Races::Dwarf => dwarf::dwarf(),
            Races::Count => unreachable!("`Count` is not a valid race"),
        }
    }
}

// TODO: Must keep in sync with each added race
impl From<&Race> for Races {
    fn from(value: &Race) -> Self {
        match value.name.as_str() {
            "Dwarf" => Self::Dwarf,
            _ => unreachable!(),
        }
    }
}

impl Display for Races {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Races::Dwarf => f.write_str("Dwarf"),
            Races::Count => unreachable!(),
        }
    }
}
