use super::{
    lore_database::LoreDatabase,
    schema::history_items::{self},
    search_params::HistoryItemSearchParams,
};
use crate::errors::{sql_loading_error, LoreCoreError};
use ::diesel::prelude::*;
use diesel::Insertable;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Insertable, Queryable)]
#[diesel(table_name = history_items)]
#[repr(C)]
pub struct HistoryItem {
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

impl LoreDatabase {
    pub fn write_history_items(&self, cols: Vec<HistoryItem>) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        for col in cols.into_iter() {
            diesel::insert_into(history_items::table)
                .values(&col)
                .execute(&mut connection)
                .map_err(|e| {
                    LoreCoreError::SqlError(
                        "Writing history item to database failed: ".to_string() + &e.to_string(),
                    )
                })?;
        }
        Ok(())
    }

    pub fn read_history_items(
        &self,
        search_params: HistoryItemSearchParams,
    ) -> Result<Vec<HistoryItem>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let mut query = history_items::table.into_boxed();
        let year = search_params.year;
        if let Some(year) = year {
            query = query.filter(history_items::year.eq(year));
        }
        let day = search_params.day;
        if day.is_some() {
            query = query.filter(history_items::day.eq(day));
        }
        let timestamp = search_params.timestamp;
        if let Some(timestamp) = timestamp {
            query = query.filter(history_items::timestamp.eq(timestamp));
        }
        let content = search_params.content;
        if content.is_some() {
            if content.is_exact {
                query = query.filter(history_items::content.eq(content.exact_text()));
            } else {
                query = query.filter(history_items::content.like(content.search_pattern()));
            }
        }
        let mut items = query.load::<HistoryItem>(&mut connection).map_err(|e| {
            sql_loading_error("history items", vec![("year", &year), ("day", &day)], e)
        })?;
        items.sort();
        Ok(items)
    }
}

pub fn extract_years(items: &[HistoryItem]) -> Vec<i32> {
    let mut years: Vec<_> = items.iter().map(|item| item.year).collect();
    years.sort();
    years.dedup();
    years
}

pub fn extract_days(items: &[HistoryItem]) -> Vec<Option<i32>> {
    let mut days: Vec<_> = items.iter().map(|item| item.day).collect();
    days.sort();
    days.dedup();
    days
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_extract_years() {
        use super::*;
        let items = vec![
            HistoryItem {
                timestamp: 0,
                year: 2021,
                day: None,
                content: "".to_string(),
                properties: None,
            },
            HistoryItem {
                timestamp: 0,
                year: 2020,
                day: None,
                content: "".to_string(),
                properties: None,
            },
            HistoryItem {
                timestamp: 0,
                year: 2020,
                day: Some(4),
                content: "".to_string(),
                properties: None,
            },
        ];
        let years = extract_years(&items);
        assert!(years == vec![2020, 2021]);
    }

    #[test]
    fn test_extract_days() {
        use super::*;
        let items = vec![
            HistoryItem {
                timestamp: 0,
                year: 2020,
                day: Some(2),
                content: "".to_string(),
                properties: None,
            },
            HistoryItem {
                timestamp: 0,
                year: 2020,
                day: Some(1),
                content: "".to_string(),
                properties: None,
            },
            HistoryItem {
                timestamp: 0,
                year: 2020,
                day: Some(1),
                content: "".to_string(),
                properties: None,
            },
            HistoryItem {
                timestamp: 0,
                year: 2020,
                day: None,
                content: "".to_string(),
                properties: None,
            },
        ];
        let days = extract_days(&items);
        assert!(days == vec![None, Some(1), Some(2)]);
    }
}
