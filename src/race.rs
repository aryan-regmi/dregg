use std::fmt::Display;

use iced::{
    widget::{column, container, horizontal_rule, radio, row, Text},
    Element, Length,
};

use crate::utils::{self, AgeInfo, Attribute, Language, SizeInfo, Speed, Summary, Trait, ASI};

/// Represents a race.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Race {
    /// Name of the race.
    pub name: String,

    /// Plural name of the race.
    pub plural_name: Option<String>,

    /// Description of the race.
    pub summary: Summary,

    /// ASIs provided by the race.
    pub asi: Option<Vec<ASI>>,

    /// The age info for the race.
    pub age: Option<AgeInfo>,

    /// The size info for the race.
    pub size: SizeInfo,

    /// The various speeds of the race.
    pub speed: Vec<Speed>,

    /// A list of traits provided by the race.
    pub traits: Option<Vec<Trait>>,

    /// The languages provided by the race.
    pub languages: Option<Vec<Language>>,

    /// The various subraces of the race.
    pub subraces: Option<Vec<Subrace>>,
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl Race {
    pub fn view<'a>(&'a self, selected_subrace: Option<&'a Subrace>) -> iced::Element<Message> {
        let line = container(horizontal_rule(1.0)).padding(utils::styles::HORIZONTAL_LINE_PADDING);

        let title = container(
            container(Text::new(&self.name).size(utils::styles::TITLE_FONT_SIZE))
                .center_x(Length::Fill)
                .padding(utils::styles::TITLE_INNER_PAD)
                .style(utils::styles::title),
        )
        .padding(utils::styles::TITLE_OUTER_PAD);

        let summary = self
            .summary
            .view(
                utils::styles::BASE_PADDING,
                utils::styles::SUMMARY_PADDING,
                utils::styles::SUMMARY_SUBSECTION_PADDING,
            )
            .map(|_| Message::RaceSelected(self.clone()));

        let asi = if let Some(asi_list) = &self.asi {
            let mut content = row![Text::new("Ability Score Increase: ")
                .font(utils::styles::bold_font())
                .size(styles::SUBSECTION_TITLE_SIZE)];

            // FIXME: https://github.com/aryan-regmi/dregg/issues/3
            let mut asi_text = String::with_capacity(128);
            for asi in asi_list {
                if asi.attribute == Attribute::Any {
                    asi_text.push_str("You can ");
                } else {
                    asi_text.push_str("Your ");
                }
                asi_text.push_str(&asi.to_text());
            }
            content = content.push(
                container(Text::new(asi_text)).padding(utils::styles::row_adjusted_padding()),
            );
            container(content).padding(styles::SUBSECTION_PADDING)
        } else {
            container(row![])
        };

        let age = if let Some(age) = &self.age {
            let name = self.plural_name.as_ref().unwrap_or_else(|| &self.name);
            let age_txt = format!(
                "{} are considered adults at {} years old. On average, they live to {} years.",
                name, age.adult, age.lifespan
            );
            container(row![
                Text::new("Age: ")
                    .font(utils::styles::bold_font())
                    .size(styles::SUBSECTION_TITLE_SIZE),
                container(Text::new(age_txt)).padding(utils::styles::row_adjusted_padding())
            ])
            .padding(styles::SUBSECTION_PADDING)
        } else {
            container(row![])
        };

        let size = {
            let name = self.plural_name.as_ref().unwrap_or_else(|| &self.name);
            let content = row![
                Text::new("Size: ")
                    .font(utils::styles::bold_font())
                    .size(styles::SUBSECTION_TITLE_SIZE),
                container(Text::new(self.size.to_text(name)))
                    .padding(utils::styles::row_adjusted_padding())
            ];
            container(content).padding(styles::SUBSECTION_PADDING)
        };

        let speed = {
            let mut content = row![Text::new("Speed: ")
                .font(utils::styles::bold_font())
                .size(styles::SUBSECTION_TITLE_SIZE)];

            if self.speed.is_empty() {
                content = content.push(
                    container(Text::new(format!("You have a walking speed of 30 feet. ")))
                        .padding(utils::styles::row_adjusted_padding()),
                )
            } else {
                for speed in &self.speed {
                    content = content.push(
                        container(Text::new(format!("{speed}")))
                            .padding(utils::styles::row_adjusted_padding()),
                    )
                }
            }

            container(content).padding(styles::SUBSECTION_PADDING)
        };

        let traits = if let Some(traits) = &self.traits {
            let mut content = column![];
            for tr in traits {
                let name = Text::new(format!("{}: ", tr.name))
                    .font(utils::styles::bold_font())
                    .size(styles::SUBSECTION_TITLE_SIZE);
                let summary = container(Text::new(&tr.summary))
                    .padding(utils::styles::row_adjusted_padding());
                content = content.push(row![name, summary].padding(styles::SUBSECTION_PADDING))
            }
            container(content) // .padding(styles::SUBSECTION_PADDING)
        } else {
            container(row![])
        };

        let languages = if let Some(languages) = &self.languages {
            let mut content = row![Text::new("Languages: ")
                .font(utils::styles::bold_font())
                .size(styles::SUBSECTION_TITLE_SIZE)];

            for (i, language) in languages.iter().enumerate() {
                let language_levels = {
                    let mut txt = String::new();
                    for (i, level) in language.levels.iter().enumerate() {
                        if i == language.levels.len() - 1 {
                            txt.push_str(&format!("{level}"));
                        } else {
                            txt.push_str(&format!("{level}/"));
                        }
                    }
                    txt
                };

                content = content.push(
                    container(Text::new(format!("{} ", &language.name)))
                        .padding(utils::styles::row_adjusted_padding()),
                );
                if i == languages.len() - 1 {
                    content = content.push(
                        container(Text::new(format!("({language_levels})")))
                            .padding(utils::styles::row_adjusted_padding()),
                    );
                } else {
                    content = content.push(
                        container(Text::new(format!("({language_levels}), ")))
                            .padding(utils::styles::row_adjusted_padding()),
                    );
                }
            }

            container(content).padding(styles::SUBSECTION_PADDING)
        } else {
            container(row![])
        };

        let subraces = if let Some(subraces) = &self.subraces {
            let mut content = column![];

            let subrace_list = {
                let mut inner = column![Text::new("Select a subrace: ")
                    .font(utils::styles::bold_font())
                    .size(styles::SUBSECTION_TITLE_SIZE)];

                // Create radio options for each subrace
                for subrace in subraces {
                    let toggle = container(radio(&subrace.name, subrace, selected_subrace, |v| {
                        Message::SubraceSelected(v.clone())
                    }))
                    .padding(styles::SUBSECTION_PADDING);
                    inner = inner.push(toggle);
                }

                // Display subrace info
                if let Some(selected) = selected_subrace {
                    inner = inner.push(selected.view())
                }

                inner
            };
            content = content.push(subrace_list);

            container(content).padding(styles::SUBSECTION_PADDING)
        } else {
            container(column![])
        };

        container(column![
            title, summary, line, asi, age, size, speed, traits, languages, subraces
        ])
        .into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subrace {
    /// The name of the subrace.
    pub name: String,

    /// The description of the subrace.
    pub summary: Summary,

    /// The ability score increases provided by the subrace.
    pub asi: Option<Vec<ASI>>,

    /// The languages provided by the subrace.
    pub languages: Option<Vec<Language>>,

    /// The traits provided by the subrace.
    pub traits: Option<Vec<Trait>>,
}

impl Subrace {
    fn view(&self) -> Element<Message> {
        let line = || container(horizontal_rule(1.0)).padding(styles::SUBRACE_LINE_PADDING);

        let title = container(
            container(Text::new(&self.name).size(utils::styles::TITLE_FONT_SIZE))
                .center_x(Length::Fill)
                .padding(utils::styles::TITLE_INNER_PAD)
                .style(utils::styles::title),
        )
        .padding(styles::SUBRACE_TITLE_PADDING);

        let summary = self
            .summary
            .view(
                styles::SUBRACE_PADDING,
                styles::SUBRACE_SUMMARY_PADDING,
                styles::SUBRACE_SUMMARY_SUBSECTION_PADDING,
            )
            .map(|_| Message::SubraceSelected(self.clone()));

        let asi = if let Some(asi_list) = &self.asi {
            let mut content = row![Text::new("Ability Score Increase: ")
                .font(utils::styles::bold_font())
                .size(styles::SUBSECTION_TITLE_SIZE)];

            // FIXME: https://github.com/aryan-regmi/dregg/issues/3
            let mut asi_text = String::with_capacity(128);
            for asi in asi_list {
                if asi.attribute == Attribute::Any {
                    asi_text.push_str("You can ");
                } else {
                    asi_text.push_str("Your ");
                }
                asi_text.push_str(&asi.to_text());
            }
            content = content.push(
                container(Text::new(asi_text)).padding(utils::styles::row_adjusted_padding()),
            );
            container(content).padding(styles::SUBRACE_PADDING)
        } else {
            container(row![])
        };

        let traits = if let Some(traits) = &self.traits {
            let mut content = column![];
            for tr in traits {
                let name = Text::new(format!("{}: ", tr.name))
                    .font(utils::styles::bold_font())
                    .size(styles::SUBSECTION_TITLE_SIZE);
                let summary = container(Text::new(&tr.summary))
                    .padding(utils::styles::row_adjusted_padding());
                content = content.push(row![name, summary])
            }
            container(content).padding(styles::SUBRACE_PADDING)
        } else {
            container(row![])
        };

        let languages = if let Some(languages) = &self.languages {
            let mut content = row![Text::new("Languages: ")
                .font(utils::styles::bold_font())
                .size(styles::SUBSECTION_TITLE_SIZE)];

            for (i, language) in languages.iter().enumerate() {
                let language_levels = {
                    let mut txt = String::new();
                    for (i, level) in language.levels.iter().enumerate() {
                        if i == language.levels.len() - 1 {
                            txt.push_str(&format!("{level}"));
                        } else {
                            txt.push_str(&format!("{level}/"));
                        }
                    }
                    txt
                };

                content = content.push(
                    container(Text::new(format!("{} ", &language.name)))
                        .padding(utils::styles::row_adjusted_padding()),
                );
                if i == languages.len() - 1 {
                    content = content.push(
                        container(Text::new(format!("({language_levels})")))
                            .padding(utils::styles::row_adjusted_padding()),
                    );
                } else {
                    content = content.push(
                        container(Text::new(format!("({language_levels}), ")))
                            .padding(utils::styles::row_adjusted_padding()),
                    );
                }
            }

            container(content).padding(styles::SUBRACE_PADDING)
        } else {
            container(row![])
        };

        container(column![
            line(),
            line(),
            title,
            summary,
            line(),
            asi,
            traits,
            languages
        ])
        .into()
    }
}

/// Represents the messages/events handled by a `Race`.
#[derive(Debug, Clone)]
pub enum Message {
    RaceSelected(Race),
    SubraceSelected(Subrace),
}

/// Represents commands the race can send to the application.

mod styles {
    #![allow(unused)]

    use iced::{widget::container, Background, Border, Padding, Theme};

    use crate::utils;

    /// The size of the subsections of a race (i.e ASI, Age, etc.).
    pub const SUBSECTION_TITLE_SIZE: f32 = 18.0;

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

    pub const TITLE_FONT_SIZE: f32 = 32.0;

    /// The base padding for `New Character` page components.
    pub const BASE_PADDING: Padding = Padding {
        top: 5.0,
        right: 30.0,
        bottom: 5.0,
        left: 30.0,
    };

    /// Padding for the subsections in the `New Character` page.
    pub const SUBSECTION_PADDING: Padding = Padding {
        left: 30.0,
        right: 0.0,
        ..BASE_PADDING
    };

    pub const SUBRACE_PADDING: Padding = Padding {
        left: 0.0,
        ..BASE_PADDING
    };

    pub const SUBRACE_TITLE_PADDING: Padding = Padding {
        left: 0.0,
        right: 30.0,
        top: 30.0,
        bottom: 30.0,
    };

    pub const SUBRACE_SUMMARY_PADDING: Padding = Padding {
        left: 0.0,
        ..utils::styles::SUMMARY_PADDING
    };

    pub const SUBRACE_SUMMARY_SUBSECTION_PADDING: Padding = Padding {
        left: 0.0,
        ..utils::styles::SUMMARY_SUBSECTION_PADDING
    };

    pub const SUBRACE_LINE_PADDING: Padding = Padding {
        left: 0.0,
        ..utils::styles::HORIZONTAL_LINE_PADDING
    };
}
