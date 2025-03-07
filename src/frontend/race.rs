#![allow(unused)]

// TODO: Make all `view` functions take in padding info instead of hardcoding!

use std::{collections::HashMap, fmt::Display};

use iced::{
    widget::{column, container, horizontal_rule, radio, row, scrollable, Text},
    Element, Length, Padding,
};

use super::{
    races,
    utils::{styles, Action, Age, Attribute, Choices, Language, Size, Speed, Summary},
};

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
    pub fn view<'a, Msg: 'a + Clone>(
        self,
        on_subrace_selected: &'a dyn Fn(Subrace) -> Msg,
        selected_subrace: Option<&Subrace>,
    ) -> Element<'a, Msg> {
        let line = container(horizontal_rule(1.0)).padding(styles::HORIZONTAL_LINE_PADDING);

        let title = container(
            container(Text::new(self.name).size(styles::TITLE_FONT_SIZE))
                .center_x(Length::Fill)
                .padding(styles::TITLE_INNER_PAD)
                .style(styles::title),
        )
        .padding(styles::TITLE_OUTER_PAD);

        let summary = self.summary.view(
            styles::BASE_PADDING,
            styles::SUMMARY_PADDING,
            styles::SUMMARY_SUBSECTION_PADDING,
        );

        let asi = if self.asi.len() > 0 {
            let mut content = row![Text::new("Ability Score Increase: ")
                .font(styles::bold_font())
                .size(styles::SECTION_FONT_SIZE)];

            for asi in &self.asi {
                content = content.push(
                    container(Text::new(format!("{}", asi)))
                        .padding(styles::row_adjusted_padding()),
                )
            }

            container(content).padding(styles::BASE_PADDING)
        } else {
            container(row![])
        };

        let age = {
            let age_txt = format!(
                "{} are considered adults at {} years old. On average, they live to {} years.",
                self.name_plural, self.age.adult, self.age.lifespan,
            );
            container(row![
                Text::new("Age: ")
                    .font(styles::bold_font())
                    .size(styles::SECTION_FONT_SIZE),
                container(Text::new(age_txt)).padding(styles::row_adjusted_padding()),
            ])
            .padding(styles::BASE_PADDING)
        };

        let size = self.size.view(&self.name_plural, styles::BASE_PADDING);

        let speed = if self.speed.len() > 0 {
            let mut content = row![Text::new("Speed: ")
                .font(styles::bold_font())
                .size(styles::SECTION_FONT_SIZE)];

            for speed in &self.speed {
                content = content.push(
                    container(Text::new(format!("{}", speed)))
                        .padding(styles::row_adjusted_padding()),
                );
            }

            container(content).padding(styles::BASE_PADDING)
        } else {
            container(row![])
        };

        let proficiencies = if self.proficiencies.len() > 0 {
            let mut content = column![Text::new("Proficiencies: ")
                .font(styles::bold_font())
                .size(styles::SECTION_FONT_SIZE)];

            for proficiency_list in &self.proficiencies {
                content = content.push(
                    container(Text::new(
                        proficiency_list.text("You gain proficiency with"),
                    ))
                    .padding(styles::indented_padding()),
                );
            }

            container(content).padding(styles::BASE_PADDING)
        } else {
            container(column![])
        };

        let racial_traits = if self.traits.len() > 0 {
            let mut content = column![];
            for racial_trait in self.traits {
                content = content.push(racial_trait.view())
            }
            container(content).padding(styles::BASE_PADDING)
        } else {
            container(column![])
        };

        let languages = if self.languages.len() > 0 {
            let mut languages_txt = String::with_capacity(30);
            let num_languages = self.languages.len();
            for (i, language) in self.languages.iter().enumerate() {
                if i == num_languages - 1 {
                    languages_txt.push_str("and ");
                    languages_txt.push_str(&format!("{}.", language));
                } else {
                    if num_languages > 2 {
                        languages_txt.push_str(", ");
                    }
                    languages_txt.push_str(&format!("{} ", language));
                }
            }

            container(row![
                Text::new("Languages: ")
                    .font(styles::bold_font())
                    .size(styles::SECTION_FONT_SIZE),
                container(Text::new(format!("You know {}", languages_txt)))
                    .padding(styles::row_adjusted_padding())
            ])
            .padding(styles::BASE_PADDING)
        } else {
            container(row![])
        };

        let subraces = if self.subraces.len() > 0 {
            let mut content = column![];

            let subrace_list = {
                let mut subrace_content = column![Text::new("Select a new subrace: ")
                    .font(styles::bold_font())
                    .size(styles::SECTION_FONT_SIZE)];

                // Create radio option for each subrace
                for (i, subrace) in self.subraces.iter().enumerate() {
                    let subrace_toggle =
                        container(radio(&subrace.name, subrace, selected_subrace, |v| {
                            on_subrace_selected(v.clone())
                        }))
                        .padding(styles::radio_padding());
                    subrace_content = subrace_content.push(subrace_toggle)
                }

                // Display subrace info
                if let Some(selected_subrace) = selected_subrace {
                    subrace_content = subrace_content.push(selected_subrace.clone().view());
                }

                subrace_content
            };

            content = content.push(subrace_list);
            container(content).padding(styles::BASE_PADDING)
        } else {
            container(column![])
        };

        container(scrollable(column![
            title,
            summary,
            line,
            asi,
            age,
            size,
            speed,
            proficiencies,
            racial_traits,
            languages,
            subraces
        ]))
        .padding(Padding {
            bottom: 10.0,
            ..Default::default()
        })
        .into()
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

