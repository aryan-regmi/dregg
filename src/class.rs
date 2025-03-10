#![allow(dead_code)]

use std::fmt::Debug;

use crate::common::{Die, Proficiency, Trait};

/// Represents a class.
#[derive(Debug)]
pub struct Class {
    /// The number of levels in this class.
    pub level: u8,

    /// The name of the class.
    pub name: String,

    /// The hit die of the class.
    pub hit_dice: Die,

    /// The proficiencies provided by the class.
    pub proficiencies: Vec<Proficiency>,

    /// The traits provided by the class.
    pub traits: Vec<Trait>,
}
