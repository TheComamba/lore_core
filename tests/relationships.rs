use lorecore::sql::{lore_database::LoreDatabase, relationships::EntityRelationship};
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[test]
fn write_single_relationship() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();
    let rel = EntityRelationship {
        parent: "testparent".to_string(),
        child: "testchild".to_string(),
        role: Some("testrole".to_string()),
    };
    db.write_relationships(vec![rel.clone()]).unwrap();
    let rel_out = db.get_relationships().unwrap();
    assert!(rel_out.len() == 1);
    assert!(rel == rel_out[0]);
    temp_path.close().unwrap();
}
