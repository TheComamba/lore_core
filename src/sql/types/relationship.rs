use diesel::{Insertable, Queryable};

use crate::{sql::schema::relationships, types::relationship::EntityRelationship};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Insertable, Queryable)]
#[diesel(table_name = relationships)]
pub(crate) struct SqlEntityRelationship {
    pub parent: String,
    pub child: String,
    pub role: String,
}

impl EntityRelationship {
    pub(crate) fn to_sql_entity_relationship(&self) -> SqlEntityRelationship {
        SqlEntityRelationship {
            parent: self.parent.to_string(),
            child: self.child.to_string(),
            role: self.role.to_string(),
        }
    }
}

impl SqlEntityRelationship {
    pub(crate) fn to_relationship(&self) -> EntityRelationship {
        EntityRelationship {
            parent: self.parent.as_str().into(),
            child: self.child.as_str().into(),
            role: self.role.as_str().into(),
        }
    }
}
