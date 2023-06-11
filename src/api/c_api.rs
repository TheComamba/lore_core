use super::auxil::{
    c_write_entity_column, c_write_history_item, c_write_relationship, char_ptr, CEntityColumn,
    CEntityRelationship, CHistoryItem,
};

#[no_mangle]
pub unsafe extern "C" fn write_entity_column(
    db_path: *const libc::c_char,
    column: *const CEntityColumn,
) -> *const libc::c_char {
    match c_write_entity_column(db_path, &*column) {
        Ok(()) => char_ptr(""),
        Err(e) => char_ptr(&e.to_string()),
    }
}

#[no_mangle]
pub unsafe extern "C" fn write_history_item(
    db_path: *const libc::c_char,
    item: *const CHistoryItem,
) -> *const libc::c_char {
    match c_write_history_item(db_path, &*item) {
        Ok(()) => char_ptr(""),
        Err(e) => char_ptr(&e.to_string()),
    }
}

#[no_mangle]
pub unsafe extern "C" fn write_relationship(
    db_path: *const libc::c_char,
    relationship: *const CEntityRelationship,
) -> *const libc::c_char {
    match c_write_relationship(db_path, &*relationship) {
        Ok(()) => char_ptr(""),
        Err(e) => char_ptr(&e.to_string()),
    }
}
