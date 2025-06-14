use crate::utils::{Range, RangeTrait};

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

/// Represents a weight in pounds.
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

/// Represents the size info for a race.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SizeInfo {
    /// The size category.
    pub category: Size,

    /// The height in feet and inches.
    pub height: Option<Range<Height>>,

    /// The weight in pounds (lb).
    pub weight: Option<Range<Weight>>,
}

impl SizeInfo {
    pub fn to_text(&self, name_plural: &str) -> String {
        let category = &self.category;
        let has_height = self.height.is_some();
        let has_weight = self.weight.is_some();
        let txt = if has_height && has_weight {
            let height = self.height.as_ref().unwrap();
            let weight = self.weight.as_ref().unwrap();
            format!(
                "{name_plural} stand at around {height} tall and weigh about {weight}. Your size is {category}.",
            )
        } else if has_height && !has_weight {
            let height = self.height.as_ref().unwrap();
            format!("{name_plural} stand at around {height} tall. Your size is {category}.",)
        } else if !has_height && has_weight {
            let weight = self.weight.as_ref().unwrap();
            format!("{name_plural} stand at around {weight} tall. Your size is {category}.",)
        } else {
            format!("Your size is {category}.",)
        };
        txt
    }
}

impl std::fmt::Display for Range<Height> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_singular() {
            f.write_fmt(format_args!("{}", self.start))
        } else {
            f.write_fmt(format_args!("{} - {}", self.start, self.end))
        }
    }
}

impl std::fmt::Display for Range<Weight> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_singular() {
            f.write_fmt(format_args!("{}", self.start))
        } else {
            f.write_fmt(format_args!("{} - {}", self.start, self.end))
        }
    }
}
