/// Represents a summary/description.
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    /// The main summary.
    pub main: String,

    /// A list of subsections in the form (Title, Content).
    pub subsections: Vec<(String, String)>,
}
