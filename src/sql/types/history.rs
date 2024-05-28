use diesel::{Insertable, Queryable};

use crate::sql::schema::history_items;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Insertable, Queryable)]
#[diesel(table_name = history_items)]
#[repr(C)]
pub(crate) struct HistoryItem {
    pub timestamp: i64,
    pub year: i32,
    pub day: Option<i32>,
    pub content: String,
    pub properties: Option<String>,
}

impl PartialEq<&HistoryItem> for HistoryItem {
    fn eq(&self, other: &&HistoryItem) -> bool {
        self.timestamp == other.timestamp
            && self.year == other.year
            && self.day == other.day
            && self.content == other.content
            && self.properties == other.properties
    }
}
