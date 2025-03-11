/// Represents a summary/description.
#[derive(Debug)]
pub struct Summary {
    /// The main summary.
    pub main: String,

    /// A list of subsections in the form (Title, Content).
    pub subsections: Vec<(String, String)>,
}
