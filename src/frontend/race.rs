#![allow(unused)]

use std::{collections::HashMap, fmt::Display};

use iced::{
    widget::{column, container, horizontal_rule, row, scrollable, Text},
    Element, Length, Padding,
};

use super::races;

/// Represents a race a character can be.
#[derive(Debug)]
pub struct Race {
    /// The name of the race.
    pub name: String,

    /// The plural form of the race's name.
    pub name_plural: String,

    /// The description of the race.
    pub summary: Summary,

    /// Ability score increases provided by the race.
    pub asi: Vec<Attribute>,

    /// The age info of the race.
    pub age: Age,

    /// The size of the race.
    pub size: Size,

    /// The speed of the race.
    pub speed: Vec<Speed>,

    /// The various languages a character of the race knows.
    pub languages: Vec<Language>,

    /// The proficiencies the race provides.
    pub proficiencies: Vec<Choices>,

    /// Subraces that a character may choose.
    pub subraces: Vec<Subrace>,

    /// A list of traits provided by the race.
    pub traits: Vec<RacialTrait>,
}

impl Race {
    pub fn view<'a, Msg: 'a>(self) -> Element<'a, Msg> {
        let line = container(horizontal_rule(1.0)).padding(styles::HORIZONTAL_LINE_PADDING);

        let title = container(
            container(Text::new(self.name).size(styles::TITLE_FONT))
                .center_x(Length::Fill)
                .padding(styles::TITLE_INNER_PAD)
                .style(styles::title),
        )
        .padding(styles::TITLE_OUTER_PAD);

        let summary = self.summary.view();

        let asi = {
            let mut content = row![Text::new("Ability Score Increase: ")
                .font(styles::bold_font())
                .size(styles::SECTION_FONT)];

            for asi in &self.asi {
                content = content.push(
                    container(Text::new(format!("{}", asi)))
                        .padding(styles::row_adjusted_padding()),
                )
            }

            container(content).padding(styles::BASE_PADDING)
        };

        let age = {
            let age_txt = format!(
                "{} are considered adults at {} years old. On average, they live to {} years.",
                self.name_plural, self.age.adult, self.age.lifespan,
            );
            container(row![
                Text::new("Age: ")
                    .font(styles::bold_font())
                    .size(styles::SECTION_FONT),
                container(Text::new(age_txt)).padding(styles::row_adjusted_padding()),
            ])
            .padding(styles::BASE_PADDING)
        };

        container(scrollable(column![title, summary, line, asi, age])).into()
    }
}

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
#[derive(Debug)]
pub struct Height {
    pub feet: f32,
    pub inches: f32,
}

/// Represents a speed of a character.
#[derive(Debug, Clone, PartialEq)]
pub enum Speed {
    Walking(u16),
    Flying(u16),
    Swimming(u16),
    Climbing(u16),
}

/// Represents a language a character knows.
#[derive(Debug)]
pub struct Language {
    pub name: String,
    pub levels: Vec<LanguageLevel>,
}

/// Represents the various levels of proficiency in a language.
#[derive(Debug)]
pub enum LanguageLevel {
    Speak,
    Read,
    Write,
    Understand,
}

/// Represents various choices a character can make.
#[derive(Debug)]
pub enum Choices {
    /// A list of choices out of which only one can be selected.
    One(Vec<String>),

    /// A list of choices from which all are selected.
    All(Vec<String>),
}

/// Represents a subrace of a race.
#[derive(Debug)]
pub struct Subrace {
    /// The name of the race.
    pub name: String,

    /// The description of the race.
    pub summary: Summary,

    /// Ability score increases provided by the subrace.
    pub asi: Vec<Attribute>,

    /// The various languages a character of the subrace knows.
    pub languages: Vec<Language>,

    /// The proficiencies the race provides.
    pub proficiencies: Vec<Choices>,

    /// A list of traits provided by the subrace.
    pub traits: Vec<RacialTrait>,
}

/// Represents a trait provided by a race.
#[derive(Debug)]
pub struct RacialTrait {
    /// The name of the trait.
    pub name: String,

    /// The trait's description.
    pub summary: String,

    /// The type of action of the trait.
    pub action_type: Option<Action>,
}

/// Types of actions.
#[derive(Debug)]
pub enum Action {
    Action,
    BonusAction,
    Reaction,
}

/// Represents a summary/description.
#[derive(Debug)]
pub struct Summary {
    pub main: String,
    pub subsections: HashMap<String, String>,
}

impl Summary {
    pub fn view<'a, Msg: 'a>(self) -> Element<'a, Msg> {
        let mut content = column![];

        let main = container(Text::new(self.main.clone())).padding(styles::SUMMARY_PADDING);
        content = content.push(main);

        for (section, text) in self.subsections {
            let title_header = container(Text::new(section).font(styles::bold_font()))
                .padding(styles::BASE_PADDING);
            content = content.push(title_header);

            let text = container(Text::new(text)).padding(styles::SUMMARY_SUBSECTION_PADDING);
            content = content.push(text);
        }

        container(content).into()
    }
}

// TODO: Update with all races
//
/// All of the possible races.
#[derive(Debug, Clone, PartialEq)]
pub enum RaceName {
    Dwarf,
    _Count,
}

impl RaceName {
    // NOTE: Keep synced with `Races` enum
    pub const ALL: [RaceName; RaceName::_Count as usize] = [RaceName::Dwarf];
}

impl Display for RaceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RaceName::Dwarf => f.write_str("Dwarf"),
            RaceName::_Count => unreachable!("Invalid race"),
        }
    }
}

impl From<Race> for &RaceName {
    fn from(value: Race) -> Self {
        match value.name.as_str() {
            "Dwarf" => &RaceName::Dwarf,
            _ => unreachable!("Invalid race"),
        }
    }
}

impl Into<Race> for &RaceName {
    fn into(self) -> Race {
        match self {
            RaceName::Dwarf => races::dwarf::dwarf(),
            RaceName::_Count => unreachable!("`_Count` is not a valid race"),
        }
    }
}

mod styles {
    use iced::{font, widget::container, Background, Border, Font, Padding, Shadow, Theme};

    pub const SECTION_FONT: f32 = 18.0;
    pub const BASE_PADDING: Padding = Padding {
        top: 5.0,
        right: TITLE_OUTER_PAD,
        bottom: 5.0,
        left: TITLE_OUTER_PAD,
    };

    pub const TITLE_FONT: f32 = 32.0;
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

    pub const HORIZONTAL_LINE_PADDING: Padding = Padding {
        right: 20.0,
        left: 20.0,
        ..BASE_PADDING
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
