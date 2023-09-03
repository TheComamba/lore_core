use lorecore::sql::{entity::EntityColumn, lore_database::LoreDatabase};
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[test]
fn writing_single_entity_column() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();
    let entity = EntityColumn {
        label: "testlabel".to_string(),
        descriptor: "testdescriptor".to_string(),
        description: Some("testdescription".to_string()),
    };
    db.write_entity_columns(vec![entity.clone()]).unwrap();
    let entity_out = db.get_all_entity_columns().unwrap();
    assert!(entity_out.len() == 1);
    assert!(entity == entity_out[0]);
    temp_path.close().unwrap();
}

#[test]
fn write_many_entity_columns() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();

    let labels = vec!["testlabel1".to_string(), "testlabel2".to_string()];
    let descriptors = vec!["testdescriptor1".to_string(), "testdescriptor2".to_string()];
    let mut entities: Vec<EntityColumn> = Vec::new();
    for label in labels.iter() {
        for descriptor in descriptors.iter() {
            entities.push(EntityColumn {
                label: label.clone(),
                descriptor: descriptor.clone(),
                description: Some(label.clone() + descriptor),
            });
        }
    }

    db.write_entity_columns(entities.clone()).unwrap();

    let entities_out = db.get_all_entity_columns().unwrap();
    assert!(entities.len() == entities_out.len());
    for entity in entities.iter() {
        assert!(entities_out.contains(entity));
    }
    temp_path.close().unwrap();
}

#[test]
fn write_entity_with_empty_description() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();
    let entity = EntityColumn {
        label: "testlabel".to_string(),
        descriptor: "testdescriptor".to_string(),
        description: None,
    };
    db.write_entity_columns(vec![entity.clone()]).unwrap();
    let entity_out = db.get_all_entity_columns().unwrap();
    assert!(entity_out.len() == 1);
    assert!(entity == entity_out[0]);
    temp_path.close().unwrap();
}
