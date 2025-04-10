/// Represents a height in feet and inches.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Height {
    pub feet: f32,
    pub inches: f32,
}

impl std::fmt::Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.inches == 0.0 {
            f.write_fmt(format_args!("{} ft", self.feet))
        } else {
            f.write_fmt(format_args!("{} ft {} in", self.feet, self.inches))
        }
    }
}

/// Represents a height in pounds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weight(pub f32);

impl std::fmt::Display for Weight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} lbs", self.0))
    }
}

/// Represents the size of a character.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Size {
    Tiny,
    Small,
    #[default]
    Medium,
    Large,
    Gargantuan,
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Tiny => f.write_str("Tiny"),
            Size::Small => f.write_str("Small"),
            Size::Medium => f.write_str("Medium"),
            Size::Large => f.write_str("Large"),
            Size::Gargantuan => f.write_str("Gargantuan"),
        }
    }
}
