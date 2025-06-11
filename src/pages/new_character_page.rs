use iced::{
    widget::{button, column, container, pane_grid, pick_list, scrollable, PaneGrid, Text},
    Element, Length,
};

use crate::{
    race::{self, Race, Subrace},
    races,
};

/// Represents the `New Character` page.
#[derive(Debug)]
pub struct NewCharacterPage {
    /// The menu and info panes of the page.
    panes: pane_grid::State<Pane>,

    /// The currently selected option in the menu pane.
    current_menu: MenuOpts,

    /// The currently selected race.
    pub(crate) selected_race: Option<Race>,

    /// The currently selected subrace.
    pub(crate) selected_subrace: Option<Subrace>,
}

impl NewCharacterPage {
    pub fn new(selected_race: Option<Race>, selected_subrace: Option<Subrace>) -> Self {
        // Ratio of the menu pane to the content pane.
        const SPLIT_RATIO: f32 = 0.2;

        // Create new pane (menu) and split it (content)
        let (mut pane_state, pane) = pane_grid::State::new(Pane::Menu);
        let split = pane_state.split(pane_grid::Axis::Vertical, pane, Pane::Content);
        pane_state.resize(split.expect("Invalid split").1, SPLIT_RATIO);

        Self {
            panes: pane_state,
            current_menu: MenuOpts::Race,
            selected_race,
            selected_subrace,
        }
    }

    pub fn update(&mut self, message: Message) -> Command {
        match message {
            Message::RaceButtonPressed => {
                self.current_menu = MenuOpts::Race;
                Command::None
            }

            Message::ClassButtonPressed => {
                self.current_menu = MenuOpts::Class;
                Command::None
            }

            Message::RaceSelected(msg) => match msg {
                race::Message::RaceSelected(race) => {
                    self.selected_race = Some(race.clone());
                    Command::RaceSelected(race)
                }

                race::Message::SubraceSelected(subrace) => {
                    self.selected_subrace = Some(subrace.clone());
                    Command::SubraceSelected(subrace)
                }
            },
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let pane_grid = PaneGrid::new(&self.panes, |_pane, pane_state, _is_maximized| {
            pane_grid::Content::new(match pane_state {
                // The navigation menu pane
                Pane::Menu => {
                    column![
                        self.create_menu_button("Race", Message::RaceButtonPressed),
                        self.create_menu_button("Class", Message::ClassButtonPressed),
                    ]
                }

                // The content pane
                Pane::Content => {
                    column![self.create_content_pane()]
                }
            })
            .style(styles::panes)
        });
        pane_grid.into()
    }

    /// Creates the content pane.
    fn create_content_pane(&self) -> Element<Message> {
        match self.current_menu {
            MenuOpts::Race => scrollable(
                column![
                    self.create_race_dropdown(races::races()),
                    self.create_race_info()
                ]
                .padding(5),
            )
            .spacing(1)
            .into(),

            MenuOpts::Class => column![].into(),
        }
    }

    /// Creates a menu button with the given label.
    fn create_menu_button<'a>(&'a self, label: &'a str, on_press: Message) -> Element<Message> {
        let style = if self.current_menu == on_press.clone().into() {
            styles::selected_menu_button
        } else {
            styles::menu_button
        };

        container(container(
            button(Text::new(label).width(Length::Fill).center())
                .style(style)
                .on_press(on_press)
                .padding(10)
                .width(Length::Fill),
        ))
        .padding(5)
        .center_x(Length::Fill)
        .into()
    }

    /// Creates a dropdown list of all availabe races.
    fn create_race_dropdown<'a>(&'a self, races: Vec<Race>) -> Element<Message> {
        let dropdown = pick_list(races, self.selected_race.as_ref(), |race| {
            Message::RaceSelected(race::Message::RaceSelected(race))
        })
        .style(styles::dropdown)
        .menu_style(styles::dropdown_item)
        .placeholder("Select your race:");

        container(scrollable(column![dropdown]))
            .padding(5)
            .center_x(Length::Fill)
            .into()
    }

    /// Creates a container to display info for the selected race.
    fn create_race_info(&self) -> Element<Message> {
        if let Some(race) = &self.selected_race {
            container(
                race.view(self.selected_subrace.as_ref())
                    .map(|msg| Message::RaceSelected(msg)),
            )
            .into()
        } else {
            column![].into()
        }
    }
}

