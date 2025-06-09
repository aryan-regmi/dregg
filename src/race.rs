use std::fmt::Display;

use iced::{
    widget::{column, container, horizontal_rule, row, Text},
    Length,
};

use crate::utils::{self, AgeInfo, Attribute, Language, SizeInfo, Speed, Summary, Trait, ASI};

/// Represents a race.
#[derive(Debug, Clone, PartialEq)]
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
    pub traits: Vec<Trait>,

    /// The languages provided by the race.
    pub languages: Option<Vec<Language>>,
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl Race {
    pub fn view<'a, Message: 'a>(&'a self) -> iced::Element<Message> {
        let line = container(horizontal_rule(1.0)).padding(utils::styles::HORIZONTAL_LINE_PADDING);

        let title = container(
            container(Text::new(&self.name).size(utils::styles::TITLE_FONT_SIZE))
                .center_x(Length::Fill)
                .padding(utils::styles::TITLE_INNER_PAD)
                .style(utils::styles::title),
        )
        .padding(utils::styles::TITLE_OUTER_PAD);

        let summary = self.summary.view();

        let asi = if let Some(asi_list) = &self.asi {
            let mut content = row![Text::new("Ability Score Increase: ")
                .font(utils::styles::bold_font())
                .size(styles::SUBSECTION_TITLE_SIZE)];

            // TODO: Add dropdown of attributes to choose if ASI is `Any`
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

        container(column![title, summary, line, asi]).into()
    }
}

mod styles {
    #![allow(unused)]

    use iced::{widget::container, Background, Border, Padding, Theme};

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

    /// Adusts the padding for `row![]` contents.
    pub const ROW_ADJUSTED_PADDING: Padding = Padding {
        top: BASE_PADDING.top / 2.0,
        right: 0.0,
        bottom: BASE_PADDING.bottom / 2.0,
        left: 0.0,
    };
}
