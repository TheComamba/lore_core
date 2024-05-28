use diesel::{Insertable, Queryable};

use crate::{sql::schema::relationships, types::relationship::EntityRelationship};

const NO_ROLE: &str = "_NO_ROLE_";

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

impl SqlEntityRelationship {
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
