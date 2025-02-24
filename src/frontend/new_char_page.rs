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

fn view_main_pane(state: &State) -> Element<Message> {
    match state.menu_opt {
        Message::None => unreachable!("There is no `None` button to click"),

        // FIXME: Change to display race data!
        Message::Race | Message::RaceSelected(_) => {
            column![helpers::races_list(state), helpers::race_info_view(state)].into()
        }

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
        widget::{button, column, container, pick_list, responsive, row, scrollable, Text},
        Element, Font, Length, Padding,
    };

    use crate::frontend::race::{Race, RaceName};

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
        container(scrollable(content))
            .padding(5)
            .center_x(Length::Fill)
            .into()
    }

    pub fn race_info_view(state: &State) -> Element<Message> {
        const PAD: u16 = 10;
        const FONT_SIZE: f32 = 28.0;
        let bold_font: Font = Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        };
        let container_pad: Padding = Padding { left: PAD as f32, right: 0.0, top: 5.0, bottom: 5.0 };

        if let Some(race) = state.selected_race {
            let race: Race = race.into();
            container(responsive(move |dim| {
                // Title
                let title = {
                let font_size = (dim.width + dim.height) / FONT_SIZE;
                container(
                    container(Text::new(race.name.clone()).size(font_size))
                        .height(Length::Shrink)
                        .padding(5)
                        .center_x(Length::Fill)
                        .style(style::race_title),
                )
                .padding(20)
                };

                // Summary
                let summary = container(Text::new(race.summary.clone()))
                    .center(Length::Fill)
                    .height(Length::Shrink);

                // ASI
                let asi = {
                let mut asi_txt = String::with_capacity(30);
                for asi in &race.asi {
                    let txt = match asi {
                        crate::frontend::race::Attribute::Strength(amount) => {
                            format!("Strength score increases by {amount}.\n")
                        }
                        crate::frontend::race::Attribute::Dexterity(amount) => {
                            format!("Dexterity score increases by {amount}.\n")
                        }
                        crate::frontend::race::Attribute::Constitution(amount) => {
                            format!("Constitution score increases by {amount}.\n")
                        }
                        crate::frontend::race::Attribute::Intelligence(amount) => {
                            format!("Intelligence score increases by {amount}.\n")
                        }
                        crate::frontend::race::Attribute::Wisdom(amount) => {
                            format!("Wisdom score increases by {amount}.\n")
                        }
                        crate::frontend::race::Attribute::Charisma(amount) => {
                            format!("Charisma score increases by {amount}.\n")
                        }
                    };
                    asi_txt.push_str(&txt);
                }
                container(column![
                    container(Text::new("Ability Score Increase: ").font(bold_font))
                    .padding(container_pad),
                    container(Text::new(asi_txt)).padding(Padding {
                        left: 20.0,
                        ..Default::default()
                    })
                ])
                .height(Length::Shrink)
                .padding(container_pad)
                };

                // Age
                let mut age = container(
                    row![
                        Text::new("Age: ").font(bold_font),
                        Text::new(format!("{} are considered an adult at {} years old. On average, they live about {} years.", race.plural_name,race.age.adult, race.age.lifespan)),
                    ].padding(container_pad)
                ).padding(container_pad);

                // Size
                let size = {
                let size_category = match race.size.size {
                    crate::frontend::race::SizeCategory::Tiny => "Tiny",
                    crate::frontend::race::SizeCategory::Small => "Small",
                    crate::frontend::race::SizeCategory::Medium => "Medium",
                    crate::frontend::race::SizeCategory::Large => "Large",
                    crate::frontend::race::SizeCategory::Gargantuan => "Gargantuan",
                };
                let size_txt = {
                    let plural_name = race.plural_name.clone();
                    let has_height = race.size.height.is_some();
                    let has_weight = race.size.weight.is_some();
                    if has_height && has_weight {
                        let height = race.size.height.unwrap();
                        let weight = race.size.height.unwrap();
                        format!("{plural_name} stand at around {height} feet tall and about {weight} pounds. Your size is {size_category}.")
                    } else if has_height && !has_weight {
                        let height = race.size.height.unwrap();
                        format!("{plural_name} stand at around {height} feet tall. Your size is {size_category}.")
                    } else if !has_height && has_weight {
                        let weight = race.size.height.unwrap();
                        format!("{plural_name} are about {weight} pounds. Your size is {size_category}.")
                    } else {
                        format!("Your size is {size_category}.")
                    }
                };
                container(row![
                    Text::new("Size: ").font(bold_font),
                    Text::new(size_txt)
                ].padding(container_pad)).padding(container_pad)
                };

                // Speed
                let speed = {
                let mut speed_txt = String::with_capacity(30);
                for speed in &race.speed {
                    match speed {
                        crate::frontend::race::Speed::Walking(distance) => speed_txt.push_str(&format!("Walking speed of {distance} feet.")),
                        crate::frontend::race::Speed::Flying(distance) => speed_txt.push_str(&format!("Flying speed of {distance} feet.")),
                        crate::frontend::race::Speed::Climbing(distance) => speed_txt.push_str(&format!("Climbing speed of {distance} feet.")),
                        crate::frontend::race::Speed::Swimming(distance) => speed_txt.push_str(&format!("Swimming speed of {distance} feet.")),
                    }
                }
                container(column![
                    container(Text::new("Speed: ").font(bold_font))
                    .padding(container_pad),
                    container(Text::new(speed_txt)).padding(Padding {
                        left: 20.0,
                        ..Default::default()
                    })
                ])
                .height(Length::Shrink)
                .padding(container_pad)
                };
                
                // TODO: Add rest of sections
                
                scrollable(column![title, summary, asi, age, size, speed]).style(style::scrollbar).into()
            }))
            .padding(2)
            .into()
        } else {
            column![].into()
        }
    }
}

