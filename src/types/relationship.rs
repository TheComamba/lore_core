#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityRelationship {
    pub parent: String,
    pub child: String,
    pub role: Option<String>,
}
