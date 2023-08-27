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

#[test]
fn write_many_relationships() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();

    let parents = vec!["testparent1".to_string(), "testparent2".to_string()];
    let children = vec!["testchild1".to_string(), "testchild2".to_string()];
    let roles = vec![Some("testrole".to_string()), None];
    let mut rels: Vec<EntityRelationship> = Vec::new();
    for parent in parents.iter() {
        for child in children.iter() {
            for role in roles.iter() {
                rels.push(EntityRelationship {
                    parent: parent.clone(),
                    child: child.clone(),
                    role: role.clone(),
                });
            }
        }
    }

    db.write_relationships(rels.clone()).unwrap();

    let rels_out = db.get_relationships().unwrap();
    assert!(rels.len() == rels_out.len());
    for rel in rels.iter() {
        assert!(rels_out.contains(rel));
    }
    temp_path.close().unwrap();
}

#[test]
fn writing_several_roles_to_same_relationship() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();

    let parent = "testparent".to_string();
    let child = "testchild".to_string();
    let roles = vec!["testrole1".to_string(), "testrole2".to_string()];
    let mut rels: Vec<EntityRelationship> = Vec::new();
    for role in roles.iter() {
        rels.push(EntityRelationship {
            parent: parent.clone(),
            child: child.clone(),
            role: Some(role.clone()),
        });
    }

    db.write_relationships(rels.clone()).unwrap();

    let rels_out = db.get_relationships().unwrap();
    assert!(rels.len() == rels_out.len());
    for rel in rels.iter() {
        assert!(rels_out.contains(rel));
    }
    temp_path.close().unwrap();
}
