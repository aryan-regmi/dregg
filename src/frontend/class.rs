#![allow(unused)]

use super::utils::{Choices, Summary};

/// Represents a class a character can be.
#[derive(Debug)]
pub struct Class {
    /// The name of the class.
    pub name: String,

    /// The plural form of the class name.
    pub name_plural: String,

    /// The description of the class.
    pub summary: Summary,

    // FIXME: Add the class table
    pub class_table: (),

    /// The hit points info for the class.
    pub hit_points: HitPoints,

    /// The proficiencies the class provides.
    pub proficiencies: Vec<Choices<String>>,
}

#[derive(Debug)]
pub struct HitPoints {
    hit_dice: Die,
}

/// Represents a die.
#[derive(Debug)]
pub struct Die {
    pub num_sides: usize,
    pub value: usize,
}
