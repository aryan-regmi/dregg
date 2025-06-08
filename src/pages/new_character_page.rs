use iced::{
    widget::{button, column, container, pane_grid, PaneGrid, Text},
    Element, Length,
};

use crate::component::Component;

/// Represents the `New Character` page.
#[derive(Debug)]
pub struct NewCharacterPage {
    /// The menu and info panes of the page.
    panes: pane_grid::State<Pane>,

    /// The currently selected option in the menu pane.
    current_menu: MenuOpts,
}

impl NewCharacterPage {
    pub fn new() -> Self {
        // Ratio of the menu pane to the content pane.
        const SPLIT_RATIO: f32 = 0.2;

        // Create new pane (menu) and split it (content)
        let (mut pane_state, pane) = pane_grid::State::new(Pane::Menu);
        let split = pane_state.split(pane_grid::Axis::Vertical, pane, Pane::Content);
        pane_state.resize(split.expect("Invalid split").1, SPLIT_RATIO);

        Self {
            panes: pane_state,
            current_menu: MenuOpts::Race,
        }
    }

    /// Creates a menu button with the given label.
    fn create_menu_button<'a>(
        label: &'a str,
        on_press: Message,
        current_menu: &MenuOpts,
    ) -> Element<'a, Message> {
        let style = if *current_menu == on_press.clone().into() {
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
        .into()
    }
}

impl Component<Message, Command> for NewCharacterPage {
    fn update(&mut self, message: Message) -> Command {
        match message {
            Message::RaceButtonPressed => {
                self.current_menu = MenuOpts::Race;
                Command::None
            }

            Message::ClassButtonPressed => {
                self.current_menu = MenuOpts::Class;
                Command::None
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let pane_grid = PaneGrid::new(&self.panes, |_pane, pane_state, _is_maximized| {
            pane_grid::Content::new(match pane_state {
                // The navigation menu pane
                Pane::Menu => {
                    column![
                        Self::create_menu_button(
                            "Race",
                            Message::RaceButtonPressed,
                            &self.current_menu,
                        ),
                        Self::create_menu_button(
                            "Class",
                            Message::ClassButtonPressed,
                            &self.current_menu,
                        ),
                    ]
                }

                // The content pane
                Pane::Content => {
                    column![]
                }
            })
            .style(styles::panes)
        });
        pane_grid.into()
    }
}

/// Represents the messages/events handled by the `NewCharacterPage`.
#[derive(Debug, Clone)]
pub enum Message {
    RaceButtonPressed,
    ClassButtonPressed,
}

impl Into<MenuOpts> for Message {
    fn into(self) -> MenuOpts {
        match self {
            Message::RaceButtonPressed => MenuOpts::Race,
            Message::ClassButtonPressed => MenuOpts::Class,
        }
    }
}

/// Represents commands this page can send to the application.
pub enum Command {
    None,
}

/// Represents a `pane` in the page.
#[derive(Debug)]
enum Pane {
    Menu,
    Content,
}

/// Represents the options in the menu pane.
#[derive(Debug, PartialEq)]
enum MenuOpts {
    Race,
    Class,
}

mod styles {
    use iced::{
        theme::palette,
        widget::{button, container},
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
}
