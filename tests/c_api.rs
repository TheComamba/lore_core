#[cfg(test)]
mod tests {
    use lorecore::c_api::{
        auxil::{char_pointer_to_string, string_to_char_pointer},
        c_api::*,
        types::{CEntityColumn, CEntityRelationship, CHistoryItem},
    };
    use tempfile::NamedTempFile;

    #[test]
    fn write_entity_column() {
        let temp_path = NamedTempFile::new().unwrap().into_temp_path();
        let db_path = temp_path.as_os_str();
        let db_path_ptr = string_to_char_pointer(&db_path.to_str().unwrap());
        let column1 = CEntityColumn {
            label: string_to_char_pointer("testlabel1"),
            descriptor: string_to_char_pointer("testdescriptor1"),
            description: string_to_char_pointer("testdescription1"),
        };
        let column2 = CEntityColumn {
            label: string_to_char_pointer("testlabel2"),
            descriptor: string_to_char_pointer("testdescriptor2"),
            description: string_to_char_pointer("testdescription2"),
        };
        let columns = [column1, column2];
        let columns_ptr = columns.as_ptr();

        unsafe {
            let result = write_entity_columns(db_path_ptr, columns_ptr, columns.len() as isize);
            assert_eq!(char_pointer_to_string(result).unwrap(), "");

            let mut size: isize = 0;
            let size_ptr: *mut isize = &mut size;
            let result = get_number_of_entity_columns(db_path_ptr, size_ptr);
            assert_eq!(char_pointer_to_string(result).unwrap(), "");
            assert_eq!(size, columns.len() as isize);

            let mut read_columns: Vec<CEntityColumn> = Vec::with_capacity(size as usize);
            let read_columns_ptr = read_columns.as_mut_ptr();
            let result = read_entity_columns(db_path_ptr, read_columns_ptr);
            assert_eq!(char_pointer_to_string(result).unwrap(), "");
            read_columns.set_len(size as usize);
        }

        temp_path.close().unwrap();
    }

    #[test]
    fn write_history_item() {
        let temp_path = NamedTempFile::new().unwrap().into_temp_path();
        let db_path = temp_path.as_os_str();
        let db_path_ptr = string_to_char_pointer(&db_path.to_str().unwrap());
        let item1 = CHistoryItem {
            timestamp: get_current_timestamp(),
            year: 2020,
            day: 14,
            content: string_to_char_pointer("testcontent1"),
            properties: string_to_char_pointer("{\"is_secret\": true}"),
        };
        let item2 = CHistoryItem {
            timestamp: get_current_timestamp(),
            year: 2021,
            day: 15,
            content: string_to_char_pointer("testcontent2"),
            properties: string_to_char_pointer("{\"is_secret\": false}"),
        };
        let items = [item1, item2];
        let items_ptr = items.as_ptr();

        unsafe {
            let result = write_history_items(db_path_ptr, items_ptr, items.len() as isize);
            assert_eq!(char_pointer_to_string(result).unwrap(), "");

            let mut size: isize = 0;
            let size_ptr: *mut isize = &mut size;
            let result = get_number_of_history_items(db_path_ptr, size_ptr);
            assert_eq!(char_pointer_to_string(result).unwrap(), "");
            assert_eq!(size, items.len() as isize);

            let mut read_items: Vec<CHistoryItem> = Vec::with_capacity(size as usize);
            let read_items_ptr = read_items.as_mut_ptr();
            let result = read_history_items(db_path_ptr, read_items_ptr);
            assert_eq!(char_pointer_to_string(result).unwrap(), "");
            read_items.set_len(size as usize);
        }

        temp_path.close().unwrap();
    }

    #[test]
    fn write_relationship() {
        let temp_path = NamedTempFile::new().unwrap().into_temp_path();
        let db_path = temp_path.as_os_str();
        let db_path_ptr = string_to_char_pointer(&db_path.to_str().unwrap());
        let relationship1 = CEntityRelationship {
            parent: string_to_char_pointer("testparent1"),
            child: string_to_char_pointer("testchild1"),
            role: string_to_char_pointer("testrole1"),
        };
        let relationship2 = CEntityRelationship {
            parent: string_to_char_pointer("testparent2"),
            child: string_to_char_pointer("testchild2"),
            role: string_to_char_pointer("testrole2"),
        };
        let relationships = [relationship1, relationship2];
        let relationships_ptr = relationships.as_ptr();

        unsafe {
            let result =
                write_relationships(db_path_ptr, relationships_ptr, relationships.len() as isize);
            assert_eq!(char_pointer_to_string(result).unwrap(), "");

            let mut size: isize = 0;
            let size_ptr: *mut isize = &mut size;
            let result = get_number_of_relationships(db_path_ptr, size_ptr);
            assert_eq!(char_pointer_to_string(result).unwrap(), "");
            assert_eq!(size, relationships.len() as isize);

            let mut read_rels: Vec<CEntityRelationship> = Vec::with_capacity(size as usize);
            let read_relationships_ptr = read_rels.as_mut_ptr();
            let result = read_relationships(db_path_ptr, read_relationships_ptr);
            assert_eq!(char_pointer_to_string(result).unwrap(), "");
            read_rels.set_len(size as usize);
        }

        temp_path.close().unwrap();
    }
}
