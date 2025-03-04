#![allow(unused)]

use iced::{
    widget::{button, column, container, pane_grid, PaneGrid, Text},
    Element, Length,
};

#[derive(Debug, Clone, Default)]
pub enum Message {
    /// `Race` button pressed to choose race.
    #[default]
    RaceButtonPressed,

    /// `Class` button pressed to choose race.
    ClassButtonPressed,

    /// Race has been selected.
    RaceSelected(String),
}

/// Represents the menu and info panes of this page.
#[derive(Debug, Clone)]
enum Pane {
    Menu,
    Info,
}

/// Commands returned by the `update` function.
#[derive(Debug, Clone)]
pub enum Command {
    None,
}

/// Menu options for the `New Character` page.
#[derive(Debug, Clone)]
enum MenuOpts {
    Race,
    Class,
}

#[derive(Debug)]
pub struct NewCharacterPage {
    /// Represents the two panes (menu pane and info pane).
    panes: pane_grid::State<Pane>,

    /// Currently selected menu option.
    menu_option: MenuOpts,
}

impl NewCharacterPage {
    const SPLIT_RATIO: f32 = 0.2;

    pub fn new() -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::Menu);
        let split = panes.split(pane_grid::Axis::Vertical, pane, Pane::Info);
        panes.resize(split.expect("Invalid split").1, Self::SPLIT_RATIO);
        Self {
            panes,
            menu_option: MenuOpts::Race,
        }
    }

    pub fn update(&mut self, message: Message) -> Command {
        match message {
            Message::RaceButtonPressed => {
                self.menu_option = MenuOpts::Race;
                Command::None
            }
            Message::ClassButtonPressed => {
                self.menu_option = MenuOpts::Class;
                Command::None
            }
            Message::RaceSelected(_) => todo!(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let pane_grid = PaneGrid::new(&self.panes, |_pane, pane_state, _is_maximized| {
            pane_grid::Content::new(match pane_state {
                // The navigation menu pane
                Pane::Menu => column![
                    Self::menu_pane_button("Race", Message::RaceButtonPressed),
                    Self::menu_pane_button("Class", Message::ClassButtonPressed),
                ],

                // The content pane
                Pane::Info => column![self.view_info_pane()],
            })
        });
        pane_grid.into()
    }
}

impl NewCharacterPage {
    /// Creates a button in the menu pane.
    fn menu_pane_button(name: &str, on_press: Message) -> Element<Message> {
        container(
            container(
                button(name)
                    .width(Length::Fill)
                    .padding(10)
                    .on_press(on_press.clone()),
            )
            .center_x(Length::Fill),
        )
        .center_x(Length::Fill)
        .into()
    }

    /// Displays the info pane.
    fn view_info_pane(&self) -> Element<Message> {
        match self.menu_option {
            MenuOpts::Race => column![self.races_list()].into(),
            MenuOpts::Class => todo!(),
        }
    }

    /// Creates a dropdown list of races.
    fn races_list(&self) -> Element<Message> {
        container(Text::new("Hello")).into()
    }
}
