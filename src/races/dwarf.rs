use crate::{
    race::Race,
    utils::{self, Summary},
};

pub fn dwarf() -> Race {
    Race {
        name: "Dwarf".into(),
        plural_name: Some("Dwarves".into()),
        summary: summary(),
    }
}

fn summary() -> Summary {
    Summary {
        main: "Summary Here".into(),
        subsections: vec![("H1".into(), "Paragraph".into())],
        base_padding: utils::styles::BASE_PADDING,
        summary_padding: utils::styles::SUMMARY_PADDING,
        summary_subsection_padding: utils::styles::SUMMARY_SUBSECTION_PADDING,
    }
}
