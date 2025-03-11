#![allow(dead_code)]

use std::fmt::Debug;

use crate::common::{ArmorType, Weight};

/// Represents a character's equipment.
#[derive(Debug, Default)]
pub struct Equipment {
    /// The amount of money a character has.
    pub money: Vec<Money>,

    /// The armors in a player's inventory.
    pub armors: Vec<Armor>,
    // TODO: Add rest of equipment:
    //  - Adventuring gear
    //  - Trinkets
    //  - Weapons
    //  - Wondrous Items
    //  - Poisons
    //  - Tools
    //  - Siege Equipment?
}

// TODO: Add conversion functions: https://www.dndbeyond.com/sources/dnd/basic-rules-2014/equipment#Coinage
//
/// Represents money.
#[derive(Debug)]
pub enum Money {
    Copper(usize),
    Silver(usize),
    Gold(usize),
    Electrum(usize),
    Platinum(usize),
}

/// Represents armor.
#[derive(Debug)]
pub struct Armor {
    /// The type of armor (light, heavy, or medium).
    pub kind: ArmorType,

    /// Whether the armor is equipped or not.
    pub equipped: bool,

    /// The weight of the armor.
    pub weight: Weight,

    /// Whether or not the armor imposed a Stealth disadvantage.
    pub stealth_disadvantage: bool,

    /// The Strength requirement for the armor, if there is one.
    pub strength_requirement: Option<u8>,

    /// The AC provided by the armor.
    pub armor_class: ArmorClass,
}

/// Represents armor class.
#[derive(Debug)]
pub struct ArmorClass {
    /// The base armor class provided by the armor.
    pub base: u8,

    /// Whether or not the Dexterity mod is added to the AC value.
    pub dexterity_mod: bool,

    /// The max number that can be added to the AC from the Dexterity mod.
    pub dexterity_mod_limit: Option<u8>,
}
