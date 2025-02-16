use super::main_menu_btn;
use crate::{Message, Screen, State};
use iced::{
    widget::{column, container, pane_grid, text, PaneGrid},
    Element, Task,
};

pub enum NewCharPagePane {
    Menu,
    Main,
}

pub struct NewCharacterPage {
    panes: pane_grid::State<NewCharPagePane>,
}

impl NewCharacterPage {
    pub fn new() -> Self {
        // Create new pane grid and split into `menu` and `main` panes
        let (mut panes, pane) = pane_grid::State::new(NewCharPagePane::Menu);
        let split = panes.split(pane_grid::Axis::Vertical, pane, NewCharPagePane::Main);

        // Make the `menu` pain smaller
        panes.resize(split.expect("Invalid split").1, 0.3);

        Self { panes }
    }

    pub fn view(&self) -> Element<Message> {
        let pane_grid = PaneGrid::new(&self.panes, |_pane, state, _is_maximized| {
            pane_grid::Content::new(match state {
                NewCharPagePane::Menu => text("Menu Items"),
                NewCharPagePane::Main => text("Main window"),
            })
            .style(style::pane_active)
        });

        container(column![pane_grid, main_menu_btn()]).into()
    }

    pub fn update(&self, _message: Message) -> Task<Message> {
        // TODO: Need to implement!
        Task::none()
    }
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
