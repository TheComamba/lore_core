use diesel::{Insertable, Queryable};

use crate::sql::schema::entities;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Insertable, Queryable)]
#[diesel(table_name = entities)]
#[repr(C)]
pub(crate) struct SqlEntityColumn {
    pub label: String,
    pub descriptor: String,
    pub description: Option<String>,
}
