use crate::race::Race;

pub mod dwarf;

pub use dwarf::dwarf;

pub fn races() -> Vec<Race> {
    vec![dwarf::dwarf()]
}
