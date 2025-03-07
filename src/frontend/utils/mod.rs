use std::fmt::Display;

use iced::{
    widget::{column, container, row, Text},
    Element, Padding,
};

/// Represents an attribute of a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attribute {
    Strength(u8),
    Dexterity(u8),
    Constitution(u8),
    Intelligence(u8),
    Wisdom(u8),
    Charisma(u8),
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Strength(amount) => {
                f.write_fmt(format_args!("Strength score increases by {amount}. "))
            }
            Attribute::Dexterity(amount) => {
                f.write_fmt(format_args!("Dexterity score increases by {amount}. "))
            }
            Attribute::Constitution(amount) => {
                f.write_fmt(format_args!("Constitution score increases by {amount}. "))
            }
            Attribute::Intelligence(amount) => {
                f.write_fmt(format_args!("Intelligence score increases by {amount}. "))
            }
            Attribute::Wisdom(amount) => {
                f.write_fmt(format_args!("Wisdom score increases by {amount}. "))
            }
            Attribute::Charisma(amount) => {
                f.write_fmt(format_args!("Charisma score increases by {amount}. "))
            }
        }
    }
}

#[derive(Debug)]
pub struct Age {
    /// The age at which a character is considered an adult.
    pub adult: u16,

    /// The average lifespan of a character.
    pub lifespan: u16,
}

/// Represents a range of possible values.
#[derive(Debug)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl<T: PartialEq> Range<T> {
    /// Returns `true` if the start and end values of the range are the same.
    pub fn is_singular(&self) -> bool {
        self.start == self.end
    }
}

impl Range<Height> {
    /// Converts a height range into a printable string.
    pub fn text(&self) -> String {
        // No range
        if self.is_singular() {
            let inches_txt = if self.start.feet_only() {
                String::from("")
            } else {
                format!(" and {} inches", self.start.inches)
            };

            format!("{} feet{}", self.start.feet, inches_txt)
        }
        // Range
        else {
            let start_inches_txt = if self.start.feet_only() {
                String::from("")
            } else {
                format!(" and {} inches", self.start.inches)
            };
            let end_inches_txt = if self.end.feet_only() {
                String::from("")
            } else {
                format!(" and {} inches", self.end.inches)
            };

            format!(
                "{} feet{} to {} feet{}",
                self.start.feet, start_inches_txt, self.end.feet, end_inches_txt
            )
        }
    }
}

impl Range<f32> {
    // "{} stand at around {} and about {}. Your size is {}.",
    //
    /// Converts a height range into a printable string.
    pub fn text(&self) -> String {
        if self.is_singular() {
            format!("{} pounds", self.start)
        } else {
            format!("{} to {} pounds", self.start, self.end)
        }
    }
}

/// Represents the size info for a character.
#[derive(Debug)]
pub struct Size {
    /// The size category.
    pub category: SizeCategory,

    /// The height in feet and inches.
    pub height: Option<Range<Height>>,

    /// The weight in pounds (lb).
    pub weight: Option<Range<f32>>,
}

impl Size {
    pub fn view<'a, Msg: 'a>(self, name_plural: &str, base_padding: Padding) -> Element<'a, Msg> {
        let mut content = row![Text::new("Size: ")
            .font(styles::bold_font())
            .size(styles::SECTION_FONT_SIZE)];

        let category = match self.category {
            SizeCategory::Tiny => "Tiny",
            SizeCategory::Small => "Small",
            SizeCategory::Medium => "Medium",
            SizeCategory::Large => "Large",
            SizeCategory::Gargantuan => "Gargantuan",
        };

        // View for the size info
        {
            let has_height = self.height.is_some();
            let has_weight = self.weight.is_some();

            let txt = if has_height && has_weight {
                format!(
                    "{} stand at around {} tall and weigh about {}. Your size is {}.",
                    name_plural,
                    self.height.unwrap().text(),
                    self.weight.unwrap().text(),
                    category
                )
            } else if has_height && !has_weight {
                format!(
                    "{} stand at around {} tall. Your size is {}.",
                    name_plural,
                    self.height.unwrap().text(),
                    category
                )
            } else if !has_height && has_weight {
                format!(
                    "{} weight about {}. Your size is {}.",
                    name_plural,
                    self.weight.unwrap().text(),
                    category
                )
            } else {
                format!("Your size is {}.", category)
            };

            content =
                content.push(container(Text::new(txt)).padding(styles::row_adjusted_padding()));
        };

        container(content).padding(base_padding).into()
    }
}

/// Represents the size category of a character.
#[derive(Debug, Clone, PartialEq)]
pub enum SizeCategory {
    Tiny,
    Small,
    Medium,
    Large,
    Gargantuan,
}

/// Represents the height of a character in feet and inches.
#[derive(Debug, PartialEq)]
pub struct Height {
    pub feet: f32,
    pub inches: f32,
}

impl Height {
    /// Returns `true` if `inches` is 0.
    pub fn feet_only(&self) -> bool {
        self.inches == 0.0
    }
}

/// Represents a speed of a character.
#[derive(Debug, Clone, PartialEq)]
pub enum Speed {
    Walking(u16),
    Flying(u16),
    Swimming(u16),
    Climbing(u16),
}

impl Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Speed::Walking(amount) => {
                f.write_fmt(format_args!("Walking speed of {} feet.", amount))
            }
            Speed::Flying(amount) => f.write_fmt(format_args!("Flying speed of {} feet.", amount)),
            Speed::Swimming(amount) => {
                f.write_fmt(format_args!("Swimming speed of {} feet.", amount))
            }
            Speed::Climbing(amount) => {
                f.write_fmt(format_args!("Climbing speed of {} feet.", amount))
            }
        }
    }
}

/// Represents a language a character knows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language {
    pub name: String,
    pub levels: Vec<LanguageLevel>,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} ({})",
            self.name,
            self.levels
                .iter()
                .map(|lvl| lvl.text())
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

/// Represents the various levels of proficiency in a language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanguageLevel {
    Speak,
    Read,
    Write,
    Understand,
}

impl LanguageLevel {
    pub fn text(&self) -> String {
        match self {
            LanguageLevel::Speak => "Speak".into(),
            LanguageLevel::Read => "Read".into(),
            LanguageLevel::Write => "Write".into(),
            LanguageLevel::Understand => "Understand".into(),
        }
    }
}

/// Represents various choices a character can make.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choices {
    /// A list of choices out of which only one can be selected.
    One(Vec<String>),

    /// A list of choices from which all are selected.
    All(Vec<String>),
}

impl Choices {
    pub fn text(&self, header: &str) -> String {
        match self {
            Choices::One(items) => format!(
                "{} one of the following of your choice: {}.",
                header,
                items.join(", ")
            ),
            Choices::All(items) => {
                format!("{} all of the following: {}.", header, items.join(", "))
            }
        }
    }
}

/// Types of actions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Action,
    BonusAction,
    Reaction,
}

/// Represents a summary/description.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary {
    pub main: String,
    pub subsections: Vec<(String, String)>,
}

impl Summary {
    pub fn view<'a, Msg: 'a>(
        self,
        base_padding: Padding,
        summary_padding: Padding,
        summary_subsection_padding: Padding,
    ) -> Element<'a, Msg> {
        let mut content = column![];

        let main = container(Text::new(self.main.clone())).padding(summary_padding);
        content = content.push(main);

        for (section, text) in self.subsections {
            let title_header =
                container(Text::new(section).font(styles::bold_font())).padding(base_padding);
            content = content.push(title_header);

            let text = container(Text::new(text)).padding(summary_subsection_padding);
            content = content.push(text);
        }

        container(content).into()
    }
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
