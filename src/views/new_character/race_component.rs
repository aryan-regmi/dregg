use std::{fmt::Display, marker::PhantomData};

use crate::{
    common::{Age, Height, Language, Range, RangeTrait, Size, Speed, Trait, Weight, ASI},
    race::{Race, Subrace},
    views::{common::Summary, new_character::races::dwarf, styles, Component},
};

use iced::{
    widget::{column, container, horizontal_rule, radio, row, Text},
    Length, Padding,
};

#[derive(Clone, Debug)]
pub enum Message {
    NoSubraceSelected,
    SubraceSelected(Subrace),
}

#[derive(Debug)]
pub enum Command {
    None,
    SubraceSelected(Subrace),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RaceComponent<'a> {
    /// The name of the race.
    pub name: String,

    /// The plural form of the race's name.
    pub name_plural: String,

    /// The description of the race.
    pub summary: Summary,

    /// The ability score increases provided by the race.
    pub asi: Option<Vec<ASI>>,

    /// The age info of the race.
    pub age: AgeInfo,

    /// The size info of the race.
    pub size: SizeInfo,

    /// The speed of the race.
    pub speed: Vec<Speed>,

    /// A list of traits provided by the race.
    pub traits: Vec<Trait>,

    /// The languages provided by the race.
    pub languages: Option<Vec<Language>>,

    /// Subraces that a character may choose.
    pub subrace_options: Option<Vec<SubraceComponent>>,

    /// Required for the lifetime.
    pub selected_subrace: Option<SubraceComponent>,

    /// Required for the lifetime.
    pub _marker: &'a PhantomData<()>,
}

impl<'a> Default for RaceComponent<'a> {
    fn default() -> Self {
        Self {
            name: Default::default(),
            name_plural: Default::default(),
            summary: Summary {
                main: "".into(),
                subsections: vec![],
            },
            asi: Default::default(),
            age: AgeInfo {
                adult: Age(0),
                lifespan: Age(0),
            },
            size: SizeInfo {
                category: Size::default(),
                height: None,
                weight: None,
            },
            speed: Default::default(),
            traits: Default::default(),
            languages: Default::default(),
            subrace_options: Default::default(),
            selected_subrace: None,
            _marker: &PhantomData,
        }
    }
}

impl<'a> Component for RaceComponent<'a> {
    type Message = Message;

    type Context = Option<SubraceComponent>;

    type Command = Command;

    fn view(&self, ctx: Self::Context) -> iced::Element<Self::Message> {
        if self.name != "" {
            let title = container(
                container(Text::new(&self.name).size(styles::new_character_page::TITLE_FONT_SIZE))
                    .center_x(Length::Fill)
                    .padding(10)
                    .style(component_styles::title),
            )
            .padding(Padding {
                top: 10.0,
                bottom: 10.0,
                ..Default::default()
            });

            let summary = self.summary.view(()).map(|_| Message::NoSubraceSelected);

            let asi = if let Some(asi_list) = &self.asi {
                let mut content = row![Text::new("Ability Score Increase: ")
                    .font(styles::bold_font())
                    .size(component_styles::SUBSECTION_TITLE_SIZE)];

                // TODO: Add dropdown of atrributes if ASI is `Any`
                let mut asi_text = String::with_capacity(128);
                for asi in asi_list {
                    if let ASI::Any(_) = asi {
                        asi_text.push_str("You can ");
                    } else {
                        asi_text.push_str("Your ");
                    }
                    asi_text.push_str(&asi.text());
                }
                content = content.push(
                    container(Text::new(asi_text))
                        .padding(styles::new_character_page::ROW_ADJUSTED_PADDING),
                );

                container(content).padding(styles::new_character_page::SUBSECTION_PADDING)
            } else {
                container(row![])
            };

            let age = {
                let age_txt = format!(
                    "{} are considered adults at {} years old. On average, they live to {} years.",
                    self.name_plural, self.age.adult, self.age.lifespan
                );
                container(row![
                    Text::new("Age: ")
                        .font(styles::bold_font())
                        .size(component_styles::SUBSECTION_TITLE_SIZE),
                    container(Text::new(age_txt))
                        .padding(styles::new_character_page::ROW_ADJUSTED_PADDING)
                ])
                .padding(styles::new_character_page::SUBSECTION_PADDING)
            };

            let size = {
                let content = row![
                    Text::new("Size: ")
                        .font(styles::bold_font())
                        .size(component_styles::SUBSECTION_TITLE_SIZE),
                    container(Text::new(self.size.text(&self.name_plural)))
                        .padding(styles::new_character_page::ROW_ADJUSTED_PADDING)
                ];
                container(content).padding(styles::new_character_page::SUBSECTION_PADDING)
            };

            let speed = if self.speed.len() > 0 {
                let mut content = row![Text::new("Speed: ")
                    .font(styles::bold_font())
                    .size(component_styles::SUBSECTION_TITLE_SIZE)];
                for speed in &self.speed {
                    content = content.push(
                        container(Text::new(format!("{speed}")))
                            .padding(styles::new_character_page::ROW_ADJUSTED_PADDING),
                    )
                }
                container(content).padding(styles::new_character_page::SUBSECTION_PADDING)
            } else {
                container(row![])
            };

            let traits = if self.traits.len() > 0 {
                let mut content = column![];
                for tr in &self.traits {
                    let name = Text::new(format!("{}: ", tr.name))
                        .font(styles::bold_font())
                        .size(component_styles::SUBSECTION_TITLE_SIZE);
                    let summary = container(Text::new(&tr.summary))
                        .padding(styles::new_character_page::ROW_ADJUSTED_PADDING);
                    content = content.push(
                        row![name, summary].padding(styles::new_character_page::SUBSECTION_PADDING),
                    )
                }
                container(content).padding(styles::new_character_page::SUBSECTION_PADDING)
            } else {
                container(row![])
            };

            let languages = if let Some(languages) = &self.languages {
                let mut content = row![Text::new("Languages: ")
                    .font(styles::bold_font())
                    .size(component_styles::SUBSECTION_TITLE_SIZE)];

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
                            .padding(styles::new_character_page::ROW_ADJUSTED_PADDING),
                    );
                    if i == languages.len() - 1 {
                        content = content.push(
                            container(Text::new(format!("({language_levels})")))
                                .padding(styles::new_character_page::ROW_ADJUSTED_PADDING),
                        );
                    } else {
                        content = content.push(
                            container(Text::new(format!("({language_levels}), ")))
                                .padding(styles::new_character_page::ROW_ADJUSTED_PADDING),
                        );
                    }
                }

                container(content).padding(styles::new_character_page::SUBSECTION_PADDING)
            } else {
                container(row![])
            };

