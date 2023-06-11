use crate::{
    errors::LoreCoreError,
    sql::{
        entity::EntityColumn, history::HistoryItem, lore_database::LoreDatabase,
        relationships::EntityRelationship,
    },
};
use std::{
    ffi::{CStr, CString},
    path::PathBuf,
};

fn char_pointer_to_string(string: *const libc::c_char) -> Result<String, LoreCoreError> {
    let string: &str = unsafe {
        CStr::from_ptr(string).to_str().map_err(|e| {
            LoreCoreError::InputError(
                "Could not convert characterpointer to string.".to_string() + &e.to_string(),
            )
        })?
    };
    Ok(string.to_string())
}

fn char_pointer_to_optional_string(
    string: *const libc::c_char,
) -> Result<Option<String>, LoreCoreError> {
    let string = char_pointer_to_string(string)?;
    Ok(if string.is_empty() {
        None
    } else {
        Some(string)
    })
}

pub(super) fn char_ptr(message: &str) -> *const libc::c_char {
    CString::new(message).unwrap().into_raw()
}

#[repr(C)]
pub struct CEntityColumn {
    pub label: *const libc::c_char,
    pub descriptor: *const libc::c_char,
    pub description: *const libc::c_char,
}

#[repr(C)]
pub struct CEntityRelationship {
    pub parent: *const libc::c_char,
    pub child: *const libc::c_char,
    pub role: *const libc::c_char,
}

#[repr(C)]
pub struct CHistoryItem {
    pub label: *const libc::c_char,
    pub content: *const libc::c_char,
    pub is_concerns_others: bool,
    pub is_secret: bool,
    pub year: i32,
    pub day: i32,
    pub originator: *const libc::c_char,
    pub year_format: *const libc::c_char,
}

fn to_entity_column(column: &CEntityColumn) -> Result<EntityColumn, LoreCoreError> {
    Ok(EntityColumn {
        label: char_pointer_to_string(column.label)?,
        descriptor: char_pointer_to_string(column.descriptor)?,
        description: char_pointer_to_string(column.description)?,
    })
}

pub(super) fn c_write_entity_column(
    db_path: *const libc::c_char,
    column: &CEntityColumn,
) -> Result<(), LoreCoreError> {
    let db_path = char_pointer_to_string(db_path)?;
    let db_path = PathBuf::from(db_path);
    let column = to_entity_column(column)?;
    let db = LoreDatabase::open(db_path)?;
    db.write_entity_column(column)?;
    Ok(())
}

fn to_history_item(item: &CHistoryItem) -> Result<HistoryItem, LoreCoreError> {
    Ok(HistoryItem {
        label: char_pointer_to_string(item.label)?,
        content: char_pointer_to_string(item.content)?,
        is_concerns_others: item.is_concerns_others,
        is_secret: item.is_secret,
        year: item.year,
        day: if item.day > 0 { Some(item.day) } else { None },
        originator: char_pointer_to_optional_string(item.originator)?,
        year_format: char_pointer_to_optional_string(item.year_format)?,
    })
}

pub(super) fn c_write_history_item(
    db_path: *const libc::c_char,
    item: &CHistoryItem,
) -> Result<(), LoreCoreError> {
    let db_path = char_pointer_to_string(db_path)?;
    let db_path = PathBuf::from(db_path);
    let item = to_history_item(item)?;
    let db = LoreDatabase::open(db_path)?;
    db.write_history_item(item)?;
    Ok(())
}

fn to_relationship(rel: &CEntityRelationship) -> Result<EntityRelationship, LoreCoreError> {
    Ok(EntityRelationship {
        parent: char_pointer_to_string(rel.parent)?,
        child: char_pointer_to_string(rel.child)?,
        role: char_pointer_to_optional_string(rel.role)?,
    })
}

pub(super) fn c_write_relationship(
    db_path: *const libc::c_char,
    rel: &CEntityRelationship,
) -> Result<(), LoreCoreError> {
    let db_path = char_pointer_to_string(db_path)?;
    let db_path = PathBuf::from(db_path);
    let relationship = to_relationship(rel)?;
    let db = LoreDatabase::open(db_path)?;
    db.write_relationship(relationship)?;
    Ok(())
}
