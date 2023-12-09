use super::{
    lore_database::LoreDatabase,
    schema::history_items::{self},
};
use crate::errors::{sql_loading_error, sql_loading_error_no_params, LoreCoreError};
use ::diesel::prelude::*;
use diesel::Insertable;

#[derive(Insertable, Queryable, PartialEq, Clone, Debug)]
#[diesel(table_name = history_items)]
#[repr(C)]
pub struct HistoryItem {
    pub timestamp: i64,
    pub year: i32,
    pub day: Option<i32>,
    pub content: String,
    pub properties: Option<String>,
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

    pub fn get_all_history_items(&self) -> Result<Vec<HistoryItem>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let items = history_items::table
            .load::<HistoryItem>(&mut connection)
            .map_err(|e| sql_loading_error_no_params("history items", "all years", e))?;
        Ok(items)
    }

    pub fn get_all_years(&self) -> Result<Vec<i32>, LoreCoreError> {
        let items = self.get_all_history_items()?;
        let mut years: Vec<_> = items.into_iter().map(|item| item.year).collect();
        years.dedup();
        Ok(years)
    }

    pub fn get_days(&self, year: i32) -> Result<Vec<Option<i32>>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let mut query = history_items::table.into_boxed();
        query = query.filter(history_items::year.eq(year));
        let mut days: Vec<_> = query
            .load::<HistoryItem>(&mut connection)
            .map_err(|e| sql_loading_error("history items", "days", vec![("year", &year)], e))?
            .into_iter()
            .map(|item| item.day)
            .collect();
        days.dedup();
        Ok(days)
    }

    pub fn get_history_item_content(&self, timestamp: i64) -> Result<String, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let items = history_items::table
            .filter(history_items::timestamp.eq(timestamp))
            .load::<HistoryItem>(&mut connection)
            .map_err(|e| {
                sql_loading_error(
                    "history item",
                    "content",
                    vec![("timestamp", &timestamp)],
                    e,
                )
            })?;
        if items.len() > 1 {
            Err(LoreCoreError::SqlError(format!(
                "More than one entry found for timestamp {}",
                timestamp
            )))
        } else {
            let content = match items.first() {
                Some(item) => item.content.to_owned(),
                None => {
                    return Err(LoreCoreError::SqlError(format!(
                        "No content found for timestamp {}",
                        timestamp
                    )))
                }
            };
            Ok(content)
        }
    }
}
