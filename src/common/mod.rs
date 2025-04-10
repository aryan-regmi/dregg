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
    fn is_singular(&self) -> bool;
}

/// Represents a range of possible values.
#[derive(Debug, Clone, PartialEq)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl<T> RangeTrait<T> for Range<T>
where
    T: Clone + PartialEq,
{
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

    fn is_singular(&self) -> bool {
        self.start == self.end
    }
}

/// Represents a range of values under the `start` value and over the `end` value.
#[derive(Debug)]
pub struct UnderOver<T> {
    pub under: T,
    pub over: T,
}

impl<T> RangeTrait<T> for UnderOver<T>
where
    T: Clone + PartialEq,
{
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

    fn is_singular(&self) -> bool {
        self.under == self.over
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Action,
    BonusAction,
    Reaction,
}

/// Represents an age (in years).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Age(pub usize);

impl std::fmt::Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}
