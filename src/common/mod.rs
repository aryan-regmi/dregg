pub mod attributes;
pub mod language;
pub mod proficiency;
pub mod size;
pub mod skills;
pub mod speed;
pub mod spell;
pub mod traits;

pub use attributes::*;
pub use language::*;
pub use proficiency::*;
pub use size::*;
pub use skills::*;
pub use speed::*;
pub use spell::*;
pub use traits::*;

pub trait RangeTrait<T> {
    fn start(&self) -> T;
    fn end(&self) -> T;
    fn singular(value: T) -> Self;
}

/// Represents a range of possible values.
#[derive(Debug, Clone, PartialEq)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl<T> Range<T> {}

impl<T: Clone> RangeTrait<T> for Range<T> {
    fn start(&self) -> T {
        self.start.clone()
    }

    fn end(&self) -> T {
        self.end.clone()
    }

    fn singular(value: T) -> Self {
        Self {
            start: value.clone(),
            end: value,
        }
    }
}

/// Represents a range of values under the `start` value and over the `end` value.
#[derive(Debug)]
pub struct UnderOver<T> {
    pub under: T,
    pub over: T,
}

impl<T: Clone> RangeTrait<T> for UnderOver<T> {
    fn start(&self) -> T {
        self.under.clone()
    }

    fn end(&self) -> T {
        self.over.clone()
    }

    fn singular(value: T) -> Self {
        Self {
            under: value.clone(),
            over: value,
        }
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

/// Represents a die.
#[derive(Debug, Clone, PartialEq)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

/// Represents a type of action.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Action,
    BonusAction,
    Reaction,
}

/// Represents an age (in years).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Age(pub usize);
