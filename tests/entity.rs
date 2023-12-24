use lorecore::sql::{
    entity::{get_descriptors, get_labels, EntityColumn},
    lore_database::LoreDatabase,
    search_params::{EntityColumnSearchParams, SqlSearchText},
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
        .get_entity_columns(EntityColumnSearchParams::empty())
        .unwrap();
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

    let entities_out = db
        .get_entity_columns(EntityColumnSearchParams::empty())
        .unwrap();
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
    let entity_out = db
        .get_entity_columns(EntityColumnSearchParams::empty())
        .unwrap();
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

fn check_output(
    out: Vec<EntityColumn>,
    expected_labels: &Vec<String>,
    expected_descriptors: &Vec<String>,
) {
    let labels_out = get_labels(&out);
    let descriptors_out = get_descriptors(&out);
    assert!(out.len() == expected_labels.len() * expected_descriptors.len());
    assert!(labels_out.len() == expected_labels.len());
    assert!(descriptors_out.len() == expected_descriptors.len());
    for label in expected_labels.iter() {
        assert!(labels_out.contains(label));
    }
    for descriptor in expected_descriptors.iter() {
        assert!(descriptors_out.contains(descriptor));
    }
    for label in expected_labels.iter() {
        for descriptor in expected_descriptors.iter() {
            let description = label.clone() + descriptor;
            assert!(out
                .iter()
                .any(|c| c.description == Some(description.to_owned())))
        }
    }
}

#[test]
fn get_all_entity_columns() {
    let (temp_path, db, labels, descriptors) = create_example();

    let out = db
        .get_entity_columns(EntityColumnSearchParams::empty())
        .unwrap();
    check_output(out, &labels, &descriptors);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_label_filter() {
    let (temp_path, db, labels, descriptors) = create_example();

    let no_result = db
        .get_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("fununu")),
            None,
        ))
        .unwrap();
    check_output(no_result, &vec![], &vec![]);

    let label1_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("bel1")),
            None,
        ))
        .unwrap();
    check_output(label1_out, &vec![labels[0].clone()], &descriptors);

    let all_labels_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("bel")),
            None,
        ))
        .unwrap();
    check_output(all_labels_out, &labels, &descriptors);

    let label1_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("testlabel1")),
            None,
        ))
        .unwrap();
    check_output(label1_out, &vec![labels[0].clone()], &descriptors);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_exact_label_filter() {
    let (temp_path, db, labels, descriptors) = create_example();

    let no_result = db
        .get_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact("fununu")),
            None,
        ))
        .unwrap();
    check_output(no_result, &vec![], &vec![]);

    let all_labels_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact("bel")),
            None,
        ))
        .unwrap();
    check_output(all_labels_out, &vec![], &vec![]);

    let label1_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact("testlabel1")),
            None,
        ))
        .unwrap();
    check_output(label1_out, &vec![labels[0].clone()], &descriptors);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_descriptor_filter() {
    let (temp_path, db, labels, descriptors) = create_example();

    let no_result = db
        .get_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::partial("fununu")),
        ))
        .unwrap();
    check_output(no_result, &vec![], &vec![]);

    let descriptor1_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::partial("riptor1")),
        ))
        .unwrap();
    check_output(descriptor1_out, &labels, &vec![descriptors[0].clone()]);

    let all_descriptors_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::partial("riptor")),
        ))
        .unwrap();
    check_output(all_descriptors_out, &labels, &descriptors);

    let descriptor1_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::partial("testdescriptor1")),
        ))
        .unwrap();
    check_output(descriptor1_out, &labels, &vec![descriptors[0].clone()]);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_exact_descriptor_filter() {
    let (temp_path, db, labels, descriptors) = create_example();

    let no_result = db
        .get_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::exact("fununu")),
        ))
        .unwrap();
    check_output(no_result, &vec![], &vec![]);

    let all_descriptors_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::exact("riptor")),
        ))
        .unwrap();
    check_output(all_descriptors_out, &vec![], &vec![]);

    let descriptor1_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            None,
            Some(SqlSearchText::exact("testdescriptor1")),
        ))
        .unwrap();
    check_output(descriptor1_out, &labels, &vec![descriptors[0].clone()]);

    temp_path.close().unwrap();
}

#[test]
fn get_entities_with_label_and_descriptor_filter() {
    let (temp_path, db, labels, descriptors) = create_example();

    let no_result = db
        .get_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("bel")),
            Some(SqlSearchText::partial("fununu")),
        ))
        .unwrap();
    check_output(no_result, &vec![], &vec![]);

    let no_result = db
        .get_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("fununu")),
            Some(SqlSearchText::partial("riptor")),
        ))
        .unwrap();
    check_output(no_result, &vec![], &vec![]);

    let label1_descriptor1_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("bel1")),
            Some(SqlSearchText::partial("riptor1")),
        ))
        .unwrap();
    check_output(
        label1_descriptor1_out,
        &vec![labels[0].clone()],
        &vec![descriptors[0].clone()],
    );

    let label1_descriptor1_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact("testlabel1")),
            Some(SqlSearchText::exact("testdescriptor1")),
        ))
        .unwrap();
    check_output(
        label1_descriptor1_out,
        &vec![labels[0].clone()],
        &vec![descriptors[0].clone()],
    );

    let label1_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::exact("testlabel1")),
            Some(SqlSearchText::partial("riptor")),
        ))
        .unwrap();
    check_output(label1_out, &vec![labels[0].clone()], &descriptors);

    let descriptor1_out = db
        .get_entity_columns(EntityColumnSearchParams::new(
            Some(SqlSearchText::partial("bel")),
            Some(SqlSearchText::exact("testdescriptor1")),
        ))
        .unwrap();
    check_output(descriptor1_out, &labels, &vec![descriptors[0].clone()]);

    temp_path.close().unwrap();
}
