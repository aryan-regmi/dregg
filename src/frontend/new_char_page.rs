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

// TODO: Move what's possible to it's own view function (i.e. RacialTrait, etc)
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
        const PAD: u16 = 20;
        const TITLE_PAD: u16 = 20;
        const INDENT_FACTOR: f32 = 1.5;
        const FONT_SIZE: f32 = 28.0;
        let bold_font: Font = Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        };
        let container_pad: Padding = Padding { 
            left: PAD as f32,
            right: 0.0,
            top: 5.0,
            bottom: 5.0 
        };
        let padded_indent = Padding {
            left: container_pad.left * INDENT_FACTOR,
            ..Default::default() 
        };
        
        if let Some(race) = state.selected_race {
            let race: Race = race.into();
            container(responsive(move |dim| {
                let title = {
                    let font_size = (dim.width + dim.height) / FONT_SIZE;
                    container(
                        container(Text::new(race.name.clone()).size(font_size))
                            .height(Length::Shrink)
                            .center_x(Length::Fill)
                            .style(style::race_title),
                    )
                    .padding(TITLE_PAD)
                };
              
                let summary = container(Text::new(race.summary.clone()))
                    .center(Length::Fill)
                    .height(Length::Shrink);

                let asi = {
                    let mut content = column![
                        Text::new("Ability Score Increase: ").font(bold_font)
                    ].spacing(container_pad.bottom);
                    
                    for asi in &race.asi {
                        content = content.push(container(
                            Text::new(asi.text())
                        ).padding(padded_indent));
                    }
                    
                    container(content)
                        .height(Length::Shrink)
                        .padding(container_pad)
                };

                let age = {
                    let age_txt = format!(
                        "{} are considered adults at {} years old. On average, they live about {} years.",
                        race.plural_name,
                        race.age.adult,
                        race.age.lifespan,
                    );
                    container(row![
                        Text::new("Age: ").font(bold_font),
                        Text::new(age_txt)
                    ])
                        .height(Length::Shrink)
                        .padding(container_pad)
                };

                let size = {
                    let category = race.size.size.text();
                    let has_height = race.size.height.is_some();
                    let has_weight = race.size.weight.is_some();
                    
                    let size_txt = if has_height && has_weight  {
                        format!(
                            "{} stand at around {} feet tall and about {} pounds. Your size is {}.",
                            race.plural_name,
                            race.size.height.unwrap(),
                            race.size.weight.unwrap(),
                            category,
                        )
                    } else if has_height && !has_weight {
                        format!(
                            "{} stand at around {} feet tall. Your size is {}.",
                            race.plural_name,
                            race.size.height.unwrap(),
                            category,
                        )
                    } else if !has_height && has_weight {
                        format!(
                            "{} are about {} pounds. Your size is {}.",
                            race.plural_name,
                            race.size.weight.unwrap(),
                            category,
                        )
                    } else {
                        format!("Your size is {}.", category)
                    };

                    container(row![
                        Text::new("Size: ").font(bold_font),
                        Text::new(size_txt),
                    ])
                        .height(Length::Shrink)
                        .padding(container_pad)
                };

                let speed = {
                    let mut content = column![
                        Text::new("Speed: ").font(bold_font)
                    ].spacing(container_pad.bottom);

                    for speed in &race.speed {
                        content = content.push(container(
                                Text::new(speed.text())
                        ).padding(padded_indent));
                    }

                    container(content)
                        .height(Length::Shrink)
                        .padding(container_pad)
                };
              
                let proficiencies = {
                    let mut content = column![
                        Text::new("Proficiencies: ").font(bold_font)
                    ].spacing(container_pad.bottom);

                    for proficiency_list in &race.proficiencies {
                        content = content.push(container(
                            Text::new(proficiency_list.text("You gain proficiency with "))
                        ).padding(padded_indent));
                    }
                        
                    container(content)
                        .height(Length::Shrink)
                        .padding(container_pad)
                };

                let racial_traits = {
                    let mut content = column![].spacing(container_pad.bottom * 2.0);

                    for racial_trait in &race.traits {
                        let mut inner_content = row![];
                        let name = Text::new(format!("{}: ", racial_trait.name)).font(bold_font);
                        let summary = Text::new(racial_trait.summary);
                        inner_content = inner_content.push(name);
                        inner_content = inner_content.push(summary);
                        content = content.push(inner_content);
                    }

                    container(content)
                        .height(Length::Shrink)
                        .padding(container_pad)
                };

                let languages = {
                    let mut languages_txt = String::with_capacity(30);
                    let num_languages = race.languages.len();
                    for (i, language) in race.languages.iter().enumerate() {
                        if i == num_languages - 1 {
                            languages_txt.push_str("and ");
                            languages_txt.push_str(&format!("{}.", language));
                        } else {
                            if num_languages > 2 {
                                languages_txt.push_str(", ");
                            }
                            languages_txt.push_str(&format!("{} ", language));
                        }
                    }

                    container(row![
                        Text::new("Languages: ").font(bold_font),
                        Text::new(format!("You can speak, read, and write {}", languages_txt))
                    ])
                        .height(Length::Shrink)
                        .padding(container_pad)
                };
                
                scrollable(column![
                    title,
                    summary,
                    asi,
                    age,
                    size,
                    speed,
                    proficiencies,
                    racial_traits,
                    languages,
                ])
                    .style(style::scrollbar)
                    .into()
                    
            }))
                .height(Length::Shrink)
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
