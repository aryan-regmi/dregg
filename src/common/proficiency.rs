/// Represents a proficiency.
#[derive(Debug)]
pub struct Proficiency {
    /// The proficiency level (Proficient or Expertise).
    pub level: ProficiencyLevel,

    /// Determines what the proficiency applies to.
    pub kind: ProficiencyType,

    /// Extra context that is required for proficiencies with conditional activations.
    pub context: Option<String>,
}

/// Represents a level of proficiency.
#[derive(Debug, Default)]
pub enum ProficiencyLevel {
    #[default]
    Proficient,
    Expertise,
}

// TODO: Update with actual inner types
//
/// Represents a type of proficiency.
#[derive(Debug)]
pub enum ProficiencyType {
    Armor(ArmorType),
    Weapons,
    Tools,
    SavingThrows,
    Skills,
}

/// Represents an types of armors.
#[derive(Debug)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
    Shield,
}
