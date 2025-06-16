pub mod age;
pub mod attributes;
pub mod proficiency;
pub mod range;
pub mod size;
pub mod skills;
pub mod speed;
pub mod spell;
pub mod traits;

pub use age::*;
pub use attributes::*;
pub use proficiency::*;
pub use range::*;
pub use size::*;
pub use skills::*;
pub use speed::*;
pub use spell::*;
pub use traits::*;

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
pub enum ActionType {
    Action,
    BonusAction,
    Reaction,
}
