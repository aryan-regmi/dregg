use crate::race::Race;

pub mod dwarf;

pub fn races() -> Vec<Race> {
    vec![dwarf::dwarf()]
}
