use iced::{
    widget::{column, container, Text},
    Padding,
};

use crate::utils;

/// Represents a summary/description.
#[derive(Debug, Clone, Default)]
pub struct Summary {
    pub main: String,
    pub subsections: Vec<(String, String)>,
}

impl PartialEq for Summary {
    fn eq(&self, other: &Self) -> bool {
        self.main == other.main && self.subsections == other.subsections
    }
}

impl Eq for Summary {}

impl Summary {
    pub fn view(
        &self,
        base_padding: Padding,
        summary_padding: Padding,
        summary_subsection_padding: Padding,
    ) -> iced::Element<()> {
        let mut content = column![];

        let main = container(Text::new(self.main.clone())).padding(summary_padding);
        content = content.push(main);

        for (section, text) in &self.subsections {
            let title_header = container(Text::new(section).font(utils::styles::bold_font()))
                .padding(base_padding);
            content = content.push(title_header);

            let text = container(Text::new(text)).padding(summary_subsection_padding);
            content = content.push(text);
        }

        container(content).into()
    }
}
