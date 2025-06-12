use iced::widget;

use crate::{all_races, components::race::Race};

#[derive(Debug, Clone)]
pub enum Message {
    /// Display race content.
    RaceView,

    /// Display class content.
    ClassView,

    /// Return to main menu.
    MainMenu,

    /// Update selected race.
    RaceSelected(Race),

    /// Create a custom race
    CustomRaceCreator,
}

/// Actions to communicate with parent of the component.
pub enum Action {
    /// No action required.
    None,

    /// Returns to the main menu.
    MainMenu,

    /// Update selected race.
    UpdateSelectedRace(Race),

    /// Opens the custom race creator.
    OpenCustomRaceCreator,
}

/// The different types of panes in the `NewCharacter` component.
enum Pane {
    /// Contains the menu navigation.
    Menu,

    /// The actual contents.
    Content,
}

/// The different menu options in the `NewCharacter` component.
enum MenuOpts {
    Race,
    Class,
}

/// A new character component.
pub struct NewCharacter {
    /// The menu and info panes of the page.
    panes: widget::pane_grid::State<Pane>,

    /// The currently selected menu option.
    selected_menu: MenuOpts,

    /// The race chosen by the user.
    selected_race: Option<Race>,
}

impl NewCharacter {
    pub fn new(selected_race: Option<Race>) -> Self {
        // Ratio of the menu pane to the content pane.
        const SPLIT_RATIO: f32 = 0.2;

        // Create new pane (menu) and split it (content)
        let (mut panes, pane) = widget::pane_grid::State::new(Pane::Menu);
        let split = panes.split(widget::pane_grid::Axis::Vertical, pane, Pane::Content);
        panes.resize(split.expect("Invalid split").1, SPLIT_RATIO);

        Self {
            panes,
            selected_menu: MenuOpts::Race,
            selected_race,
        }
    }
}

impl NewCharacter {
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            // Display race content
            Message::RaceView => {
                self.selected_menu = MenuOpts::Race;
                Action::None
            }

            // Display class content
            Message::ClassView => {
                self.selected_menu = MenuOpts::Class;
                Action::None
            }

            // Display main menu
            Message::MainMenu => Action::MainMenu,

            // Update selected race
            Message::RaceSelected(race) => {
                self.selected_race = Some(race.clone());
                Action::UpdateSelectedRace(race)
            }

            // Create custom race
            Message::CustomRaceCreator => Action::OpenCustomRaceCreator,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let panes = widget::pane_grid(&self.panes, |_, state, _| {
            widget::pane_grid::Content::new(match state {
                // The navigation menu pane
                Pane::Menu => {
                    widget::column![
                        self.menu_button("Race", Message::RaceView),
                        self.menu_button("Class", Message::ClassView),
                    ]
                }

                // The content pane
                Pane::Content => {
                    widget::column![widget::row![
                        self.race_dropdown(all_races()),
                        widget::button("+ Race").on_press(Message::CustomRaceCreator)
                    ],]
                }
            })
        });

        widget::column![
            panes,
            widget::button("Main Menu").on_press(Message::MainMenu)
        ]
        .into()
    }
}

impl NewCharacter {
    /// Creates a menu button.
    fn menu_button<'a>(&'a self, label: &'a str, on_press: Message) -> iced::Element<Message> {
        widget::container(widget::container(
            widget::button(widget::text(label).width(iced::Fill).center())
                .on_press(on_press)
                .width(iced::Fill),
        ))
        .center_x(iced::Fill)
        .into()
    }

    fn race_dropdown<'a>(&'a self, races: Vec<Race>) -> iced::Element<Message> {
        let dropdown = widget::pick_list(races, self.selected_race.as_ref(), |race| {
            Message::RaceSelected(race)
        });

        widget::container(widget::scrollable(dropdown))
            .center_x(iced::Fill)
            .into()
    }
}
