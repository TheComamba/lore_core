use std::path::PathBuf;

use lorecore::sql::{entity::EntityColumn, lore_database::LoreDatabase};
use tempfile::NamedTempFile;

#[test]
fn writing_single_entity_column() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();
    let entity = EntityColumn {
        label: "testlabel".to_string(),
        descriptor: "testdescriptor".to_string(),
        description: "testdescription".to_string(),
    };
    db.write_entity_columns(vec![entity.clone()]).unwrap();
    let entity_out = db.get_all_entity_columns().unwrap();
    assert!(entity_out.len() == 1);
    assert!(entity == entity_out[0]);
    temp_path.close().unwrap();
}
