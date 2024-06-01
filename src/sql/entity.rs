use ::diesel::prelude::*;
use diesel::RunQueryDsl;

use crate::{
    errors::{sql_loading_error, LoreCoreError},
    sql::schema::entities,
    types::{description::Description, descriptor::Descriptor, entity::EntityColumn, label::Label},
};

use super::{
    lore_database::LoreDatabase, search_params::EntityColumnSearchParams,
    types::entity::SqlEntityColumn,
};

impl LoreDatabase {
    pub fn write_entity_columns(&self, cols: Vec<EntityColumn>) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        for col in cols.into_iter() {
            let col = col.to_sql_entity_column();
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

    pub fn relabel_entity(
        &self,
        old_label: &Label,
        new_label: &Label,
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::update(entities::table.filter(entities::label.eq(old_label.to_str())))
            .set(entities::label.eq(new_label.to_str()))
            .execute(&mut connection)
            .map_err(|e| {
                LoreCoreError::SqlError(
                    "Relabeling entity in database failed: ".to_string() + &e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn delete_entity(&self, label: Label) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::delete(entities::table.filter(entities::label.eq(label.to_str())))
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
        (label, old_descriptor): (&Label, Descriptor),
        new_descriptor: &Descriptor,
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::update(
            entities::table
                .filter(entities::label.eq(label.to_str()))
                .filter(entities::descriptor.eq(old_descriptor.to_str())),
        )
        .set(entities::descriptor.eq(new_descriptor.to_str()))
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
        (label, descriptor): (Label, Descriptor),
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::delete(
            entities::table
                .filter(entities::label.eq(label.to_str()))
                .filter(entities::descriptor.eq(descriptor.to_str())),
        )
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
        (label, descriptor): (&Label, &Descriptor),
        new_description: &Description,
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::update(
            entities::table
                .filter(entities::label.eq(label.to_str()))
                .filter(entities::descriptor.eq(descriptor.to_str())),
        )
        .set(entities::description.eq(new_description.to_str()))
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
        let mut cols: Vec<_> = query
            .load::<SqlEntityColumn>(&mut connection)
            .map_err(|e| {
                sql_loading_error(
                    "entities",
                    vec![("label", &label), ("descriptor", &descriptor)],
                    e,
                )
            })?
            .into_iter()
            .map(|c| c.to_entity_column())
            .collect();
        cols.sort();
        Ok(cols)
    }
}
