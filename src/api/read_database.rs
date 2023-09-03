use super::{
    auxil::{char_pointer_to_string, optional_string_to_char_pointer, string_to_char_pointer},
    types::{CEntityColumn, CEntityRelationship, CHistoryItem},
};
use crate::{errors::LoreCoreError, sql::lore_database::LoreDatabase};

pub(super) fn c_read_entity_columns(
    db_path: *const libc::c_char,
) -> Result<Vec<CEntityColumn>, LoreCoreError> {
    let db_path = char_pointer_to_string(db_path)?;
    let db = LoreDatabase::open(db_path.into())?;
    let mut columns = Vec::new();
    let database_entity_columns = db.get_all_entity_columns()?;
    for col in database_entity_columns {
        columns.push(CEntityColumn {
            label: string_to_char_pointer(&col.label),
            descriptor: string_to_char_pointer(&col.descriptor),
            description: optional_string_to_char_pointer(&col.description),
        });
    }
    Ok(columns)
}

pub(super) fn c_read_history_items(
    db_path: *const libc::c_char,
) -> Result<Vec<CHistoryItem>, LoreCoreError> {
    let db_path = char_pointer_to_string(db_path)?;
    let db = LoreDatabase::open(db_path.into())?;
    let mut items = Vec::new();
    let history_columns = db.get_all_history_items()?;
    for col in history_columns {
        items.push(CHistoryItem {
            label: string_to_char_pointer(&col.label),
            year: col.year,
            day: if let Some(day) = col.day { day } else { 0 },
            content: string_to_char_pointer(&col.content),
            properties: optional_string_to_char_pointer(&col.properties),
        });
    }
    Ok(items)
}

pub(super) fn c_read_relationships(
    db_path: *const libc::c_char,
) -> Result<Vec<CEntityRelationship>, LoreCoreError> {
    let db_path = char_pointer_to_string(db_path)?;
    let db = LoreDatabase::open(db_path.into())?;
    let mut relationships = Vec::new();
    let relationship_columns = db.get_relationships()?;
    for col in relationship_columns {
        relationships.push(CEntityRelationship {
            parent: string_to_char_pointer(&col.parent),
            child: string_to_char_pointer(&col.child),
            role: optional_string_to_char_pointer(&col.role),
        });
    }
    Ok(relationships)
}
