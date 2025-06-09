/// Represents a speed of a character.
#[derive(Debug, Clone, PartialEq)]
pub enum Speed {
    Walking(u16),
    Flying(u16),
    Swimming(u16),
    Climbing(u16),
}

impl std::fmt::Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Speed::Walking(amt) => {
                f.write_fmt(format_args!("You have a walking speed of {amt} feet. "))
            }
            Speed::Flying(amt) => {
                f.write_fmt(format_args!("You have a flying speed of {amt} feet. "))
            }
            Speed::Swimming(amt) => {
                f.write_fmt(format_args!("You have a swimming speed of {amt} feet. "))
            }
            Speed::Climbing(amt) => {
                f.write_fmt(format_args!("You have a climbing speed of {amt} feet. "))
            }
        }
    }
}
