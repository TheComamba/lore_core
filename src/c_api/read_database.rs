use super::{
    auxil::char_pointer_to_string,
    types::{entity::CEntityColumn, history::CHistoryItem, relationship::CEntityRelationship},
};
use crate::{
    errors::LoreCoreError,
    sql::{
        lore_database::LoreDatabase,
        search_params::{
            EntityColumnSearchParams, HistoryItemSearchParams, RelationshipSearchParams,
        },
    },
};

pub(super) unsafe fn c_read_entity_columns(
    db_path: *const libc::c_char,
) -> Result<Vec<CEntityColumn>, LoreCoreError> {
    let db_path = char_pointer_to_string(db_path)?;
    let db = LoreDatabase::open(db_path.into())?;
    let mut columns = Vec::new();
    let database_entity_columns = db.read_entity_columns(EntityColumnSearchParams::empty())?;
    for col in database_entity_columns {
        columns.push(col.try_into()?);
    }
    Ok(columns)
}

pub(super) unsafe fn c_read_history_items(
    db_path: *const libc::c_char,
) -> Result<Vec<CHistoryItem>, LoreCoreError> {
    let db_path = char_pointer_to_string(db_path)?;
    let db = LoreDatabase::open(db_path.into())?;
    let mut items = Vec::new();
    let history_columns = db.read_history_items(HistoryItemSearchParams::empty())?;
    for col in history_columns {
        items.push(col.try_into()?);
    }
    Ok(items)
}

pub(super) unsafe fn c_read_relationships(
    db_path: *const libc::c_char,
) -> Result<Vec<CEntityRelationship>, LoreCoreError> {
    let db_path = char_pointer_to_string(db_path)?;
    let db = LoreDatabase::open(db_path.into())?;
    let mut relationships = Vec::new();
    let relationship_columns = db.read_relationships(RelationshipSearchParams::empty())?;
    for col in relationship_columns {
        relationships.push(col.try_into()?);
    }
    Ok(relationships)
}
