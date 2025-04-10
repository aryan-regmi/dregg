use iced::{
    widget::{button, column, container, pane_grid, pick_list, scrollable, PaneGrid, Text},
    Element, Length,
};

use crate::{
    race::Race,
    views::{common, races, Component},
};

use super::{race_component, RaceComponent};

#[derive(Clone, Debug, Default)]
pub enum Message {
    /// Button pressed to choose a race.
    #[default]
    RaceButtonPressed,

    /// Button pressed to choose a class.
    ClassButtonPressed,

    /// Race is selected.
    RaceSelected((Race, race_component::Message)),
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

    /// The race component used for views.
    race_component: RaceComponent,
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

        Self {
            panes: grid,
            race_state: race_state.clone(),
            selected_menu_opt: Default::default(),
            race_component: race_state.clone().into(),
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
            .style(component_styles::pane_grid)
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
            Message::RaceSelected((race, msg)) => {
                let mut race_component: RaceComponent = race.clone().into();
                let cmd = race_component.update(msg);
                match cmd {
                    race_component::Command::SubraceSelected(subrace) => {
                        self.race_state.subrace = Some(subrace);
                    }
                    race_component::Command::None => {}
                }
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

    /// Displays the contents each option.
    fn content_pane_view(&self) -> Element<Message> {
        match self.selected_menu_opt {
            MenuOpt::Race => scrollable(column![
                self.races_list(),
                container(column![self
                    .race_component
                    .view(self.race_state.subrace.clone())
                    .map(|_| Message::default())])
            ])
            .into(),
            MenuOpt::Class => column![].into(),
        }
    }

    /// Create a dropdown list of all the races.
    fn races_list(&self) -> Element<Message> {
        let races = pick_list(races::all_races(), Some(self.race_state.clone()), |v| {
            Message::RaceSelected((v, race_component::Message::NoSubraceSelected))
        })
        .placeholder("Select a race...")
        .style(component_styles::dropdown);
        container(scrollable(races))
            .padding(5)
            .center_x(Length::Fill)
            .into()
    }
}

mod component_styles {
    use iced::{
        widget::{container, pick_list},
        Background, Border, Color, Theme,
    };

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

    pub fn dropdown(theme: &Theme, _status: pick_list::Status) -> pick_list::Style {
        let palette = theme.extended_palette();
        pick_list::Style {
            border: Border {
                radius: 3.into(),
                ..Default::default()
            },
            text_color: palette.background.base.text,
            placeholder_color: palette.background.weak.text,
            handle_color: palette.primary.base.color,
            background: Background::Color(palette.background.weak.color),
        }
    }
}
