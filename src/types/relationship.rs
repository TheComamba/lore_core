#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityRelationship {
    pub parent: String,       //TODO: Use stronger typing
    pub child: String,        //TODO: Use stronger typing
    pub role: Option<String>, //TODO: Use stronger typing
}
