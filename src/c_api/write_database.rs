use super::{auxil::char_pointer_to_string, c_types::*};
use crate::{errors::LoreCoreError, sql::lore_database::LoreDatabase};
use std::path::PathBuf;

pub(super) fn c_write_entity_column(
    db_path: *const libc::c_char,
    column: &CEntityColumn,
) -> Result<(), LoreCoreError> {
    let db_path = char_pointer_to_string(db_path)?;
    let db_path = PathBuf::from(db_path);
    let column = column.try_into()?;
    let db = LoreDatabase::open(db_path)?;
    db.write_entity_columns(vec![column])?;
    Ok(())
}

pub(super) fn c_write_history_item(
    db_path: *const libc::c_char,
    item: &CHistoryItem,
) -> Result<(), LoreCoreError> {
    let db_path = char_pointer_to_string(db_path)?;
    let db_path = PathBuf::from(db_path);
    let item = item.try_into()?;
    let db = LoreDatabase::open(db_path)?;
    db.write_history_items(vec![item])?;
    Ok(())
}

pub(super) fn c_write_relationship(
    db_path: *const libc::c_char,
    rel: &CEntityRelationship,
) -> Result<(), LoreCoreError> {
    let db_path = char_pointer_to_string(db_path)?;
    let db_path = PathBuf::from(db_path);
    let relationship = rel.try_into()?;
    let db = LoreDatabase::open(db_path)?;
    db.write_relationships(vec![relationship])?;
    Ok(())
}
