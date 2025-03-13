use iced::{font, Font};

pub mod button;

pub fn bold_font() -> Font {
    Font {
        weight: font::Weight::Bold,
        ..Default::default()
    }
}

pub mod new_character_page {
    use iced::Padding;

    pub const TITLE_FONT_SIZE: f32 = 32.0;

    /// The base padding for `New Character` page components.
    pub const BASE_PADDING: Padding = Padding {
        top: 5.0,
        right: 30.0,
        bottom: 5.0,
        left: 30.0,
    };
}
