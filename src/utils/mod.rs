pub mod age;
pub mod attributes;
pub mod common_traits;
pub mod language;
pub mod proficiency;
pub mod size;
pub mod skills;
pub mod speed;
pub mod spell;
pub mod summary;
pub mod traits;

pub use age::*;
pub use attributes::*;
pub use common_traits::*;
pub use language::*;
pub use proficiency::*;
pub use size::*;
pub use skills::*;
pub use speed::*;
pub use spell::*;
pub use summary::*;
pub use traits::*;

/// A trait to represent a range.
pub trait RangeTrait<T> {
    /// The starting point of the range.
    fn start(&self) -> T;

    /// The ending point of the range.
    fn end(&self) -> T;

    /// Creates a new instance with the same value for `start` and `end`.
    fn singular(value: T) -> Self;

    /// Determines if an instance is singular (i.e. starts and ends on the same value).
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

pub mod styles {
    use iced::{font, widget::container, Background, Border, Font, Padding, Theme};

    pub const SECTION_FONT_SIZE: f32 = 18.0;
    pub const BASE_PADDING: Padding = Padding {
        top: 5.0,
        right: TITLE_OUTER_PAD,
        bottom: 5.0,
        left: TITLE_OUTER_PAD,
    };

    pub const TITLE_FONT_SIZE: f32 = 32.0;
    pub const TITLE_INNER_PAD: f32 = 10.0;
    pub const TITLE_OUTER_PAD: f32 = 30.0;

    pub const SUMMARY_HEADING_FONT: f32 = 24.0;
    pub const SUMMARY_PADDING: Padding = Padding {
        top: 0.0,
        right: TITLE_OUTER_PAD,
        bottom: 10.0,
        left: TITLE_OUTER_PAD,
    };
    pub const SUMMARY_SUBSECTION_PADDING: Padding = Padding {
        bottom: 10.0,
        ..SUMMARY_PADDING
    };

    const INDENT_FACTOR: f32 = 1.5;
    pub fn indented_padding() -> Padding {
        Padding {
            left: BASE_PADDING.left * INDENT_FACTOR,
            ..Default::default()
        }
    }
    pub const COLUMN_SPACING: f32 = BASE_PADDING.bottom;

    pub fn radio_padding() -> Padding {
        Padding {
            left: BASE_PADDING.left,
            top: BASE_PADDING.top,
            bottom: BASE_PADDING.bottom,
            ..Default::default()
        }
    }

    pub const HORIZONTAL_LINE_PADDING: Padding = Padding {
        right: 20.0,
        left: 20.0,
        ..BASE_PADDING
    };

    pub const SUBRACE_PADDING: Padding = Padding {
        right: 0.0,
        left: 0.0,
        ..BASE_PADDING
    };
    pub const SUBRACE_TITLE_PADDING: Padding = Padding {
        top: TITLE_OUTER_PAD,
        right: 0.0,
        bottom: TITLE_OUTER_PAD,
        left: 0.0,
    };

    pub fn row_adjusted_padding() -> Padding {
        Padding {
            top: 2.0,
            ..Default::default()
        }
    }

    pub fn bold_font() -> Font {
        Font {
            weight: font::Weight::Bold,
            ..Default::default()
        }
    }

    pub fn title(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();
        container::Style {
            background: Some(Background::Color(palette.background.weak.color)),
            border: Border {
                color: palette.background.strong.color,
                width: 1.0,
                radius: 3.into(),
            },
            ..Default::default()
        }
    }
}
