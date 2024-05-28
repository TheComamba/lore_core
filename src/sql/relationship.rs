use ::diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

use crate::errors::{sql_loading_error, LoreCoreError};
use crate::types::relationship::EntityRelationship;

use super::search_params::RelationshipSearchParams;
use super::types::relationship::{role_to_sql, SqlEntityRelationship};
use super::{lore_database::LoreDatabase, schema::relationships};

impl LoreDatabase {
    pub fn write_relationships(&self, rels: Vec<EntityRelationship>) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        for rel in rels.into_iter() {
            let rel = rel.to_sql_entity_relationship();
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

    pub fn change_relationship_role(
        &self,
        old_relationship: EntityRelationship,
        new_role: &Option<String>,
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::update(
            relationships::table.filter(
                relationships::parent
                    .eq(old_relationship.parent)
                    .and(relationships::child.eq(old_relationship.child))
                    .and(relationships::role.eq(role_to_sql(&old_relationship.role))),
            ),
        )
        .set(relationships::role.eq(role_to_sql(new_role)))
        .execute(&mut connection)
        .map_err(|e| {
            LoreCoreError::SqlError(
                "Changing relationship role in database failed: ".to_string() + &e.to_string(),
            )
        })?;
        Ok(())
    }

    pub fn delete_relationship(
        &self,
        relationship: EntityRelationship,
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::delete(
            relationships::table.filter(
                relationships::parent
                    .eq(relationship.parent)
                    .and(relationships::child.eq(relationship.child))
                    .and(relationships::role.eq(role_to_sql(&relationship.role))),
            ),
        )
        .execute(&mut connection)
        .map_err(|e| {
            LoreCoreError::SqlError(
                "Deleting relationship from database failed: ".to_string() + &e.to_string(),
            )
        })?;
        Ok(())
    }

    pub fn read_relationships(
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
        let rels = query
            .load::<SqlEntityRelationship>(&mut connection)
            .map_err(|e| {
                sql_loading_error(
                    "relationships",
                    vec![("parent", &parent), ("child", &child)],
                    e,
                )
            })?;
        let mut rels: Vec<EntityRelationship> =
            rels.into_iter().map(|rel| rel.to_relationship()).collect();
        rels.sort();
        Ok(rels)
    }
}

pub fn extract_parents(rels: &[EntityRelationship]) -> Vec<String> {
    let mut parents: Vec<_> = rels.iter().map(|rel| rel.parent.clone()).collect();
    parents.sort();
    parents.dedup();
    parents
}

pub fn extract_children(rels: &[EntityRelationship]) -> Vec<String> {
    let mut children: Vec<_> = rels.iter().map(|rel| rel.child.clone()).collect();
    children.sort();
    children.dedup();
    children
}

pub fn extract_roles(rels: &[EntityRelationship]) -> Vec<String> {
    let mut roles: Vec<_> = rels.iter().filter_map(|rel| rel.role.clone()).collect();
    roles.sort();
    roles.dedup();
    roles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_parents() {
        let rels = vec![
            EntityRelationship {
                parent: "b".to_string(),
                child: "c".to_string(),
                role: None,
            },
            EntityRelationship {
                parent: "a".to_string(),
                child: "b".to_string(),
                role: None,
            },
            EntityRelationship {
                parent: "a".to_string(),
                child: "c".to_string(),
                role: None,
            },
        ];
        let parents = extract_parents(&rels);
        assert!(parents == vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_extract_children() {
        let rels = vec![
            EntityRelationship {
                parent: "b".to_string(),
                child: "c".to_string(),
                role: None,
            },
            EntityRelationship {
                parent: "a".to_string(),
                child: "b".to_string(),
                role: None,
            },
            EntityRelationship {
                parent: "a".to_string(),
                child: "c".to_string(),
                role: None,
            },
        ];
        let children = extract_children(&rels);
        assert!(children == vec!["b".to_string(), "c".to_string()]);
    }

    #[test]
    fn test_extract_roles() {
        let rels = vec![
            EntityRelationship {
                parent: "b".to_string(),
                child: "c".to_string(),
                role: Some("r1".to_string()),
            },
            EntityRelationship {
                parent: "a".to_string(),
                child: "b".to_string(),
                role: Some("r2".to_string()),
            },
            EntityRelationship {
                parent: "a".to_string(),
                child: "c".to_string(),
                role: Some("r1".to_string()),
            },
        ];
        let roles = extract_roles(&rels);
        assert!(roles == vec!["r1".to_string(), "r2".to_string()]);
    }
}
