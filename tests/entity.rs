use lorecore::sql::{
    entity::EntityColumn, lore_database::LoreDatabase, search_text::SqlSearchText,
};
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

fn create_example() -> (tempfile::TempPath, LoreDatabase, Vec<String>, Vec<String>) {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();

    let labels = vec!["testlabel1".to_string(), "testlabel2and_stuff".to_string()];
    let descriptors = vec![
        "testdescriptor1".to_string(),
        "testdescriptor2and_stuff".to_string(),
    ];
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
    (temp_path, db, labels, descriptors)
}

#[test]
fn get_all_labels() {
    let (temp_path, db, labels, _descriptors) = create_example();

    let labels_out = db.get_entity_labels(SqlSearchText::empty()).unwrap();
    assert!(labels.len() == labels_out.len());
    for label in labels.iter() {
        assert!(labels_out.contains(label));
    }

    temp_path.close().unwrap();
}

#[test]
fn get_labels_with_filter() {
    let (temp_path, db, labels, _descriptors) = create_example();

    let no_result = db.get_entity_labels(SqlSearchText::new("fununu")).unwrap();
    assert!(no_result.len() == 0);

    let label1_out = db.get_entity_labels(SqlSearchText::new("bel1")).unwrap();
    assert!(label1_out.len() == 1);
    assert!(label1_out[0] == labels[0]);

    let label2_out = db.get_entity_labels(SqlSearchText::new("bel2")).unwrap();
    assert!(label2_out.len() == 1);
    assert!(label2_out[0] == labels[1]);

    let all_labels_out = db.get_entity_labels(SqlSearchText::new("bel")).unwrap();
    assert!(all_labels_out.len() == 2);
    for label in labels.iter() {
        assert!(all_labels_out.contains(label));
    }

    temp_path.close().unwrap();
}

#[test]
fn get_descriptors() {
    let (temp_path, db, labels, descriptors) = create_example();

    let no_descriptors_out = db
        .get_descriptors(&labels[0], SqlSearchText::new("fununu"))
        .unwrap();
    assert!(no_descriptors_out.len() == 0);

    let descriptor1_out = db
        .get_descriptors(&labels[0], SqlSearchText::new("riptor1"))
        .unwrap();
    assert!(descriptor1_out.len() == 1);
    assert!(descriptor1_out[0] == descriptors[0]);

    let descriptor2_out = db
        .get_descriptors(&labels[0], SqlSearchText::new("riptor2"))
        .unwrap();
    assert!(descriptor2_out.len() == 1);
    assert!(descriptor2_out[0] == descriptors[1]);

    let all_descriptors_out = db
        .get_descriptors(&labels[0], SqlSearchText::new("cript"))
        .unwrap();
    assert!(all_descriptors_out.len() == descriptors.len());
    for descriptor in descriptors.iter() {
        assert!(all_descriptors_out.contains(descriptor));
    }

    temp_path.close().unwrap();
}