            let subraces = if let Some(subraces) = &self.subrace_options {
                let mut content = column![];

                let subrace_list = {
                    let mut inner = column![Text::new("Select a subrace: ")
                        .font(styles::bold_font())
                        .size(component_styles::SUBSECTION_TITLE_SIZE)];

                    // Create radio options for each subrace
                    for subrace in subraces {
                        let toggle = container(radio(
                            &subrace.name,
                            subrace,
                            self.selected_subrace.as_ref(),
                            |v| Message::SubraceSelected(v.as_subrace()),
                        ));
                        inner = inner.push(toggle);
                    }

                    // Display subrace info
                    if let Some(selected) = &self.selected_subrace {
                        inner = inner.push(selected.view(()))
                    }

                    inner
                };
                content = content.push(subrace_list);

                container(content).padding(styles::new_character_page::SUBSECTION_PADDING)
            } else {
                container(column![])
            };

            column![
                horizontal_rule(1.0),
                title,
                summary,
                horizontal_rule(1.0),
                asi,
                age,
                size,
                speed,
                traits,
                languages,
                subraces
            ]
            .padding(styles::new_character_page::BASE_PADDING)
            .into()
        } else {
            column![].into()
        }
    }

    fn update(&mut self, message: Self::Message) -> Self::Command {
        match message {
            Message::NoSubraceSelected => Command::None,
            Message::SubraceSelected(s) => {
                self.selected_subrace = Some(SubraceComponent::from_subrace(&s));
                Command::SubraceSelected(s)
            }
        }
    }
}

impl<'a> Display for RaceComponent<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl<'a> From<RaceComponent<'a>> for Race {
    fn from(value: RaceComponent) -> Self {
        Self {
            name: value.name,
            asi: value.asi,
            size: value.size.category,
            speed: value.speed,
            languages: value.languages,
            traits: value.traits,
            subrace: None,
        }
    }
}

impl<'a> From<Race> for RaceComponent<'a> {
    fn from(value: Race) -> Self {
        // NOTE: Keep in sync with each added race!
        match value.name.as_str() {
            "Dwarf" => dwarf::dwarf(),
            "" => Self::default(),
            _ => unreachable!("Invalid race type"),
        }
    }
}

/// Represents the defining ages of a race.
#[derive(Debug, Clone, PartialEq)]
pub struct AgeInfo {
    /// The age at which a character is considered an adult.
    pub adult: Age,

    /// The average lifespan of a character.
    pub lifespan: Age,
}

/// Represents the size info for a race.
#[derive(Debug, Clone, PartialEq)]
pub struct SizeInfo {
    /// The size category.
    pub category: Size,

    /// The height in feet and inches.
    pub height: Option<Range<Height>>,

    /// The weight in pounds (lb).
    pub weight: Option<Range<Weight>>,
}

impl std::fmt::Display for Range<Height> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_singular() {
            f.write_fmt(format_args!("{}", self.start))
        } else {
            f.write_fmt(format_args!("{} - {}", self.start, self.end))
        }
    }
}

impl std::fmt::Display for Range<Weight> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_singular() {
            f.write_fmt(format_args!("{}", self.start))
        } else {
            f.write_fmt(format_args!("{} - {}", self.start, self.end))
        }
    }
}

