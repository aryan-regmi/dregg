#![allow(dead_code)]

pub mod ability_scores;
pub mod attribute;
pub mod language;
pub mod proficiency;
pub mod size;
pub mod speed;
pub mod traits;

pub use ability_scores::*;
pub use attribute::*;
pub use language::*;
pub use proficiency::*;
pub use size::*;
pub use speed::*;
pub use traits::*;

pub trait RangeTrait<T> {
    fn start(&self) -> T;
    fn end(&self) -> T;
}

/// Represents a range of possible values.
#[derive(Debug)]
pub struct Range<T> {
    start: T,
    end: T,
}

impl<T: Clone> RangeTrait<T> for Range<T> {
    fn start(&self) -> T {
        self.start.clone()
    }

    fn end(&self) -> T {
        self.end.clone()
    }
}

/// Represents a range of values under the `start` value and over the `end` value.
#[derive(Debug)]
pub struct UnderOver<T> {
    under: T,
    over: T,
}

impl<T: Clone> RangeTrait<T> for UnderOver<T> {
    fn start(&self) -> T {
        self.under.clone()
    }

    fn end(&self) -> T {
        self.over.clone()
    }
}

/// Represents various choices a character can make.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choice<T> {
    /// A list of choices out of which only one can be selected.
    OneOf(Vec<T>),

    /// A list of choices from which all are selected.
    AllOf(Vec<T>),

    /// A single choice.
    Single(T),
}
