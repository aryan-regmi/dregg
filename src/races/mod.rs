pub mod dwarf;
pub use dwarf::*;

use crate::components::race::Race;

/// Returns all built-in races.
pub fn all_races() -> Vec<Race> {
    vec![dwarf()]
}
