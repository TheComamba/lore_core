use super::{
    lore_database::LoreDatabase,
    schema::history_items::{self},
    search_params::HistoryItemSearchParams,
};
use crate::errors::{sql_loading_error, LoreCoreError};
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

    pub fn get_history_items(
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
        let items = query.load::<HistoryItem>(&mut connection).map_err(|e| {
            sql_loading_error("history items", vec![("year", &year), ("day", &day)], e)
        })?;
        Ok(items)
    }
}

pub fn get_years(items: &Vec<HistoryItem>) -> Vec<i32> {
    let mut years: Vec<_> = items.iter().map(|item| item.year).collect();
    years.sort();
    years.dedup();
    years
}

pub fn get_days(items: &Vec<HistoryItem>) -> Vec<Option<i32>> {
    let mut days: Vec<_> = items.iter().map(|item| item.day).collect();
    days.sort();
    days.dedup();
    days
}

pub fn get_contents(items: &Vec<HistoryItem>) -> Vec<String> {
    let mut contents: Vec<_> = items.iter().map(|item| item.content.clone()).collect();
    contents.sort();
    contents.dedup();
    contents
}