mod style {

    use iced::{
        border::Radius,
        widget::{
            button::{self, Status},
            container, scrollable,
        },
        Background, Border, Color, Theme,
    };

    pub fn pane_active(theme: &Theme) -> container::Style {
        const PANE_BORDER_WIDTH: f32 = 2.0;
        let palette = theme.palette();

        container::Style {
            background: Some(Background::Color(palette.background)),
            border: Border {
                width: PANE_BORDER_WIDTH,
                color: Color::from_rgb8(0, 0, 0),
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn menu_btn(theme: &Theme, status: Status) -> button::Style {
        const MENU_BTN_BORDER_WIDTH: f32 = 0.5;
        let palette = theme.palette();

        match status {
            Status::Active => button::Style {
                background: Some(Background::Color(palette.success)),
                text_color: palette.text,
                border: Border {
                    color: Color::from_rgb8(0, 0, 0),
                    width: MENU_BTN_BORDER_WIDTH,
                    radius: Radius::default(),
                },
                ..Default::default()
            },

            Status::Hovered => button::Style {
                background: Some(Background::Color(palette.danger)),
                text_color: palette.text,
                border: Border {
                    color: Color::from_rgb8(0, 0, 0),
                    width: MENU_BTN_BORDER_WIDTH,
                    radius: Radius::default(),
                },
                ..Default::default()
            },

            _ => button::Style {
                background: Some(Background::Color(palette.danger)),
                text_color: palette.text,
                border: Border {
                    color: Color::from_rgb8(0, 0, 0),
                    width: MENU_BTN_BORDER_WIDTH,
                    radius: Radius::default(),
                },
                ..Default::default()
            },
        }
    }

    pub fn menu_btn_clicked(theme: &Theme, _: Status) -> button::Style {
        const MENU_BTN_BORDER_WIDTH: f32 = 0.5;
        let palette = theme.palette();
        button::Style {
            background: Some(Background::Color(palette.danger)),
            text_color: palette.text,
            border: Border {
                color: Color::from_rgb8(0, 0, 0),
                width: MENU_BTN_BORDER_WIDTH,
                radius: Radius::default(),
            },
            ..Default::default()
        }
    }

    pub fn race_title(theme: &Theme) -> container::Style {
        const RACE_TITLE_BORDER_WIDTH: f32 = 1.0;
        const RACE_TITLE_BORDER_RADIUS: f32 = 3.0;
        let palette = theme.palette();

        container::Style {
            background: Some(Background::Color(Color::from_rgb8(100, 100, 100))),
            border: Border {
                color: Color::from_rgb8(0, 0, 0),
                width: RACE_TITLE_BORDER_WIDTH,
                radius: RACE_TITLE_BORDER_RADIUS.into(),
            },
            ..Default::default()
        }
    }

    pub fn scrollbar(theme: &Theme, _status: scrollable::Status) -> scrollable::Style {
        let palette = theme.palette();
        scrollable::Style { 
            container: container::Style::default(),
            vertical_rail: scrollable::Rail {
                background: Some(Background::Color(palette.background)),
                border: Border::default(),
                scroller: scrollable::Scroller { 
                    color: palette.background,
                    border: Border::default()
                },
            },
            horizontal_rail: scrollable::Rail {
                background: Some(Background::Color(palette.background)),
                border: Border::default(),
                scroller: scrollable::Scroller { 
                    color: palette.background,
                    border: Border::default()
                },
            },
            gap: None,
        }
    }
}
