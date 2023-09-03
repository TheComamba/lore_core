use super::{lore_database::LoreDatabase, schema::relationships};
use crate::errors::{sql_loading_error, sql_loading_error_no_params, LoreCoreError};
use ::diesel::prelude::*;
use diesel::Insertable;
use diesel::{QueryDsl, Queryable, RunQueryDsl};

#[derive(Insertable, Queryable, PartialEq, Clone, Debug)]
#[diesel(table_name = relationships)]
#[repr(C)]
pub struct EntityRelationship {
    pub parent: String,
    pub child: String,
    pub role: Option<String>,
}

impl LoreDatabase {
    pub fn write_relationships(&self, rels: Vec<EntityRelationship>) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        for rel in rels.into_iter() {
            diesel::insert_into(relationships::table)
                .values(&rel)
                .execute(&mut connection)
                .map_err(|e| {
                    LoreCoreError::SqlError(
                        "Writing relationship to database failed: ".to_string() + &e.to_string(),
                    )
                })?;
        }
        Ok(())
    }

    pub fn get_relationships(&self) -> Result<Vec<EntityRelationship>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let rels = relationships::table
            .load::<EntityRelationship>(&mut connection)
            .map_err(|e| sql_loading_error_no_params("relationships", "all", e))?;
        Ok(rels)
    }

    pub fn get_parents(&self, child: &Option<&String>) -> Result<Vec<String>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let mut query = relationships::table.into_boxed();
        if let Some(child) = child {
            query = query.filter(relationships::child.eq(child));
        }
        let parents = query
            .load::<EntityRelationship>(&mut connection)
            .map_err(|e| sql_loading_error("relationships", "parents", vec![("child", child)], e))?
            .into_iter()
            .map(|r| r.parent)
            .collect::<Vec<_>>();
        Ok(parents)
    }

    pub fn get_children(&self, parent: &Option<&String>) -> Result<Vec<String>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let mut query = relationships::table.into_boxed();
        if let Some(parent) = parent {
            query = query.filter(relationships::parent.eq(parent))
        }
        let children = query
            .load::<EntityRelationship>(&mut connection)
            .map_err(|e| {
                sql_loading_error("relationships", "children", vec![("parent", parent)], e)
            })?
            .into_iter()
            .map(|r| r.child)
            .collect::<Vec<_>>();
        Ok(children)
    }

    pub fn get_relationship_role(
        &self,
        parent: &String,
        child: &String,
    ) -> Result<Option<String>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let relationships = relationships::table
            .filter(relationships::parent.eq(parent))
            .filter(relationships::child.eq(child))
            .load::<EntityRelationship>(&mut connection)
            .map_err(|e| {
                sql_loading_error(
                    "relationship",
                    "role",
                    vec![("parent", &Some(parent)), ("child", &Some(child))],
                    e,
                )
            })?;
        if relationships.len() > 1 {
            Err(LoreCoreError::SqlError(
                "More than one entry found for parent '".to_string()
                    + parent
                    + "' and child '"
                    + child
                    + "'.",
            ))
        } else {
            let role = match relationships.first() {
                Some(relationship) => relationship.role.to_owned(),
                None => {
                    return Err(LoreCoreError::SqlError(
                        "No content found for parent '".to_string()
                            + parent
                            + "' and child '"
                            + child
                            + "'.",
                    ))
                }
            };
            Ok(role)
        }
    }
}
