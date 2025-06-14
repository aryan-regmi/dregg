/// Represents an age (in years).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Age(pub usize);

impl std::fmt::Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

/// Represents the characteristic ages of a race.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AgeInfo {
    /// The age at which a character is considered an adult.
    pub adult: Age,

    /// The average lifespan of a character.
    pub lifespan: Age,
}
