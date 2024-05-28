#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct EntityColumn {
    pub label: String,
    pub descriptor: String,
    pub description: Option<String>,
}
