use std::fmt::Display;

use super::race::Race;

pub mod dwarf;

pub mod common {
    use crate::frontend::race::RacialTrait;
    
    pub fn darkvision() -> RacialTrait {
        RacialTrait { 
            name: "Darkvision".into(),
            summary: "You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can’t discern color in darkness, only shades of gray.".into(),
            action_type: None 
        }
    }
}

// TODO: Update with all races 
//
/// All of the possible races.
#[derive(Debug, Clone, PartialEq)]
pub enum RaceName {
    Dwarf,
    _Count,
}

impl RaceName {
    // NOTE: Keep synced with `Races` enum
    pub const ALL: [RaceName; RaceName::_Count as usize] = [RaceName::Dwarf];
}

impl Display for RaceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RaceName::Dwarf => f.write_str("Dwarf"),
            RaceName::_Count => unreachable!("Invalid race"),
        }
    }
}

impl From<Race> for &RaceName {
    fn from(value: Race) -> Self {
        match value.name.as_str() {
            "Dwarf" => &RaceName::Dwarf,
            _ => unreachable!("Invalid race")
        }
    }
}

impl Into<Race> for &RaceName {
    fn into(self) -> Race {
        match self {
            RaceName::Dwarf => dwarf::dwarf(),
            RaceName::_Count => unreachable!("`_Count` is not a valid race"),
        }
    }
}
