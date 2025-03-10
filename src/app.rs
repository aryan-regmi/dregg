use iced::{
    alignment::Horizontal,
    widget::{button, column, container},
    Border, Color, Element, Length, Task, Theme,
};

use crate::frontend::{
    new_character_page::{self, NewCharacterPage},
    race::{RaceName, Subrace},
};

#[derive(Debug, Clone)]
pub enum Message {
    MainMenuButtonPressed,
    LoadCharacterButtonPressed,
    NewCharacterButtonPressed(new_character_page::Message),
}

#[derive(Default, Debug)]
enum Page {
    #[default]
    Main,
    LoadCharacter,
    NewCharacter(NewCharacterPage),
}

impl Page {
    fn view(&self) -> Element<Message> {
        let main_menu_btn = container(button("Main Menu").on_press(Message::MainMenuButtonPressed))
            .padding(20)
            .align_x(Horizontal::Center)
            .width(Length::Fill)
            .style(|_| container::Style {
                border: Border {
                    color: Color::from_rgb8(0, 0, 0),
                    width: 2.0,
                    ..Border::default()
                },
                ..Default::default()
            })
            .into();

        match self {
            Page::Main => container(
                column![
                    Self::main_opts_button("Load Character", Message::LoadCharacterButtonPressed),
                    Self::main_opts_button(
                        "New Character",
                        Message::NewCharacterButtonPressed(new_character_page::Message::default())
                    ),
                ]
                .spacing(20),
            )
            .center(Length::Fill)
            .into(),

            Page::LoadCharacter => main_menu_btn,

            Page::NewCharacter(page) => container(column![
                page.view().map(Message::NewCharacterButtonPressed),
                main_menu_btn,
            ])
            .padding(0.5)
            .into(),
        }
    }

    /// Creates a button in the main page.
    fn main_opts_button(name: &str, on_press: Message) -> Element<Message> {
        container(
            container(button(name).padding(10).on_press(on_press.clone())).center_x(Length::Fill),
        )
        .center_x(Length::Fill)
        .into()
    }
}

#[derive(Default, Debug)]
struct RaceProps {
    /// The selected race for the character.
    selected_race: Option<RaceName>,

    /// The selected subrace, if one exists.
    selected_subrace: Option<Subrace>,
}

#[allow(unused)]
#[derive(Default, Debug)]
pub struct App {
    /// The theme of the app.
    theme: Theme,

    /// The current page being displayed.
    page: Page,

    race_props: RaceProps,
}

impl App {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            page: Page::default(),
            race_props: RaceProps {
                selected_race: None,
                selected_subrace: None,
            },
        }
    }

    pub fn title(&self) -> String {
        String::from("Dregg")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MainMenuButtonPressed => {
                self.page = Page::Main;
                Task::none()
            }
            Message::LoadCharacterButtonPressed => {
                self.page = Page::LoadCharacter;
                Task::none()
            }
            Message::NewCharacterButtonPressed(msg) => {
                self.page = Page::NewCharacter(NewCharacterPage::new(
                    self.race_props.selected_race.clone(),
                    self.race_props.selected_subrace.clone(),
                ));
                match &mut self.page {
                    Page::NewCharacter(new_character_page) => {
                        let command = new_character_page.update(msg);
                        match command {
                            new_character_page::Command::None => Task::none(),
                            new_character_page::Command::RaceSelected(race_name) => {
                                self.race_props.selected_race = Some(race_name);
                                Task::none()
                            }
                            new_character_page::Command::SubraceSelected(subrace) => {
                                self.race_props.selected_subrace = Some(subrace);
                                Task::none()
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        self.page.view()
    }
}
