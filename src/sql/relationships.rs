use super::search_params::RelationshipSearchParams;
use super::{lore_database::LoreDatabase, schema::relationships};
use crate::errors::{sql_loading_error, LoreCoreError};
use ::diesel::prelude::*;
use diesel::Insertable;
use diesel::{QueryDsl, Queryable, RunQueryDsl};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Insertable, Queryable)]
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

    pub fn get_relationships(
        &self,
        search_params: RelationshipSearchParams,
    ) -> Result<Vec<EntityRelationship>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let mut query = relationships::table.into_boxed();
        let parent = search_params.parent;
        if parent.is_some() {
            if parent.is_exact {
                query = query.filter(relationships::parent.eq(parent.exact_text()));
            } else {
                query = query.filter(relationships::parent.like(parent.search_pattern()));
            }
        }
        let child = search_params.child;
        if child.is_some() {
            if child.is_exact {
                query = query.filter(relationships::child.eq(child.exact_text()));
            } else {
                query = query.filter(relationships::child.like(child.search_pattern()));
            }
        }
        let mut rels = query
            .load::<EntityRelationship>(&mut connection)
            .map_err(|e| {
                sql_loading_error(
                    "relationships",
                    vec![("parent", &parent), ("child", &child)],
                    e,
                )
            })?;
        rels.sort();
        Ok(rels)
    }
}

pub fn get_parents(rels: &Vec<EntityRelationship>) -> Vec<String> {
    let mut parents: Vec<_> = rels.iter().map(|rel| rel.parent.clone()).collect();
    parents.sort();
    parents.dedup();
    parents
}

pub fn get_children(rels: &Vec<EntityRelationship>) -> Vec<String> {
    let mut children: Vec<_> = rels.iter().map(|rel| rel.child.clone()).collect();
    children.sort();
    children.dedup();
    children
}

pub fn get_roles(rels: &Vec<EntityRelationship>) -> Vec<Option<String>> {
    let mut roles: Vec<_> = rels.iter().map(|rel| rel.role.clone()).collect();
    roles.sort();
    roles.dedup();
    roles
}
