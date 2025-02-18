use super::race::{self, Race, RaceName};
use iced::{
    widget::{
        button, column, container, pane_grid, pick_list, responsive, scrollable,
        shader::wgpu::hal::auxil::db, text, PaneGrid,
    },
    Element, Length, Padding,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    None,
    Race,
    Class,
    RaceSelected(RaceName),
}

pub enum Command {
    None,
    RaceSelected(Race),
}

pub enum Pane {
    Menu,
    Main,
}

pub struct State {
    panes: pane_grid::State<Pane>,
    menu_opt: Message,
    selected_race: Option<RaceName>,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("menu_opt", &self.menu_opt)
            .field("selected_race", &self.selected_race)
            .finish()
    }
}

impl State {
    pub fn new(selected_race: Option<RaceName>) -> Self {
        // Create new pane grid and split into `menu` and `main` panes
        let (mut panes, pane) = pane_grid::State::new(Pane::Menu);
        let split = panes.split(pane_grid::Axis::Vertical, pane, Pane::Main);

        // Make the `menu` pain smaller
        panes.resize(split.expect("Invalid split").1, 0.3);

        Self {
            panes,
            menu_opt: Message::Race,
            selected_race,
        }
    }
}

fn view_main_pane<'a>(state: &State) -> Element<'a, Message> {
    match state.menu_opt {
        Message::None => unreachable!("There is no `None` button to click"),
        Message::Race | Message::RaceSelected(_) => helpers::races_list(state),
        Message::Class => column![].into(),
    }
}

pub fn view<'a>(state: &'a State) -> Element<'a, Message> {
    let pane_grid = PaneGrid::new(&state.panes, |_pane, pane_state, _is_maximized| {
        pane_grid::Content::new(match pane_state {
            // The navigation menu pane
            Pane::Menu => {
                column![
                    helpers::menu_btn(&state, "Race", Message::Race), // .explain(Color::from_rgb8(100, 255, 50)),
                    helpers::menu_btn(&state, "Class", Message::Class),
                ]
                .spacing(1)
            }

            // The content pane
            Pane::Main => {
                column![view_main_pane(state)]
            }
        })
        .style(style::pane_active)
    });

    pane_grid.into()
}

pub fn update<'a>(state: &mut State, message: Message) -> Command {
    match message {
        Message::None => Command::None,
        Message::Race | Message::Class => {
            state.menu_opt = message;
            Command::None
        }
        Message::RaceSelected(race_name) => {
            // Send the selected race to parent state (2-way binding)
            state.selected_race = Some(race_name);
            Command::RaceSelected(race_name.into())
        }
    }
}

mod helpers {
    use iced::{
        widget::{button, column, container, pick_list, responsive, scrollable},
        Element, Length, Padding,
    };

    use crate::frontend::race::RaceName;

    use super::{style, Message, State};

    /// Creates a menu button with the given name and message to send when clicked.
    pub fn menu_btn<'a>(state: &State, name: &'a str, on_press: Message) -> Element<'a, Message> {
        let tb_pad = 10.0;
        let lr_pad_offset = (name.len() * 4) as f32;
        if state.menu_opt == on_press {
            container(responsive(move |size| {
                container(
                    button(name)
                        .padding(Padding {
                            left: (size.width / 2.) - lr_pad_offset,
                            right: 0.0,
                            top: tb_pad,
                            bottom: tb_pad,
                        })
                        .width(size.width)
                        .on_press(on_press.clone())
                        .style(style::menu_btn_clicked),
                )
                .into()
            }))
            .center_x(Length::Fill)
            .padding(2)
            .height(50)
            .into()
        } else {
            container(responsive(move |size| {
                container(
                    button(name)
                        .padding(Padding {
                            left: (size.width / 2.) - lr_pad_offset,
                            right: 0.0,
                            top: tb_pad,
                            bottom: tb_pad,
                        })
                        .width(size.width)
                        .on_press(on_press.clone())
                        .style(style::menu_btn),
                )
                .into()
            }))
            .center_x(Length::Fill)
            .padding(2)
            .height(50)
            .into()
        }
    }

    /// Creates a dropdown list of races.
    pub fn races_list<'a>(state: &State) -> Element<'a, Message> {
        let race_list = pick_list(
            &RaceName::ALL[..],
            state.selected_race,
            Message::RaceSelected,
        )
        .placeholder("Select your race:");
        let content = column![race_list];
        scrollable(content).into()
    }
}

mod style {
    use iced::{
        border::Radius,
        widget::{
            button::{self, Status},
            container,
        },
        Background, Border, Color, Theme,
    };

    pub fn pane_active(theme: &Theme) -> container::Style {
        let palette = theme.palette();

        container::Style {
            background: Some(Background::Color(palette.background)),
            border: Border {
                width: 2.0,
                color: Color::from_rgb8(0, 0, 0),
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn menu_btn(theme: &Theme, status: Status) -> button::Style {
        let palette = theme.palette();

        match status {
            Status::Active => button::Style {
                background: Some(Background::Color(palette.success)),
                text_color: palette.text,
                border: Border {
                    color: Color::from_rgb8(0, 0, 0),
                    width: 0.5,
                    radius: Radius::default(),
                },
                ..Default::default()
            },

            Status::Hovered => button::Style {
                background: Some(Background::Color(palette.danger)),
                text_color: palette.text,
                border: Border {
                    color: Color::from_rgb8(0, 0, 0),
                    width: 0.5,
                    radius: Radius::default(),
                },
                ..Default::default()
            },

            _ => button::Style {
                background: Some(Background::Color(palette.danger)),
                text_color: palette.text,
                border: Border {
                    color: Color::from_rgb8(0, 0, 0),
                    width: 0.5,
                    radius: Radius::default(),
                },
                ..Default::default()
            },
        }
    }

    pub fn menu_btn_clicked(theme: &Theme, _: Status) -> button::Style {
        let palette = theme.palette();
        button::Style {
            background: Some(Background::Color(palette.danger)),
            text_color: palette.text,
            border: Border {
                color: Color::from_rgb8(0, 0, 0),
                width: 0.5,
                radius: Radius::default(),
            },
            ..Default::default()
        }
    }
}
