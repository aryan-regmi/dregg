use iced::{
    widget::{button, column, container, pane_grid, responsive, text, PaneGrid},
    Element, Length, Padding,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    None,
    Race,
    Class,
    Tst,
}

pub enum Action {
    None,
}

pub enum Pane {
    Menu,
    Main,
}

pub struct State {
    panes: pane_grid::State<Pane>,
    clicked: Message,
}

impl State {
    pub fn new() -> Self {
        // Create new pane grid and split into `menu` and `main` panes
        let (mut panes, pane) = pane_grid::State::new(Pane::Menu);
        let split = panes.split(pane_grid::Axis::Vertical, pane, Pane::Main);

        // Make the `menu` pain smaller
        panes.resize(split.expect("Invalid split").1, 0.3);

        Self {
            panes,
            clicked: Message::Race,
        }
    }
}

fn menu_btn<'a>(state: &State, name: &'a str, on_press: Message) -> Element<'a, Message> {
    let tb_pad = 10.0;
    let lr_pad_offset = (name.len() * 4) as f32;
    if state.clicked == on_press {
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

pub fn view(state: &State) -> Element<Message> {
    let pane_grid = PaneGrid::new(&state.panes, |_pane, pane_state, _is_maximized| {
        pane_grid::Content::new(match pane_state {
            Pane::Menu => column![
                menu_btn(&state, "Race", Message::Race), // .explain(Color::from_rgb8(100, 255, 50)),
                menu_btn(&state, "Class", Message::Class),
                menu_btn(&state, "Longer Name", Message::Tst),
            ]
            .spacing(1),
            Pane::Main => column![text("Main window")].padding(2).spacing(10),
        })
        .style(style::pane_active)
    });

    pane_grid.into()
}

pub fn update(state: &mut State, message: Message) -> Action {
    match message {
        Message::None => {}
        _ => state.clicked = message,
    }
    Action::None
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
