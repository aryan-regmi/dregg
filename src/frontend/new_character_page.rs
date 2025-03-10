use iced::{
    widget::{button, column, container, pane_grid, pick_list, scrollable, PaneGrid, Text},
    Element, Length,
};

use crate::frontend::race::Race;

use super::race::{RaceName, Subrace};

#[derive(Debug, Clone, Default)]
pub enum Message {
    /// `Race` button pressed to choose race.
    #[default]
    RaceButtonPressed,

    /// `Class` button pressed to choose race.
    ClassButtonPressed,

    /// Race has been selected.
    RaceSelected(RaceName),

    /// Race has been selected.
    SubraceSelected(Subrace),
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
    RaceSelected(RaceName),
    SubraceSelected(Subrace),
}

/// Menu options for the `New Character` page.
#[derive(Debug, Clone, PartialEq)]
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
            Message::SubraceSelected(_) => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct NewCharacterPage {
    /// Represents the two panes (menu pane and info pane).
    panes: pane_grid::State<Pane>,

    /// Currently selected menu option.
    menu_option: MenuOpts,

    /// The race that was selected.
    selected_race: Option<RaceName>,

    /// The subrace that was selected (if one exists).
    selected_subrace: Option<Subrace>,
}

impl NewCharacterPage {
    const SPLIT_RATIO: f32 = 0.2;

    pub fn new(selected_race: Option<RaceName>, selected_subrace: Option<Subrace>) -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::Menu);
        let split = panes.split(pane_grid::Axis::Vertical, pane, Pane::Info);
        panes.resize(split.expect("Invalid split").1, Self::SPLIT_RATIO);
        Self {
            panes,
            menu_option: MenuOpts::Race,
            selected_race,
            selected_subrace,
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
            Message::RaceSelected(race) => {
                self.selected_race = Some(race.clone());
                Command::RaceSelected(race)
            }
            Message::SubraceSelected(subrace) => {
                self.selected_subrace = Some(subrace.clone());
                Command::SubraceSelected(subrace)
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let pane_grid = PaneGrid::new(&self.panes, |_pane, pane_state, _is_maximized| {
            pane_grid::Content::new(match pane_state {
                // The navigation menu pane
                Pane::Menu => column![
                    self.menu_pane_button("Race", Message::RaceButtonPressed),
                    self.menu_pane_button("Class", Message::ClassButtonPressed),
                ],

                // The content pane
                Pane::Info => column![self.view_info_pane()],
            })
            .style(styles::panes)
        });
        pane_grid.into()
    }
}

impl NewCharacterPage {
    /// Creates a button in the menu pane.
    fn menu_pane_button<'a>(&'a self, name: &'a str, on_press: Message) -> Element<Message> {
        let style = if self.menu_option == on_press.clone().into() {
            styles::selected_menu_button
        } else {
            styles::menu_button
        };

        container(container(
            button(Text::new(name).width(Length::Fill).center())
                .style(style)
                .on_press(on_press)
                .padding(10)
                .width(Length::Fill),
        ))
        .padding(5.0)
        .center_x(Length::Fill)
        .into()
    }

    /// Displays the info pane.
    fn view_info_pane(&self) -> Element<Message> {
        match self.menu_option {
            MenuOpts::Race => column![self.races_list(), self.race_info()].into(),
            MenuOpts::Class => column![self.classes_list(), self.class_info()].into(),
        }
    }

    /// Creates a dropdown list of races.
    fn races_list(&self) -> Element<Message> {
        let races = pick_list(
            &RaceName::ALL[..],
            self.selected_race.as_ref(),
            Message::RaceSelected,
        )
        .style(styles::dropdown)
        .menu_style(styles::dropdown_item)
        .placeholder("Select your race:");

        container(scrollable(column![races]))
            .padding(5)
            .center_x(Length::Fill)
            .into()
    }

    /// Displays the race info.
    fn race_info(&self) -> Element<Message> {
        if let Some(race) = &self.selected_race {
            let race: Race = race.into();
            container(race.view(
                &|subrace| Message::SubraceSelected(subrace),
                self.selected_subrace.as_ref(),
            ))
            .into()
        } else {
            container(column![]).into()
        }
    }

    /// Creates a dropdown list of classes.
    fn classes_list(&self) -> Element<Message> {
        container(scrollable(column![])).into()
    }

    /// Displays the class info.
    fn class_info(&self) -> Element<Message> {
        // NOTE: Replace with this!
        //
        // if let Some(class) = &self.selected_class {
        //     let class: Class = class.into();
        //     container(class.view(
        //         &|subclass| Message::SubclassSelected(subclass),
        //         self.selected_subclass.as_ref(),
        //     ))
        //     .into()
        // } else {
        //     container(column![]).into()
        // }

        container(column![]).into()
    }
}

mod styles {
    use iced::{
        overlay,
        theme::palette,
        widget::{
            button::{self, Status},
            container, pick_list,
        },
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

    /// Style for the menu buttons.
    pub fn menu_button(theme: &Theme, status: Status) -> button::Style {
        let palette = theme.extended_palette();
        let base = base_button(palette.primary.strong);

        match status {
            Status::Active | Status::Pressed => base,
            Status::Hovered => button::Style {
                background: Some(Background::Color(palette.success.strong.color)),
                ..base
            },
            Status::Disabled => disabled_button(base),
        }
    }

    /// Style for the currently selected menu button.
    pub fn selected_menu_button(theme: &Theme, _: Status) -> button::Style {
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
