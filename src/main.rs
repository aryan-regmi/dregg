use dregg::{
    all_races,
    components::{
        custom_race_creator::{self, CustomRaceCreator},
        new_character::{self, NewCharacter},
        race::Race,
    },
};
use iced::widget;

fn main() -> iced::Result {
    iced::application("Dregg", App::update, App::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    NewCharacterView(new_character::Message),

    LoadCharacterView,

    RaceCreatorView(custom_race_creator::Message),
}

#[derive(Default)]
enum View {
    #[default]
    Main,
    NewCharacter(new_character::NewCharacter),
    LoadCharacter,
    CustomRaceCreator(custom_race_creator::CustomRaceCreator),
}

// TODO: Move props to specific structs/types.
struct App {
    /// The current view.
    view: View,

    /// The selected race.
    selected_race: Option<Race>,

    /// The available races for a character.
    available_races: Vec<Race>,

    /// The name for the custom race.
    custom_race_name: String,

    /// The plural name for the custom race.
    custom_race_plural_name: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            view: Default::default(),
            selected_race: Default::default(),
            available_races: all_races(),
            custom_race_name: Default::default(),
            custom_race_plural_name: Default::default(),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            // Create and update `NewCharacterView`
            Message::NewCharacterView(msg) => {
                let mut component =
                    NewCharacter::new(self.selected_race.clone(), self.available_races.clone());
                match component.update(msg) {
                    new_character::Action::None => {}

                    // Return to main menu
                    new_character::Action::MainMenu => {
                        self.view = View::Main;
                        return iced::Task::none();
                    }

                    // Update the selected race
                    new_character::Action::UpdateSelectedRace(race) => {
                        self.selected_race = Some(race)
                    }

                    // Open the custom race creator
                    new_character::Action::OpenCustomRaceCreator => {
                        let race_creator = CustomRaceCreator::new();
                        self.view = View::CustomRaceCreator(race_creator);
                        return iced::Task::none();
                    }
                }
                self.view = View::NewCharacter(component);
            }

            // Create and update `LoadCharacterView`
            Message::LoadCharacterView => {
                self.view = View::LoadCharacter;
            }

            // Create and update `CustomRaceCreator`
            Message::RaceCreatorView(message) => {
                self.handle_race_creator_events(message);
            }
        }

        iced::Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        use iced::widget;
        match &self.view {
            View::Main => widget::column![
                widget::button("New Character")
                    .on_press(Message::NewCharacterView(new_character::Message::RaceView)),
                widget::button("Load Character").on_press(Message::LoadCharacterView),
            ]
            .into(),

            View::NewCharacter(component) => component.view().map(Message::NewCharacterView),

            View::LoadCharacter => widget::column![widget::button("Main Menu")
                .on_press(Message::NewCharacterView(new_character::Message::MainMenu))]
            .into(),

            View::CustomRaceCreator(component) => {
                widget::column![component.view().map(Message::RaceCreatorView)]
                    .padding(10)
                    .into()
            }
        }
    }
}

impl App {
    /// Handles the `RaceCreatorView` events.
    fn handle_race_creator_events(&mut self, message: custom_race_creator::Message) {
        if let View::CustomRaceCreator(component) = &mut self.view {
            match component.update(message) {
                custom_race_creator::Action::None => {}

                custom_race_creator::Action::CreateAndReturn(race) => {
                    // Add to available races
                    self.available_races.push(race);

                    // Return to `NewCharacter` page
                    let new_character =
                        NewCharacter::new(self.selected_race.clone(), self.available_races.clone());
                    self.view = View::NewCharacter(new_character);
                }

                custom_race_creator::Action::UpdateName(name) => {
                    self.custom_race_name = name;
                }

                custom_race_creator::Action::UpdatePluralName(plural_name) => {
                    self.custom_race_plural_name = plural_name;
                }
            }
        }
    }
}