/// Represents a trait provided by a race.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RacialTrait {
    /// The name of the trait.
    pub name: String,

    /// The trait's description.
    pub summary: String,

    /// The type of action of the trait.
    pub action_type: Option<Action>,
}

impl RacialTrait {
    pub fn view<'a, Msg: 'a>(self) -> Element<'a, Msg> {
        let mut content = row![].padding(Padding {
            right: 0.0,
            left: 0.0,
            ..styles::BASE_PADDING
        });
        let name = Text::new(format!("{}: ", self.name))
            .font(styles::bold_font())
            .size(styles::SECTION_FONT_SIZE);
        let summary = container(Text::new(self.summary)).padding(styles::row_adjusted_padding());
        content = content.push(name);
        content = content.push(summary);
        content.into()
    }
}

/// Represents a subrace of a race.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Subrace {
    pub fn view<'a, Msg: 'a>(self) -> Element<'a, Msg> {
        let line = container(horizontal_rule(1.0));

        let title = container(
            container(Text::new(self.name).size(styles::TITLE_FONT_SIZE))
                .center_x(Length::Fill)
                .padding(styles::TITLE_INNER_PAD)
                .style(styles::title),
        )
        .padding(styles::SUBRACE_TITLE_PADDING);

        let summary = self.summary.view(
            Padding::default(),
            Padding {
                bottom: 20.0,
                ..Default::default()
            },
            Padding::default(),
        );

        let asi = if self.asi.len() > 0 {
            let mut content = row![Text::new("Ability Score Increase: ")
                .font(styles::bold_font())
                .size(styles::SECTION_FONT_SIZE)];

            for asi in &self.asi {
                content = content.push(
                    container(Text::new(format!("{}", asi)))
                        .padding(styles::row_adjusted_padding()),
                )
            }

            container(content).padding(styles::SUBRACE_PADDING)
        } else {
            container(row![])
        };

        let proficiencies = if self.proficiencies.len() > 0 {
            let mut content = column![Text::new("Proficiencies: ")
                .font(styles::bold_font())
                .size(styles::SECTION_FONT_SIZE)];

            for proficiency_list in &self.proficiencies {
                content = content.push(
                    container(Text::new(
                        proficiency_list.text("You gain proficiency with"),
                    ))
                    .padding(styles::indented_padding()),
                );
            }

            container(content).padding(styles::SUBRACE_PADDING)
        } else {
            container(column![])
        };

        let racial_traits = if self.traits.len() > 0 {
            let mut content = column![];
            for racial_trait in self.traits {
                content = content.push(racial_trait.view())
            }
            container(content).padding(styles::SUBRACE_PADDING)
        } else {
            container(column![])
        };

        let languages = if self.languages.len() > 0 {
            let mut languages_txt = String::with_capacity(30);
            let num_languages = self.languages.len();
            for (i, language) in self.languages.iter().enumerate() {
                if i == num_languages - 1 {
                    languages_txt.push_str("and ");
                    languages_txt.push_str(&format!("{}.", language));
                } else {
                    if num_languages > 2 {
                        languages_txt.push_str(", ");
                    }
                    languages_txt.push_str(&format!("{} ", language));
                }
            }

            container(row![
                Text::new("Languages: ")
                    .font(styles::bold_font())
                    .size(styles::SECTION_FONT_SIZE),
                container(Text::new(format!("You know {}", languages_txt)))
                    .padding(styles::row_adjusted_padding())
            ])
            .padding(styles::SUBRACE_PADDING)
        } else {
            container(row![])
        };

        container(column![
            title,
            summary,
            line,
            asi,
            proficiencies,
            racial_traits,
            languages,
        ])
        .into()
    }
}
