use iced::widget::pane_grid::{self};

#[derive(Debug)]
pub struct NewCharacterPage {
    /// Represents the two panes (menu pane and info pane).
    panes: pane_grid::State<Pane>,
}

impl NewCharacterPage {
    pub fn new() -> Self {
        // Create new pane grid with the menu as its first pane
        let (mut panes, menu_pane) = pane_grid::State::new(Pane::Menu);

        // Splits the pane into two, adding the contnet pane and resizing it
        let (_content_pane, split) = panes
            .split(pane_grid::Axis::Vertical, menu_pane, Pane::Content)
            .expect("Failed to split pane");
        panes.resize(split, 0.2);

        Self { panes }
    }
}

#[derive(Debug, Clone, Copy)]
enum Pane {
    /// The pane that contains the menu options.
    Menu,

    /// The pane that contains the actual contents.
    Content,
}
