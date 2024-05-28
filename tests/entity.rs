use lorecore::{
    sql::{
        lore_database::LoreDatabase,
        search_params::{EntityColumnSearchParams, SqlSearchText},
    },
    types::entity::EntityColumn,
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
    let entity_out = db
        .read_entity_columns(EntityColumnSearchParams::empty())
        .unwrap();
    assert_eq!(entity_out.len(), 1);
    assert_eq!(entity, entity_out[0]);
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

    let entities_out = db
        .read_entity_columns(EntityColumnSearchParams::empty())
        .unwrap();
    assert_eq!(entities.len(), entities_out.len());
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
    let entity_out = db
        .read_entity_columns(EntityColumnSearchParams::empty())
        .unwrap();
    assert_eq!(entity_out.len(), 1);
    assert_eq!(entity, entity_out[0]);
    temp_path.close().unwrap();
}

fn create_example() -> (tempfile::TempPath, LoreDatabase, Vec<EntityColumn>) {
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
    entities.sort();

    db.write_entity_columns(entities.clone()).unwrap();
    (temp_path, db, entities)
}

#[test]
fn get_all_entities_without_filter_returns_all() {
    let (temp_path, db, entities) = create_example();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::empty())
        .unwrap();
    assert_eq!(out, entities);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_label_filter_fununu_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("fununu")),
            None,
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_label_filter_bel1_returns_some() {
    let (temp_path, db, entities) = create_example();
    let expected: Vec<_> = entities
        .iter()
        .filter(|e| e.label == "testlabel1".to_string())
        .cloned()
        .collect();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("bel1")),
            None,
        ))
        .unwrap();
    assert_eq!(out, expected);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_label_filter_testlabel1_returns_some() {
    let (temp_path, db, entities) = create_example();
    let expected: Vec<_> = entities
        .iter()
        .filter(|e| e.label == "testlabel1".to_string())
        .cloned()
        .collect();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("testlabel1")),
            None,
        ))
        .unwrap();
    assert_eq!(out, expected);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_label_filter_bel_returns_all() {
    let (temp_path, db, entities) = create_example();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("bel")),
            None,
        ))
        .unwrap();
    assert_eq!(out, entities);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_exact_label_filter_fununu_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact("fununu")),
            None,
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_exact_label_filter_bel1_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact("bel")),
            None,
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_exact_label_filter_testlabel1_returns_some() {
    let (temp_path, db, entities) = create_example();
    let expected: Vec<_> = entities
        .iter()
        .filter(|e| e.label == "testlabel1".to_string())
        .cloned()
        .collect();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact("testlabel1")),
            None,
        ))
        .unwrap();
    assert_eq!(out, expected);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_descriptor_filter_fununu_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::partial("fununu")),
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_descriptor_filter_riptor1_returns_some() {
    let (temp_path, db, entities) = create_example();
    let expected: Vec<_> = entities
        .iter()
        .filter(|e| e.descriptor == "testdescriptor1".to_string())
        .cloned()
        .collect();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::partial("riptor1")),
        ))
        .unwrap();
    assert_eq!(out, expected);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_descriptor_filter_testdescriptor1_returns_some() {
    let (temp_path, db, entities) = create_example();
    let expected: Vec<_> = entities
        .iter()
        .filter(|e| e.descriptor == "testdescriptor1".to_string())
        .cloned()
        .collect();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::partial("testdescriptor1")),
        ))
        .unwrap();
    assert_eq!(out, expected);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_descriptor_filter_riptor_returns_all() {
    let (temp_path, db, entities) = create_example();

    let all_descriptors_out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::partial("riptor")),
        ))
        .unwrap();
    assert_eq!(all_descriptors_out, entities);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_exact_descriptor_filter_fununu_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::exact("fununu")),
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_exact_descriptor_filter_riptor_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::exact("riptor")),
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_exact_descriptor_filter_testdescriptor1_returns_some() {
    let (temp_path, db, entities) = create_example();
    let expected: Vec<_> = entities
        .iter()
        .filter(|e| e.descriptor == "testdescriptor1".to_string())
        .cloned()
        .collect();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::exact("testdescriptor1")),
        ))
        .unwrap();
    assert_eq!(out, expected);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_label_filter_bel_and_descriptor_filter_fununu_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("bel")),
            Some(SqlSearchText::partial("fununu")),
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_label_filter_fununu_and_descriptor_filter_riptor_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("fununu")),
            Some(SqlSearchText::partial("riptor")),
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_label_filter_bel1_and_descriptor_filter_riptor1_returns_some() {
    let (temp_path, db, entities) = create_example();
    let expected: Vec<_> = entities
        .iter()
        .filter(|e| {
            e.label == "testlabel1".to_string() && e.descriptor == "testdescriptor1".to_string()
        })
        .cloned()
        .collect();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("bel1")),
            Some(SqlSearchText::partial("riptor1")),
        ))
        .unwrap();
    assert_eq!(out, expected);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_exact_label_filter_testlabel1_and_exact_descriptor_filter_testdescriptor1_returns_some(
) {
    let (temp_path, db, entities) = create_example();
    let expected: Vec<_> = entities
        .iter()
        .filter(|e| {
            e.label == "testlabel1".to_string() && e.descriptor == "testdescriptor1".to_string()
        })
        .cloned()
        .collect();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact("testlabel1")),
            Some(SqlSearchText::exact("testdescriptor1")),
        ))
        .unwrap();
    assert_eq!(out, expected);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_exact_label_filter_testlabel1_and_descriptor_filter_riptor_returns_some() {
    let (temp_path, db, entities) = create_example();
    let expected: Vec<_> = entities
        .iter()
        .filter(|e| e.label == "testlabel1".to_string())
        .cloned()
        .collect();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact("testlabel1")),
            Some(SqlSearchText::partial("riptor")),
        ))
        .unwrap();
    assert_eq!(out, expected);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_label_filter_bel_and_exact_descriptor_filter_testdescriptor1_returns_some() {
    let (temp_path, db, entities) = create_example();
    let expected: Vec<_> = entities
        .iter()
        .filter(|e| e.descriptor == "testdescriptor1".to_string())
        .cloned()
        .collect();

    let out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("bel")),
            Some(SqlSearchText::exact("testdescriptor1")),
        ))
        .unwrap();
    assert_eq!(out, expected);

    temp_path.close().unwrap();
}

