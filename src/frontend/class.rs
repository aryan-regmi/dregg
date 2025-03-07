#![allow(unused)]

use super::utils::Summary;

/// Represents a class a character can be.
#[derive(Debug)]
pub struct Class {
    /// The name of the class.
    pub name: String,

    /// The plural form of the class name.
    pub name_plural: String,

    /// The description of the class.
    pub summary: Summary,

    // TODO: Add the class table
    // pub class_table: ClassTable
    //
    /// The hit points info for the class.
    pub hit_points: HitPoints,
}

#[derive(Debug)]
pub struct HitPoints {
    hit_dice: f32,
}
