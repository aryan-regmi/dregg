use iced::{
    widget::{column, container, Text},
    Padding,
};

/// Represents a summary/description.
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    pub main: String,
    pub subsections: Vec<(String, String)>,
    pub base_padding: Padding,
    pub summary_padding: Padding,
    pub summary_subsection_padding: Padding,
}

impl Summary {
    pub fn view<'a, Message: 'a>(&'a self) -> iced::Element<Message> {
        let mut content = column![];

        let main = container(Text::new(self.main.clone())).padding(self.summary_padding);
        content = content.push(main);

        for (section, text) in &self.subsections {
            let title_header =
                container(Text::new(section).font(styles::bold_font())).padding(self.base_padding);
            content = content.push(title_header);

            let text = container(Text::new(text)).padding(self.summary_subsection_padding);
            content = content.push(text);
        }

        container(content).into()
    }
}

pub mod styles {
    use iced::{font, widget::container, Background, Border, Font, Padding, Theme};

    pub const SECTION_FONT_SIZE: f32 = 18.0;
    pub const BASE_PADDING: Padding = Padding {
        top: 5.0,
        right: TITLE_OUTER_PAD,
        bottom: 5.0,
        left: TITLE_OUTER_PAD,
    };

    pub const TITLE_FONT_SIZE: f32 = 32.0;
    pub const TITLE_INNER_PAD: f32 = 10.0;
    pub const TITLE_OUTER_PAD: f32 = 30.0;

    pub const SUMMARY_HEADING_FONT: f32 = 24.0;
    pub const SUMMARY_PADDING: Padding = Padding {
        top: 0.0,
        right: TITLE_OUTER_PAD,
        bottom: 10.0,
        left: TITLE_OUTER_PAD,
    };
    pub const SUMMARY_SUBSECTION_PADDING: Padding = Padding {
        bottom: 10.0,
        ..SUMMARY_PADDING
    };

    const INDENT_FACTOR: f32 = 1.5;
    pub fn indented_padding() -> Padding {
        Padding {
            left: BASE_PADDING.left * INDENT_FACTOR,
            ..Default::default()
        }
    }
    pub const COLUMN_SPACING: f32 = BASE_PADDING.bottom;

    pub fn radio_padding() -> Padding {
        Padding {
            left: BASE_PADDING.left,
            top: BASE_PADDING.top,
            bottom: BASE_PADDING.bottom,
            ..Default::default()
        }
    }

    pub const HORIZONTAL_LINE_PADDING: Padding = Padding {
        right: 20.0,
        left: 20.0,
        ..BASE_PADDING
    };

    pub const SUBRACE_PADDING: Padding = Padding {
        right: 0.0,
        left: 0.0,
        ..BASE_PADDING
    };
    pub const SUBRACE_TITLE_PADDING: Padding = Padding {
        top: TITLE_OUTER_PAD,
        right: 0.0,
        bottom: TITLE_OUTER_PAD,
        left: 0.0,
    };

    pub fn row_adjusted_padding() -> Padding {
        Padding {
            top: 2.0,
            ..Default::default()
        }
    }

    pub fn bold_font() -> Font {
        Font {
            weight: font::Weight::Bold,
            ..Default::default()
        }
    }

    pub fn title(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();
        container::Style {
            background: Some(Background::Color(palette.background.weak.color)),
            border: Border {
                color: palette.background.strong.color,
                width: 1.0,
                radius: 3.into(),
            },
            ..Default::default()
        }
    }
}
