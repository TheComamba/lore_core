use super::{lore_database::LoreDatabase, search_text::SqlSearchText};
use crate::{
    errors::{sql_loading_error, sql_loading_error_no_params, LoreCoreError},
    sql::schema::entities,
};
use ::diesel::prelude::*;
use diesel::{Insertable, RunQueryDsl};

#[derive(Insertable, Queryable, PartialEq, Clone, Debug)]
#[diesel(table_name = entities)]
#[repr(C)]
pub struct EntityColumn {
    pub label: String,
    pub descriptor: String,
    pub description: Option<String>,
}

impl LoreDatabase {
    pub fn write_entity_columns(&self, cols: Vec<EntityColumn>) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        for col in cols.into_iter() {
            diesel::insert_into(entities::table)
                .values(&col)
                .execute(&mut connection)
                .map_err(|e| {
                    LoreCoreError::SqlError(
                        "Writing column to database failed: ".to_string() + &e.to_string(),
                    )
                })?;
        }
        Ok(())
    }

    pub fn get_all_entity_columns(&self) -> Result<Vec<EntityColumn>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let cols = entities::table
            .load::<EntityColumn>(&mut connection)
            .map_err(|e| sql_loading_error_no_params("entities", "all", e))?;
        Ok(cols)
    }

    pub fn get_entity_labels(
        &self,
        search_text: SqlSearchText,
    ) -> Result<Vec<String>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let mut query = entities::table.into_boxed();
        if search_text.is_some() {
            query = query.filter(entities::label.like(search_text.to_string()));
        }
        let labels = query
            .load::<EntityColumn>(&mut connection)
            .map_err(|e| {
                sql_loading_error(
                    "entities",
                    "labels",
                    vec![("search_text", &Some(search_text))],
                    e,
                )
            })?
            .into_iter()
            .map(|c| c.label)
            .collect();
        Ok(labels)
    }

    pub fn get_descriptors(
        &self,
        label: &Option<&String>,
        search_text: SqlSearchText,
    ) -> Result<Vec<String>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let mut query = entities::table.into_boxed();
        if let Some(label) = label {
            query = query.filter(entities::label.eq(label));
        }
        if search_text.is_some() {
            query = query.filter(entities::descriptor.like(search_text.to_string()));
        }
        let descriptors = query
            .load::<EntityColumn>(&mut connection)
            .map_err(|e| {
                sql_loading_error(
                    "entities",
                    "descriptors",
                    vec![
                        ("label", label),
                        ("search_text", &Some(&search_text.to_string())),
                    ],
                    e,
                )
            })?
            .into_iter()
            .map(|c| c.descriptor)
            .collect();
        Ok(descriptors)
    }

    pub fn get_description(
        &self,
        label: &String,
        descriptor: &String,
    ) -> Result<Option<String>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let descriptions = entities::table
            .filter(entities::label.eq(label))
            .filter(entities::descriptor.eq(descriptor))
            .load::<EntityColumn>(&mut connection)
            .map_err(|e| {
                sql_loading_error(
                    "entities",
                    "description",
                    vec![("label", &Some(label)), ("descriptor", &Some(descriptor))],
                    e,
                )
            })?;
        if descriptions.len() > 1 {
            Err(LoreCoreError::SqlError(
                "More than one description found for label '".to_string()
                    + label
                    + "' and descriptor '"
                    + descriptor
                    + "'.",
            ))
        } else {
            let description = match descriptions.first() {
                Some(col) => col.description.to_owned(),
                None => {
                    return Err(LoreCoreError::SqlError(
                        "No description found for label '".to_string()
                            + label
                            + "' and descriptor '"
                            + descriptor
                            + "'.",
                    ))
                }
            };
            Ok(description)
        }
    }
}
