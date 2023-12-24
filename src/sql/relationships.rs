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

pub fn extract_parents(rels: &Vec<EntityRelationship>) -> Vec<String> {
    let mut parents: Vec<_> = rels.iter().map(|rel| rel.parent.clone()).collect();
    parents.sort();
    parents.dedup();
    parents
}

pub fn extract_children(rels: &Vec<EntityRelationship>) -> Vec<String> {
    let mut children: Vec<_> = rels.iter().map(|rel| rel.child.clone()).collect();
    children.sort();
    children.dedup();
    children
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
}
