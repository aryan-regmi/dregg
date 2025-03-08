/// Represents a speed of a character.
#[derive(Debug)]
pub enum Speed {
    Walking(u16),
    Flying(u16),
    Swimming(u16),
    Climbing(u16),
}
