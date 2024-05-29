use super::{description::Description, descriptor::Descriptor, label::Label};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityColumn {
    pub label: Label,
    pub descriptor: Descriptor,
    pub description: Description,
}
