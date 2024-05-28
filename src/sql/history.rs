use ::diesel::prelude::*;

use crate::{
    errors::{sql_loading_error, LoreCoreError},
    types::history::HistoryItem,
};

use super::{
    lore_database::LoreDatabase, schema::history_items, search_params::HistoryItemSearchParams,
    types::history::SqlHistoryItem,
};

impl LoreDatabase {
    pub fn write_history_items(&self, cols: Vec<HistoryItem>) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        for col in cols.into_iter() {
            let col = col.to_sql_history_item();
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

    pub fn redate_history_item(
        &self,
        timestamp: i64,
        year: i32,
        day: Option<i32>,
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::update(history_items::table.filter(history_items::timestamp.eq(timestamp)))
            .set((history_items::year.eq(year), history_items::day.eq(day)))
            .execute(&mut connection)
            .map_err(|e| {
                LoreCoreError::SqlError(
                    "Redating history item in database failed: ".to_string() + &e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn delete_history_item(&self, timestamp: i64) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::delete(history_items::table.filter(history_items::timestamp.eq(timestamp)))
            .execute(&mut connection)
            .map_err(|e| {
                LoreCoreError::SqlError(
                    "Deleting history item from database failed: ".to_string() + &e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn change_history_item_content(
        &self,
        timestamp: i64,
        content: &str,
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::update(history_items::table.filter(history_items::timestamp.eq(timestamp)))
            .set(history_items::content.eq(content))
            .execute(&mut connection)
            .map_err(|e| {
                LoreCoreError::SqlError(
                    "Changing history item content in database failed: ".to_string()
                        + &e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn change_history_item_properties(
        &self,
        timestamp: i64,
        properties: &Option<String>,
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::update(history_items::table.filter(history_items::timestamp.eq(timestamp)))
            .set(history_items::properties.eq(properties))
            .execute(&mut connection)
            .map_err(|e| {
                LoreCoreError::SqlError(
                    "Changing history item properties in database failed: ".to_string()
                        + &e.to_string(),
                )
            })?;
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
        let mut items: Vec<_> = query
            .load::<SqlHistoryItem>(&mut connection)
            .map_err(|e| {
                sql_loading_error("history items", vec![("year", &year), ("day", &day)], e)
            })?
            .into_iter()
            .map(|item| item.to_history_item())
            .collect();
        items.sort();
        Ok(items)
    }
}

pub fn extract_years(items: &[SqlHistoryItem]) -> Vec<i32> {
    let mut years: Vec<_> = items.iter().map(|item| item.year).collect();
    years.sort();
    years.dedup();
    years
}

pub fn extract_days(items: &[SqlHistoryItem]) -> Vec<Option<i32>> {
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
            SqlHistoryItem {
                timestamp: 0,
                year: 2021,
                day: None,
                content: "".to_string(),
                properties: None,
            },
            SqlHistoryItem {
                timestamp: 0,
                year: 2020,
                day: None,
                content: "".to_string(),
                properties: None,
            },
            SqlHistoryItem {
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
            SqlHistoryItem {
                timestamp: 0,
                year: 2020,
                day: Some(2),
                content: "".to_string(),
                properties: None,
            },
            SqlHistoryItem {
                timestamp: 0,
                year: 2020,
                day: Some(1),
                content: "".to_string(),
                properties: None,
            },
            SqlHistoryItem {
                timestamp: 0,
                year: 2020,
                day: Some(1),
                content: "".to_string(),
                properties: None,
            },
            SqlHistoryItem {
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