impl Clone for NewCharacterPage {
    fn clone(&self) -> Self {
        let mut cloned = Self::new(self.selected_race.clone(), self.selected_subrace.clone());
        cloned.current_menu = self.current_menu.clone();
        cloned
    }
}

/// Represents the messages/events handled by the `NewCharacterPage`.
#[derive(Debug, Clone)]
pub enum Message {
    RaceButtonPressed,
    ClassButtonPressed,
    RaceSelected(race::Message),
}

/// Represents commands this page can send to the application.
pub enum Command {
    None,
    RaceSelected(Race),
    SubraceSelected(Subrace),
}

/// Represents a `pane` in the page.
#[derive(Debug)]
enum Pane {
    Menu,
    Content,
}

/// Represents the options in the menu pane.
#[derive(Debug, PartialEq, Clone)]
enum MenuOpts {
    Race,
    Class,
}

impl From<Message> for MenuOpts {
    fn from(value: Message) -> Self {
        match value {
            Message::RaceButtonPressed => Self::Race,
            Message::ClassButtonPressed => Self::Class,
            Message::RaceSelected(_) => unreachable!(),
        }
    }
}

mod styles {
    use iced::{
        theme::palette,
        widget::{button, container, overlay, pick_list},
        Background, Border, Color, Theme,
    };

    /// Style for the menu and info panes.
    pub fn panes(theme: &Theme) -> container::Style {
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

    /// Style for the menu buttons.
    pub fn menu_button(theme: &Theme, status: button::Status) -> button::Style {
        let palette = theme.extended_palette();
        let base = base_button(palette.primary.strong);

        match status {
            button::Status::Active | button::Status::Pressed => base,
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(palette.success.strong.color)),
                ..base
            },
            button::Status::Disabled => disabled_button(base),
        }
    }

    /// Style for the currently selected menu button.
    pub fn selected_menu_button(theme: &Theme, _: button::Status) -> button::Style {
        let palette = theme.extended_palette();
        let base = base_button(palette.primary.strong);
        button::Style {
            background: Some(Background::Color(palette.success.strong.color)),
            ..base
        }
    }

    /// The base style shared by all buttons.
    fn base_button(pair: palette::Pair) -> button::Style {
        button::Style {
            background: Some(Background::Color(pair.color)),
            text_color: pair.text,
            border: Border {
                color: Color::from_rgb8(0, 0, 0),
                width: 0.5,
                ..Border::default()
            },
            ..Default::default()
        }
    }

    /// The styling for disabled buttons.
    fn disabled_button(style: button::Style) -> button::Style {
        const ALPHA_SCALE_FACTOR: f32 = 0.5;
        button::Style {
            background: style
                .background
                .map(|bg| bg.scale_alpha(ALPHA_SCALE_FACTOR)),
            text_color: style.text_color.scale_alpha(ALPHA_SCALE_FACTOR),
            ..style
        }
    }

    /// Style for the race dropdown.
    pub fn dropdown(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
        let palette = theme.extended_palette();

        match status {
            pick_list::Status::Active | pick_list::Status::Opened => pick_list::Style {
                border: Border {
                    radius: 3.0.into(),
                    ..Default::default()
                },
                text_color: palette.background.base.text,
                placeholder_color: palette.background.weak.text,
                handle_color: palette.primary.base.color,
                background: Background::Color(palette.background.weak.color),
            },
            pick_list::Status::Hovered => pick_list::Style {
                border: Border {
                    radius: 3.0.into(),
                    ..Default::default()
                },
                text_color: palette.background.base.text,
                placeholder_color: palette.background.weak.text,
                handle_color: palette.primary.base.color,
                background: Background::Color(palette.primary.weak.color),
            },
        }
    }

    /// Style for each item in the race dropdown.
    pub fn dropdown_item(theme: &Theme) -> overlay::menu::Style {
        let palette = theme.extended_palette();

        overlay::menu::Style {
            background: Background::Color(palette.background.weak.color),
            border: Border {
                radius: 1.5.into(),
                ..Default::default()
            },
            text_color: palette.background.base.text,
            selected_text_color: palette.background.strong.text,
            selected_background: Background::Color(palette.primary.weak.color),
        }
    }
}
