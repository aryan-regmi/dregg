use iced::{
    widget::{button, column, container, pane_grid, pick_list, scrollable, PaneGrid, Text},
    Element, Length,
};

use crate::{
    race::Race,
    views::{common, races, Component},
};

#[derive(Clone, Debug, Default)]
pub enum Message {
    /// Button pressed to choose a race.
    #[default]
    RaceButtonPressed,

    /// Button pressed to choose a class.
    ClassButtonPressed,

    /// Race is selected.
    RaceSelected(Race),
}

impl Into<MenuOpt> for Message {
    fn into(self) -> MenuOpt {
        match self {
            Message::RaceButtonPressed => MenuOpt::Race,
            Message::ClassButtonPressed => MenuOpt::Class,
            Message::RaceSelected(_) => unreachable!("Not a menu option"),
        }
    }
}

#[derive(Debug)]
pub enum Command {
    None,
    RaceSelected(Race),
}

/// Represents the menu and content panes of this page.
#[derive(Debug)]
enum Pane {
    Menu,
    Content,
}

// TODO: Add the rest (Background, Feats, Equipment, etc)
//
/// Represents the menu options.
#[derive(Debug, Default, PartialEq)]
enum MenuOpt {
    #[default]
    Race,
    Class,
}

/// The `New Character` page.
#[derive(Debug)]
pub struct NewCharacterComponent {
    /// Represents the two panes (menu pane and info pane).
    panes: pane_grid::State<Pane>,

    /// The race state.
    race_state: Race,

    /// Currently selected menu option.
    selected_menu_opt: MenuOpt,
    // /// Currently selected race info.
    // selected_race: Option<Race>,
}

impl NewCharacterComponent {
    /// Creates the `New Character` page.
    pub fn new(race_state: &Race) -> Self {
        // Create new pane grid with the menu as its first pane
        let (mut grid, menu_pane) = pane_grid::State::new(Pane::Menu);

        // Splits the pane into two, adding the contnet pane and resizing it
        let (_content_pane, split) = grid
            .split(pane_grid::Axis::Vertical, menu_pane, Pane::Content)
            .expect("Failed to split pane");
        grid.resize(split, 0.2);

        // Determine the selcted race
        // let selected_race: Option<Races> = if !race_state.name.is_empty() {
        //     Some(race_state.into())
        // } else {
        //     None
        // };

        Self {
            panes: grid,
            race_state: race_state.clone(),
            selected_menu_opt: Default::default(),
            // selected_race,
        }
    }
}

impl Component for NewCharacterComponent {
    type Message = Message;
    type Context = ();
    type Command = Command;

    fn view(&self, _ctx: Self::Context) -> iced::Element<Self::Message> {
        let pane_grid = PaneGrid::new(&self.panes, |_pane, pane_state, _is_maximized| {
            pane_grid::Content::new(match pane_state {
                // The navigation menu pane
                Pane::Menu => column![
                    self.menu_option("Race", Message::RaceButtonPressed),
                    self.menu_option("Class", Message::ClassButtonPressed),
                ],

                // The content pane
                Pane::Content => column![self.content_pane_view()],
            })
            .style(styles::pane_grid)
        });
        pane_grid.into()
    }

    fn update(&mut self, message: Self::Message) -> Self::Command {
        match message {
            Message::RaceButtonPressed => {
                self.selected_menu_opt = MenuOpt::Race;
                Command::None
            }
            Message::ClassButtonPressed => {
                self.selected_menu_opt = MenuOpt::Class;
                Command::None
            }
            Message::RaceSelected(race) => {
                // self.selected_race = Some(race);
                // let race: RaceComponent = race.into();
                // self.race_state = race.into();
                Command::RaceSelected(race)
            }
        }
    }
}

impl NewCharacterComponent {
    /// Creates a button for the menu option.
    fn menu_option<'a>(&'a self, name: &'a str, on_press: Message) -> Element<Message> {
        let style = if self.selected_menu_opt == on_press.clone().into() {
            common::styles::button::success
        } else {
            common::styles::button::primary
        };

        container(container(
            button(Text::new(name).width(Length::Fill).center())
                .style(style)
                .on_press(on_press)
                .padding(10)
                .width(Length::Fill),
        ))
        .padding(5)
        .center_x(Length::Fill)
        .into()
    }

    /// Displays the contents each option/
    fn content_pane_view(&self) -> Element<Message> {
        match self.selected_menu_opt {
            MenuOpt::Race => column![self.races_list(), self.race_info()].into(),
            MenuOpt::Class => column![].into(),
        }
    }

    /// Create a dropdown list of all the races.
    fn races_list(&self) -> Element<Message> {
        let races = pick_list(
            races::all_races(),
            Some(self.race_state.clone()),
            Message::RaceSelected,
        );
        container(scrollable(column![races]))
            .padding(5)
            .center(Length::Fill)
            .into()
    }

    /// Displays the info of the selected race.
    fn race_info(&self) -> Element<Message> {
        container(column![]).into()
    }
}

mod styles {
    use iced::{widget::container, Background, Border, Color, Theme};

    /// Style for the pane grid.
    pub fn pane_grid(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(Background::Color(palette.background.base.color)),
            border: Border {
                color: Color::from_rgb8(0, 0, 0),
                width: 2.0,
                ..Border::default()
            },
            ..Default::default()
        }
    }
}
