use diesel::{Insertable, Queryable};

use crate::sql::{relationship::EntityRelationship, schema::relationships};

const NO_ROLE: &str = "_NO_ROLE_";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Insertable, Queryable)]
#[diesel(table_name = relationships)]
pub(crate) struct EntityRelationshipSqlRepresentation {
    pub parent: String,
    pub child: String,
    pub role: String,
}

impl EntityRelationship {
    pub(crate) fn to_sql_representation(&self) -> EntityRelationshipSqlRepresentation {
        EntityRelationshipSqlRepresentation {
            parent: self.parent.clone(),
            child: self.child.clone(),
            role: role_to_sql(&self.role),
        }
    }
}

pub(crate) fn role_to_sql(role: &Option<String>) -> String {
    match role {
        Some(role) => role.clone(),
        None => NO_ROLE.to_string(),
    }
}

impl EntityRelationshipSqlRepresentation {
    pub(crate) fn to_relationship(&self) -> EntityRelationship {
        EntityRelationship {
            parent: self.parent.clone(),
            child: self.child.clone(),
            role: match self.role.as_str() {
                NO_ROLE => None,
                _ => Some(self.role.clone()),
            },
        }
    }
}
