#![allow(dead_code)]

pub mod ability_scores;
pub mod attribute;
pub mod language;
pub mod proficiency;
pub mod size;
pub mod speed;
pub mod summary;
pub mod traits;

pub use ability_scores::*;
pub use attribute::*;
pub use language::*;
pub use proficiency::*;
pub use size::*;
pub use speed::*;
pub use summary::*;
pub use traits::*;

/// Represents a range of possible values.
#[derive(Debug)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

/// Represents various choices a character can make.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choice<T> {
    /// A list of choices out of which only one can be selected.
    OnlyOne(Vec<T>),

    /// A list of choices from which all are selected.
    All(Vec<T>),

    /// A single choice.
    Single(T),
}