impl SizeInfo {
    pub fn text(&self, name_plural: &str) -> String {
        let category = &self.category;
        let has_height = self.height.is_some();
        let has_weight = self.weight.is_some();
        let txt = if has_height && has_weight {
            let height = self.height.as_ref().unwrap();
            let weight = self.weight.as_ref().unwrap();
            format!(
                "{name_plural} stand at around {height} tall and weigh about {weight}. Your size is {category}.",
            )
        } else if has_height && !has_weight {
            let height = self.height.as_ref().unwrap();
            format!("{name_plural} stand at around {height} tall. Your size is {category}.",)
        } else if !has_height && has_weight {
            let weight = self.weight.as_ref().unwrap();
            format!("{name_plural} stand at around {weight} tall. Your size is {category}.",)
        } else {
            format!("Your size is {category}.",)
        };
        txt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubraceComponent {
    /// The name of the subrace.
    pub name: String,

    /// The description of the race.
    pub summary: Summary,

    /// The ability score increases provided by the subrace.
    pub asi: Option<Vec<ASI>>,

    /// The languages provided by the subrace.
    pub languages: Option<Vec<Language>>,

    /// The traits provided by the subrace.
    pub traits: Vec<Trait>,
}

impl Component for SubraceComponent {
    type Message = Message;

    type Context = ();

    type Command = ();

    fn view(&self, _ctx: Self::Context) -> iced::Element<Self::Message> {
        let title = container(
            container(Text::new(&self.name).size(styles::new_character_page::TITLE_FONT_SIZE))
                .center_x(Length::Fill)
                .padding(10)
                .style(component_styles::title),
        )
        .padding(Padding {
            top: 10.0,
            bottom: 10.0,
            ..Default::default()
        });

        let summary = self.summary.view(()).map(|_| Message::NoSubraceSelected);

        let asi = if let Some(asi_list) = &self.asi {
            let mut content = row![Text::new("Ability Score Increase: ")
                .font(styles::bold_font())
                .size(component_styles::SUBSECTION_TITLE_SIZE)];

            // TODO: Add dropdown of atrributes if ASI is `Any`
            let mut asi_text = String::with_capacity(128);
            for asi in asi_list {
                if let ASI::Any(_) = asi {
                    asi_text.push_str("You can ");
                } else {
                    asi_text.push_str("Your ");
                }
                asi_text.push_str(&asi.text());
            }
            content = content.push(
                container(Text::new(asi_text))
                    .padding(styles::new_character_page::ROW_ADJUSTED_PADDING),
            );

            container(content).padding(styles::new_character_page::SUBSECTION_PADDING)
        } else {
            container(row![])
        };

        let traits = if self.traits.len() > 0 {
            let mut content = column![];
            for tr in &self.traits {
                let name = Text::new(format!("{}: ", tr.name))
                    .font(styles::bold_font())
                    .size(component_styles::SUBSECTION_TITLE_SIZE);
                let summary = container(Text::new(&tr.summary))
                    .padding(styles::new_character_page::ROW_ADJUSTED_PADDING);
                content = content.push(
                    row![name, summary].padding(styles::new_character_page::SUBSECTION_PADDING),
                )
            }
            container(content).padding(styles::new_character_page::SUBSECTION_PADDING)
        } else {
            container(row![])
        };

        let languages = if let Some(languages) = &self.languages {
            let mut content = row![Text::new("Languages: ")
                .font(styles::bold_font())
                .size(component_styles::SUBSECTION_TITLE_SIZE)];

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
                        .padding(styles::new_character_page::ROW_ADJUSTED_PADDING),
                );
                if i == languages.len() - 1 {
                    content = content.push(
                        container(Text::new(format!("({language_levels})")))
                            .padding(styles::new_character_page::ROW_ADJUSTED_PADDING),
                    );
                } else {
                    content = content.push(
                        container(Text::new(format!("({language_levels}), ")))
                            .padding(styles::new_character_page::ROW_ADJUSTED_PADDING),
                    );
                }
            }

            container(content).padding(styles::new_character_page::SUBSECTION_PADDING)
        } else {
            container(row![])
        };

        container(column![
            horizontal_rule(1.0),
            title,
            summary,
            horizontal_rule(1.0),
            asi,
            traits,
            languages
        ])
        .padding(styles::new_character_page::SUBSECTION_PADDING)
        .into()
    }

    fn update(&mut self, _message: Self::Message) -> Self::Command {}
}

impl SubraceComponent {
    pub fn from_subrace(subrace: &Subrace) -> Self {
        match subrace.name.as_str() {
            "Hill Dwarf" => dwarf::hill_dwarf(),
            "Mountain Dwarf" => dwarf::mountain_dwarf(),
            _ => panic!("Not a registered/valid subrace"),
        }
    }

    pub fn as_subrace(&self) -> Subrace {
        Subrace {
            name: self.name.clone(),
            asi: self.asi.clone(),
            languages: self.languages.clone(),
            traits: self.traits.clone(),
        }
    }
}

mod component_styles {
    use iced::{widget::container, Background, Border, Theme};

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
}
