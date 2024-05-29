use super::label::Label;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityColumn {
    pub label: Label,
    pub descriptor: String,          //TODO: Use stronger typing
    pub description: Option<String>, //TODO: Use stronger typing
}
