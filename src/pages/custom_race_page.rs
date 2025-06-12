use iced::{
    widget::{button, column, container, row, text, text_editor, text_input},
    Element, Length,
};

use crate::{race::Race, utils};

#[derive(Debug, Default)]
pub struct CustomRace {
    pub(crate) race: Race,
    pub(crate) summary_content: text_editor::Content,
    pub(crate) summary_text: String,
}

impl Clone for CustomRace {
    fn clone(&self) -> Self {
        Self {
            race: self.race.clone(),
            summary_content: text_editor::Content::with_text(&self.summary_text),
            // summary_content: text_editor::Content::new(),
            summary_text: self.summary_text.clone(),
        }
    }
}

impl CustomRace {
    pub fn new(race: Race, summary_text: String) -> Self {
        Self {
            race,
            summary_content: text_editor::Content::new(),
            summary_text,
        }
    }

    pub fn view(&self) -> Element<Message> {
        // TODO: Make sure that required fields are not empty!!!

        let title = container(
            text("Create Custom Race:")
                .size(utils::styles::TITLE_FONT_SIZE - 8.0)
                .center(),
        )
        .center_x(Length::Fill);

        let name = row![
            container(text("Name: ")).padding(utils::styles::row_adjusted_padding()),
            text_input("Enter name here...", &self.race.name).on_input(Message::NameEntered)
        ];

        let plural_name = row![
            container(text("Name (Plural): ")).padding(utils::styles::row_adjusted_padding()),
            text_input(
                "Enter plural name here...",
                self.race
                    .plural_name
                    .as_ref()
                    .unwrap_or_else(|| &self.race.name)
            )
            .on_input(Message::PluralNameEntered)
        ];

        let summary = row![
            container(text("Summary: ")).padding(utils::styles::row_adjusted_padding()),
            text_editor(&self.summary_content)
                .placeholder(&self.summary_text)
                .on_action(Message::SummaryEdited),
        ];

        let create_race_button = button("Create").on_press(Message::CreateButtonPressed);

        container(column![title, name, plural_name, summary, create_race_button,].spacing(5))
            .padding(styles::BASE_PADDING)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Command {
        match message {
            Message::None => Command::None,
            Message::InitialEntry => Command::InitialView,
            Message::NameEntered(name) => {
                self.race.name = name.clone();
                Command::UpdatedCustomRace(self.clone())
            }
            Message::PluralNameEntered(name) => {
                self.race.plural_name = Some(name);
                Command::UpdatedCustomRace(self.clone())
            }
            Message::SummaryEdited(action) => {
                self.summary_content.perform(action);
                self.summary_text = self.summary_content.text().chars().rev().collect();
                self.summary_content = text_editor::Content::with_text(&self.summary_text);
                // self.summary_text = self.summary_content.text();
                // self.summary_text.push_str(&self.summary_content.text());
                // Command::UpdatedCustomRace(self.clone())
                Command::None
            }
            Message::CreateButtonPressed => {
                self.race.summary.main = self.summary_text.clone();
                // self.summary_text.clear();
                Command::CustomRaceCreated(self.race.clone())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InitialEntry,
    None,
    NameEntered(String),
    PluralNameEntered(String),
    SummaryEdited(text_editor::Action),
    CreateButtonPressed,
}

#[derive(Debug)]
pub enum Command {
    None,
    InitialView,
    UpdatedCustomRace(CustomRace),
    CustomRaceCreated(Race),
}

mod styles {
    use iced::Padding;

    use crate::utils;

    pub const BASE_PADDING: Padding = Padding {
        top: 15.0,
        bottom: 15.0,
        ..utils::styles::BASE_PADDING
    };
}
