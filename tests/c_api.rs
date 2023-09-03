use lorecore::api::{
    auxil::{char_pointer_to_string, string_to_char_pointer},
    c_api::{write_entity_columns, write_history_items, write_relationships},
    types::{CEntityColumn, CEntityRelationship, CHistoryItem},
};
use tempfile::NamedTempFile;

#[test]
fn write_entity_column() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let db_path = temp_path.as_os_str();
    let db_path_ptr = string_to_char_pointer(&db_path.to_str().unwrap());
    let column = CEntityColumn {
        label: string_to_char_pointer("testlabel"),
        descriptor: string_to_char_pointer("testdescriptor"),
        description: string_to_char_pointer("testdescription"),
    };
    let column_ptr = &column as *const CEntityColumn;

    unsafe {
        let result = write_entity_columns(db_path_ptr, column_ptr, 1);
        assert_eq!(char_pointer_to_string(result).unwrap(), "");
    }

    temp_path.close().unwrap();
}

#[test]
fn write_history_item() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let db_path = temp_path.as_os_str();
    let db_path_ptr = string_to_char_pointer(&db_path.to_str().unwrap());
    let item = CHistoryItem {
        label: string_to_char_pointer("_2020-14"),
        year: 2020,
        day: 14,
        content: string_to_char_pointer("testcontent"),
        properties: string_to_char_pointer("{\"is_secret\": true}"),
    };
    let item_ptr = &item as *const CHistoryItem;

    unsafe {
        let result = write_history_items(db_path_ptr, item_ptr, 1);
        assert_eq!(char_pointer_to_string(result).unwrap(), "");
    }

    temp_path.close().unwrap();
}

#[test]
fn write_relationship() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let db_path = temp_path.as_os_str();
    let db_path_ptr = string_to_char_pointer(&db_path.to_str().unwrap());
    let relationship = CEntityRelationship {
        parent: string_to_char_pointer("testparent"),
        child: string_to_char_pointer("testchild"),
        role: string_to_char_pointer("testrole"),
    };
    let relationship_ptr = &relationship as *const CEntityRelationship;

    unsafe {
        let result = write_relationships(db_path_ptr, relationship_ptr, 1);
        assert_eq!(char_pointer_to_string(result).unwrap(), "");
    }

    temp_path.close().unwrap();
}
