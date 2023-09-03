use lorecore::api::{
    auxil::char_pointer_to_string,
    c_api::{write_entity_columns, write_history_items, write_relationships},
    types::{CEntityColumn, CEntityRelationship, CHistoryItem},
};
use tempfile::NamedTempFile;

#[test]
fn write_entity_column() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let db_path = temp_path.as_os_str();
    let db_path_ptr = db_path.to_str().unwrap().as_ptr() as *const libc::c_char;
    let column = CEntityColumn {
        label: "testlabel".as_ptr() as *const libc::c_char,
        descriptor: "testdescriptor".as_ptr() as *const libc::c_char,
        description: "testdescription".as_ptr() as *const libc::c_char,
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
    let db_path_ptr = db_path.to_str().unwrap().as_ptr() as *const libc::c_char;
    let item = CHistoryItem {
        label: "_2020-14".as_ptr() as *const libc::c_char,
        year: 2020,
        day: 14,
        content: "testcontent".as_ptr() as *const libc::c_char,
        properties: "{\"is_secret\": true}".as_ptr() as *const libc::c_char,
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
    let db_path_ptr = db_path.to_str().unwrap().as_ptr() as *const libc::c_char;
    let relationship = CEntityRelationship {
        parent: "testparent".as_ptr() as *const libc::c_char,
        child: "testchild".as_ptr() as *const libc::c_char,
        role: "testrole".as_ptr() as *const libc::c_char,
    };
    let relationship_ptr = &relationship as *const CEntityRelationship;

    unsafe {
        let result = write_relationships(db_path_ptr, relationship_ptr, 1);
        assert_eq!(char_pointer_to_string(result).unwrap(), "");
    }

    temp_path.close().unwrap();
}
