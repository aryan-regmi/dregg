use iced::{
    widget::{column, container, Text},
    Padding,
};

use crate::views::Component;

use super::styles;

/// Represents a summary/description.
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    /// The main summary.
    pub main: String,

    /// A list of subsections in the form (Title, Content).
    pub subsections: Vec<(String, String)>,
}

impl Component for Summary {
    type Message = ();

    type Context = ();

    type Command = ();

    fn view(&self, _ctx: Self::Context) -> iced::Element<Self::Message> {
        let mut content = column![];
        let summary_padding = Padding {
            top: 0.0,
            right: 0.0,
            bottom: 10.0,
            left: 0.0,
        };

        let main = container(Text::new(self.main.clone())).padding(summary_padding);
        content = content.push(main);

        for (section, text) in &self.subsections {
            let title_header =
                container(Text::new(section).font(styles::bold_font())).padding(Padding {
                    bottom: 10.0,
                    ..summary_padding
                });
            content = content.push(title_header);

            let text = container(Text::new(text)).padding(Padding {
                bottom: 10.0,
                ..summary_padding
            });
            content = content.push(text);
        }

        container(content).into()
    }

    fn update(&mut self, _message: Self::Message) -> Self::Command {}
}
