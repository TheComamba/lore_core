use crate::timestamp::current_timestamp;

use super::{
    auxil::char_ptr,
    c_types::*,
    write_database::{c_write_entity_column, c_write_history_item, c_write_relationship},
};

/// # Safety
///
/// `db_path` must be a valid C string.
/// `columns` must be a valid pointer to an array of `CEntityColumn`s.
/// `size` must correspond to the length of the array.
#[no_mangle]
pub unsafe extern "C" fn write_entity_columns(
    db_path: *const libc::c_char,
    columns: *const CEntityColumn,
    size: isize,
) -> *const libc::c_char {
    for i in 0..size {
        if let Err(e) = c_write_entity_column(db_path, &*columns.offset(i)) {
            return char_ptr(&e.to_string());
        }
    }
    char_ptr("")
}

/// # Safety
///
/// `db_path` must be a valid C string.
/// `size` must be a valid pointer to allocated memory of `isize`.
#[no_mangle]
pub unsafe extern "C" fn get_number_of_entity_columns(
    db_path: *const libc::c_char,
    size: *mut isize,
) -> *const libc::c_char {
    match super::read_database::c_read_entity_columns(db_path) {
        Ok(cols) => {
            *size = cols.len() as isize;
            char_ptr("")
        }
        Err(e) => char_ptr(&e.to_string()),
    }
}

/// # Safety
///
/// `db_path` must be a valid C string.
/// `columns` must be a valid pointer to an array of `CEntityColumn`s.
#[no_mangle]
pub unsafe extern "C" fn read_entity_columns(
    db_path: *const libc::c_char,
    columns: *mut CEntityColumn,
) -> *const libc::c_char {
    match super::read_database::c_read_entity_columns(db_path) {
        Ok(database_entries) => {
            for (i, _) in database_entries.iter().enumerate() {
                *columns.add(i) = database_entries[i].clone();
            }
            char_ptr("")
        }
        Err(e) => char_ptr(&e.to_string()),
    }
}

/// # Safety
///
/// `db_path` must be a valid C string.
/// `items` must be a valid pointer to an array of `CHistoryItem`s.
/// `size` must correspond to the length of the array.
#[no_mangle]
pub unsafe extern "C" fn write_history_items(
    db_path: *const libc::c_char,
    items: *const CHistoryItem,
    size: isize,
) -> *const libc::c_char {
    for i in 0..size {
        if let Err(e) = c_write_history_item(db_path, &*items.offset(i)) {
            return char_ptr(&e.to_string());
        }
    }
    char_ptr("")
}

/// # Safety
///
/// `db_path` must be a valid C string.
/// `size` must be a valid pointer to allocated memory of `isize`.
#[no_mangle]
pub unsafe extern "C" fn get_number_of_history_items(
    db_path: *const libc::c_char,
    size: *mut isize,
) -> *const libc::c_char {
    match super::read_database::c_read_history_items(db_path) {
        Ok(items) => {
            *size = items.len() as isize;
            char_ptr("")
        }
        Err(e) => char_ptr(&e.to_string()),
    }
}

/// # Safety
///
/// `db_path` must be a valid C string.
/// `items` must be a valid pointer to an array of `CHistoryItem`s.
#[no_mangle]
pub unsafe extern "C" fn read_history_items(
    db_path: *const libc::c_char,
    items: *mut CHistoryItem,
) -> *const libc::c_char {
    match super::read_database::c_read_history_items(db_path) {
        Ok(database_entries) => {
            for (i, _) in database_entries.iter().enumerate() {
                *items.add(i) = database_entries[i].clone();
            }
            char_ptr("")
        }
        Err(e) => char_ptr(&e.to_string()),
    }
}

/// # Safety
///
/// `db_path` must be a valid C string.
/// `relationships` must be a valid pointer to an array of `CEntityRelationship`s.
/// `size` must correspond to the length of the array.
#[no_mangle]
pub unsafe extern "C" fn write_relationships(
    db_path: *const libc::c_char,
    relationships: *const CEntityRelationship,
    size: isize,
) -> *const libc::c_char {
    for i in 0..size {
        if let Err(e) = c_write_relationship(db_path, &*relationships.offset(i)) {
            return char_ptr(&e.to_string());
        }
    }
    char_ptr("")
}

/// # Safety
///
/// `db_path` must be a valid C string.
/// `size` must be a valid pointer to allocated memory of `isize`.
#[no_mangle]
pub unsafe extern "C" fn get_number_of_relationships(
    db_path: *const libc::c_char,
    size: *mut isize,
) -> *const libc::c_char {
    match super::read_database::c_read_relationships(db_path) {
        Ok(relationships) => {
            *size = relationships.len() as isize;
            char_ptr("")
        }
        Err(e) => char_ptr(&e.to_string()),
    }
}

/// # Safety
///
/// `db_path` must be a valid C string.
/// `relationships` must be a valid pointer to an array of `CEntityRelationship`s.
#[no_mangle]
pub unsafe extern "C" fn read_relationships(
    db_path: *const libc::c_char,
    relationships: *mut CEntityRelationship,
) -> *const libc::c_char {
    match super::read_database::c_read_relationships(db_path) {
        Ok(database_entries) => {
            for (i, _) in database_entries.iter().enumerate() {
                *relationships.add(i) = database_entries[i].clone();
            }
            char_ptr("")
        }
        Err(e) => char_ptr(&e.to_string()),
    }
}

#[no_mangle]
pub extern "C" fn get_current_timestamp() -> i64 {
    current_timestamp().to_int()
}
