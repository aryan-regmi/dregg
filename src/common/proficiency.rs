// TODO: Update with actual inner types
//
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

/// Represents a type of proficiency.
#[derive(Debug)]
pub enum ProficiencyType {
    Armor,
    Weapons,
    Tools,
    SavingThrows,
    Skills,
}

/// Represents a level of proficiency.
#[derive(Debug)]
pub enum ProficiencyLevel {
    Proficient,
    Expertise,
}
