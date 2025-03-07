use super::utils::Summary;

#[derive(Debug)]
pub struct Class {
    /// The name of the class.
    pub name: String,

    /// The plural form of the class name.
    pub name_plural: String,

    /// The description of the class.
    pub summary: Summary,
}
