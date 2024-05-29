use diesel::{Insertable, Queryable};

use crate::{sql::schema::entities, types::entity::EntityColumn};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Insertable, Queryable)]
#[diesel(table_name = entities)]
pub(crate) struct SqlEntityColumn {
    pub label: String,
    pub descriptor: String,
    pub description: Option<String>,
}

impl EntityColumn {
    pub(crate) fn to_sql_entity_column(&self) -> SqlEntityColumn {
        SqlEntityColumn {
            label: self.label.to_string(),
            descriptor: self.descriptor.to_string(),
            description: self.description.to_optional_string(),
        }
    }
}

impl SqlEntityColumn {
    pub(crate) fn to_entity_column(&self) -> EntityColumn {
        EntityColumn {
            label: self.label.clone().into(),
            descriptor: self.descriptor.as_str().into(),
            description: self.description.clone().into(),
        }
    }
}
