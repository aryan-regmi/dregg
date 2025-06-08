use std::fmt::Display;

use iced::{
    widget::{column, container, horizontal_rule, Text},
    Length,
};

use crate::utils::{self, Summary};

/// Represents a race.
#[derive(Debug, Clone, PartialEq)]
pub struct Race {
    /// Name of the race.
    pub name: String,

    /// Plural name of the race.
    pub plural_name: Option<String>,

    /// Description of the race.
    pub summary: Summary,
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

        container(column![title, summary, line]).into()
    }
}
