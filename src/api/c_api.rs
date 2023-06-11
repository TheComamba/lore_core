use super::auxil::{
    c_write_entity_column, c_write_history_item, c_write_relationship, char_ptr, CEntityColumn,
    CEntityRelationship, CHistoryItem,
};

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
