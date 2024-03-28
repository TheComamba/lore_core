use super::{lore_database::LoreDatabase, search_params::EntityColumnSearchParams};
use crate::{
    errors::{sql_loading_error, LoreCoreError},
    sql::schema::entities,
};
use ::diesel::prelude::*;
use diesel::{Insertable, RunQueryDsl};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Insertable, Queryable)]
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

    pub fn relabel_entity(&self, old_label: &str, new_label: &str) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::update(entities::table.filter(entities::label.eq(old_label)))
            .set(entities::label.eq(new_label))
            .execute(&mut connection)
            .map_err(|e| {
                LoreCoreError::SqlError(
                    "Relabeling entity in database failed: ".to_string() + &e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn delete_entity(&self, label: String) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::delete(entities::table.filter(entities::label.eq(label)))
            .execute(&mut connection)
            .map_err(|e| {
                LoreCoreError::SqlError(
                    "Deleting entity from database failed: ".to_string() + &e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn change_entity_descriptor(
        &self,
        (label, old_descriptor): (&str, String),
        new_descriptor: &str,
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::update(entities::table.filter(entities::label.eq(label))
            .filter(entities::descriptor.eq(old_descriptor)))
            .set(entities::descriptor.eq(new_descriptor))
            .execute(&mut connection)
            .map_err(|e| {
                LoreCoreError::SqlError(
                    "Changing entity descriptor in database failed: ".to_string() + &e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn delete_entity_column(
        &self,
        (label, descriptor): (String, String),
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::delete(entities::table.filter(entities::label.eq(label))
            .filter(entities::descriptor.eq(descriptor)))
            .execute(&mut connection)
            .map_err(|e| {
                LoreCoreError::SqlError(
                    "Deleting entity column from database failed: ".to_string() + &e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn change_entity_description(
        &self,
        (label, descriptor): (&str, &str),
        new_description: &Option<String>,
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::update(entities::table.filter(entities::label.eq(label))
            .filter(entities::descriptor.eq(descriptor)))
            .set(entities::description.eq(new_description))
            .execute(&mut connection)
            .map_err(|e| {
                LoreCoreError::SqlError(
                    "Changing entity description in database failed: ".to_string() + &e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn read_entity_columns(
        &self,
        search_params: EntityColumnSearchParams,
    ) -> Result<Vec<EntityColumn>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let mut query = entities::table.into_boxed();
        let label = search_params.label;
        if label.is_some() {
            if label.is_exact {
                query = query.filter(entities::label.eq(label.exact_text()));
            } else {
                query = query.filter(entities::label.like(label.search_pattern()));
            }
        }
        let descriptor = search_params.descriptor;
        if descriptor.is_some() {
            if descriptor.is_exact {
                query = query.filter(entities::descriptor.eq(descriptor.exact_text()));
            } else {
                query = query.filter(entities::descriptor.like(descriptor.search_pattern()));
            }
        }
        let mut cols = query.load::<EntityColumn>(&mut connection).map_err(|e| {
            sql_loading_error(
                "entities",
                vec![("label", &label), ("descriptor", &descriptor)],
                e,
            )
        })?;
        cols.sort();
        Ok(cols)
    }
}

pub fn extract_labels(cols: &[EntityColumn]) -> Vec<String> {
    let mut labels: Vec<_> = cols.iter().map(|c| c.label.clone()).collect();
    labels.sort();
    labels.dedup();
    labels
}

pub fn extract_descriptors(cols: &[EntityColumn]) -> Vec<String> {
    let mut descriptors: Vec<_> = cols.iter().map(|c| c.descriptor.clone()).collect();
    descriptors.sort();
    descriptors.dedup();
    descriptors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_labels() {
        let cols = vec![
            EntityColumn {
                label: "qux".to_string(),
                descriptor: "bar".to_string(),
                description: None,
            },
            EntityColumn {
                label: "foo".to_string(),
                descriptor: "bar".to_string(),
                description: None,
            },
            EntityColumn {
                label: "foo".to_string(),
                descriptor: "baz".to_string(),
                description: None,
            },
        ];
        let labels = extract_labels(&cols);
        assert_eq!(labels, vec!["foo".to_string(), "qux".to_string()]);
    }

    #[test]
    fn test_extract_descriptors() {
        let cols = vec![
            EntityColumn {
                label: "foo".to_string(),
                descriptor: "bar".to_string(),
                description: None,
            },
            EntityColumn {
                label: "foo".to_string(),
                descriptor: "baz".to_string(),
                description: None,
            },
            EntityColumn {
                label: "qux".to_string(),
                descriptor: "bar".to_string(),
                description: None,
            },
        ];
        let descriptors = extract_descriptors(&cols);
        assert_eq!(descriptors, vec!["bar".to_string(), "baz".to_string()]);
    }
}
