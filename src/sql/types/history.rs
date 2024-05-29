use diesel::{Insertable, Queryable};

use crate::{sql::schema::history_items, types::history::HistoryItem};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Insertable, Queryable)]
#[diesel(table_name = history_items)]
pub(crate) struct SqlHistoryItem {
    pub timestamp: i64,
    pub year: i32,
    pub day: Option<i32>,
    pub content: String,
    pub properties: Option<String>,
}

impl PartialEq<&SqlHistoryItem> for SqlHistoryItem {
    fn eq(&self, other: &&SqlHistoryItem) -> bool {
        self.timestamp == other.timestamp
            && self.year == other.year
            && self.day == other.day
            && self.content == other.content
            && self.properties == other.properties
    }
}

impl HistoryItem {
    pub(crate) fn to_sql_history_item(&self) -> SqlHistoryItem {
        SqlHistoryItem {
            timestamp: self.timestamp,
            year: self.year,
            day: self.day.to_optional_signed_int(),
            content: self.content.clone(),
            properties: self.properties.clone(),
        }
    }
}

impl SqlHistoryItem {
    pub(crate) fn to_history_item(&self) -> HistoryItem {
        HistoryItem {
            timestamp: self.timestamp,
            year: self.year,
            day: self.day.into(),
            content: self.content.clone(),
            properties: self.properties.clone(),
        }
    }
}
