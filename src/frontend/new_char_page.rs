use iced::{
    widget::{pane_grid, text, PaneGrid},
    Element,
};

#[derive(Debug, Clone)]
pub enum Message {
    None,
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
}

impl State {
    pub fn new() -> Self {
        // Create new pane grid and split into `menu` and `main` panes
        let (mut panes, pane) = pane_grid::State::new(Pane::Menu);
        let split = panes.split(pane_grid::Axis::Vertical, pane, Pane::Main);

        // Make the `menu` pain smaller
        panes.resize(split.expect("Invalid split").1, 0.3);

        Self { panes }
    }
}

pub fn view(state: &State) -> Element<Message> {
    let pane_grid = PaneGrid::new(&state.panes, |_pane, state, _is_maximized| {
        pane_grid::Content::new(match state {
            Pane::Menu => text("Menu Items"),
            Pane::Main => text("Main window"),
        })
        .style(style::pane_active)
    });

    pane_grid.into()
}

pub fn update(_state: &State, _message: Message) -> Action {
    Action::None
}

mod style {
    use iced::{widget::container, Background, Border, Color, Theme};

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
}
