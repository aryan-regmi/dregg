use crate::race::Race;

pub mod dwarf;

pub fn all_races() -> Vec<Race> {
    vec![dwarf::dwarf().into()]
}
