use super::{child::Child, parent::Parent, role::Role};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityRelationship {
    pub parent: Parent,
    pub child: Child,
    pub role: Role,
}
