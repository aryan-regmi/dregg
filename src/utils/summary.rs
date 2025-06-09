use iced::{
    widget::{column, container, Text},
    Padding,
};

use crate::utils;

/// Represents a summary/description.
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    pub main: String,
    pub subsections: Vec<(String, String)>,
    pub base_padding: Padding,
    pub summary_padding: Padding,
    pub summary_subsection_padding: Padding,
}

impl Summary {
    pub fn view<'a, Message: 'a>(&'a self) -> iced::Element<Message> {
        let mut content = column![];

        let main = container(Text::new(self.main.clone())).padding(self.summary_padding);
        content = content.push(main);

        for (section, text) in &self.subsections {
            let title_header = container(Text::new(section).font(utils::styles::bold_font()))
                .padding(self.base_padding);
            content = content.push(title_header);

            let text = container(Text::new(text)).padding(self.summary_subsection_padding);
            content = content.push(text);
        }

        container(content).into()
    }
}