#[test]
fn test_write_read_after_db_deletion() {
    let (temp_path, db, _) = create_example();
    temp_path.close().unwrap();

    let write_result = db.write_entity_columns(vec![EntityColumn {
        label: "testlabel".to_string(),
        descriptor: "testdescriptor".to_string(),
        description: Some("testdescription".to_string()),
    }]);
    assert!(
        write_result.is_err(),
        "Expected an error when writing to a deleted database"
    );

    let read_result = db.read_entity_columns(EntityColumnSearchParams::new(None, None));
    assert!(
        read_result.is_err(),
        "Expected an error when reading from a deleted database"
    );
}

#[test]
fn test_relabel_entity() {
    let (temp_path, db, entities) = create_example();
    let old_entity = entities[0].clone();
    let new_label = "New_Label".to_string();

    // Relabel the entity
    db.relabel_entity(&old_entity.label, &new_label).unwrap();

    // Read the entity back from the database
    let updated_entity = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact(&new_label)),
            None,
        ))
        .unwrap();

    // Check that the entity was updated correctly
    assert!(!updated_entity.is_empty());
    assert_eq!(updated_entity[0].label, new_label);
    assert_eq!(updated_entity[0].descriptor, old_entity.descriptor);
    assert_eq!(updated_entity[0].description, old_entity.description);

    temp_path.close().unwrap();
}

#[test]
fn test_delete_entity() {
    let (temp_path, db, entities) = create_example();
    let entity = entities[0].clone();

    // Verify the entity exists
    let entity_out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact(&entity.label)),
            None,
        ))
        .unwrap();
    assert!(!entity_out.is_empty());
    assert_eq!(entity, entity_out[0]);

    // Delete the entity
    db.delete_entity(entity.label.clone()).unwrap();

    // Verify the entity no longer exists
    let entity_out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact(&entity.label)),
            None,
        ))
        .unwrap();
    assert_eq!(entity_out.len(), 0);

    temp_path.close().unwrap();
}

#[test]
fn test_change_entity_descriptor() {
    let (temp_path, db, entities) = create_example();
    let old_entity = entities[0].clone();
    let new_descriptor = "New_Descriptor".to_string();

    // Change the entity's descriptor
    db.change_entity_descriptor((&old_entity.label, old_entity.descriptor), &new_descriptor)
        .unwrap();

    // Read the entity back from the database
    let updated_entity = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact(&old_entity.label)),
            None,
        ))
        .unwrap();

    // Check that the entity was updated correctly
    assert!(!updated_entity.is_empty());
    assert_eq!(updated_entity[0].label, old_entity.label);
    assert_eq!(updated_entity[0].descriptor, new_descriptor);
    assert_eq!(updated_entity[0].description, old_entity.description);

    temp_path.close().unwrap();
}

#[test]
fn test_delete_entity_column() {
    let (temp_path, db, entities) = create_example();
    let entity = entities[0].clone();

    // Verify the entity exists
    let entity_out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact(&entity.label)),
            None,
        ))
        .unwrap();
    assert!(!entity_out.is_empty());
    assert_eq!(entity, entity_out[0]);

    // Delete the entity column
    db.delete_entity_column((entity.label.clone(), entity.descriptor.clone()))
        .unwrap();

    // Verify the entity column no longer exists
    let entity_out = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact(&entity.label)),
            Some(SqlSearchText::exact(&entity.descriptor)),
        ))
        .unwrap();
    assert!(entity_out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn test_change_entity_description() {
    let (temp_path, db, entities) = create_example();
    let old_entity = entities[0].clone();
    let new_description = Some("New_Description".to_string());

    // Change the entity's description
    db.change_entity_description(
        (&old_entity.label, &old_entity.descriptor),
        &new_description.clone(),
    )
    .unwrap();

    // Read the entity back from the database
    let updated_entity = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact(&old_entity.label)),
            None,
        ))
        .unwrap();

    // Check that the entity was updated correctly
    assert!(!updated_entity.is_empty());
    assert_eq!(updated_entity[0].label, old_entity.label);
    assert_eq!(updated_entity[0].descriptor, old_entity.descriptor);
    assert_eq!(updated_entity[0].description, new_description);

    temp_path.close().unwrap();
}

#[test]
fn test_change_entity_description_to_none() {
    let (temp_path, db, entities) = create_example();
    let old_entity = entities[0].clone();
    let new_description = None;

    // Change the entity's description
    db.change_entity_description(
        (&old_entity.label, &old_entity.descriptor),
        &new_description.clone(),
    )
    .unwrap();

    // Read the entity back from the database
    let updated_entity = db
        .read_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact(&old_entity.label)),
            None,
        ))
        .unwrap();

    // Check that the entity was updated correctly
    assert!(!updated_entity.is_empty());
    assert_eq!(updated_entity[0].label, old_entity.label);
    assert_eq!(updated_entity[0].descriptor, old_entity.descriptor);
    assert_eq!(updated_entity[0].description, new_description);

    temp_path.close().unwrap();
}
