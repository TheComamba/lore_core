use lorecore::sql::{
    lore_database::LoreDatabase,
    relationships::{get_children, get_parents, get_roles, EntityRelationship},
    search_params::{RelationshipSearchParams, SqlSearchText},
};
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
    let rel_out = db
        .get_relationships(RelationshipSearchParams::empty())
        .unwrap();
    assert!(rel_out.len() == 1);
    assert!(rel == rel_out[0]);
    temp_path.close().unwrap();
}

#[test]
fn write_many_relationships() {
    let (temp_path, db, rels) = create_example();

    let rels_out = db
        .get_relationships(RelationshipSearchParams::empty())
        .unwrap();
    assert!(rels.len() == rels_out.len());
    for rel in rels.iter() {
        assert!(rels_out.contains(rel));
    }
    temp_path.close().unwrap();
}

fn create_example() -> (tempfile::TempPath, LoreDatabase, Vec<EntityRelationship>) {
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
    (temp_path, db, rels)
}

fn check_output(
    rels: &Vec<EntityRelationship>,
    parents: &Vec<String>,
    children: &Vec<String>,
    roles: &Vec<Option<String>>,
) {
    let parents_out = get_parents(rels);
    assert!(parents.len() == parents_out.len());
    for parent in parents.iter() {
        assert!(parents_out.contains(parent));
    }

    let children_out = get_children(rels);
    assert!(children.len() == children_out.len());
    for child in children.iter() {
        assert!(children_out.contains(child));
    }

    let roles_out = get_roles(rels);
    assert!(roles.len() == roles_out.len());
    for role in roles.iter() {
        assert!(roles_out.contains(role));
    }
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

    let rels_out = db
        .get_relationships(RelationshipSearchParams::empty())
        .unwrap();
    assert!(rels.len() == rels_out.len());
    for rel in rels.iter() {
        assert!(rels_out.contains(rel));
    }
    temp_path.close().unwrap();
}

#[test]
fn get_all_relationships() {
    let (temp_path, db, rels) = create_example();

    let rels_out = db
        .get_relationships(RelationshipSearchParams::empty())
        .unwrap();
    check_output(
        &rels_out,
        &get_parents(&rels),
        &get_children(&rels),
        &get_roles(&rels),
    );

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_parent_filter() {
    let (temp_path, db, rels) = create_example();

    let all_parents = get_parents(&rels);
    let all_children = get_children(&rels);
    let all_roles = get_roles(&rels);

    let no_rel = db
        .get_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("fununu")),
            None,
        ))
        .unwrap();
    check_output(&no_rel, &vec![], &vec![], &vec![]);

    let parent1 = db
        .get_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("rent1")),
            None,
        ))
        .unwrap();
    check_output(
        &parent1,
        &vec!["testparent1".to_string()],
        &all_children,
        &all_roles,
    );

    let parent1 = db
        .get_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("rent")),
            None,
        ))
        .unwrap();
    check_output(&parent1, &all_parents, &all_children, &all_roles);

    let parent1 = db
        .get_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("testparent1")),
            None,
        ))
        .unwrap();
    check_output(
        &parent1,
        &vec!["testparent1".to_string()],
        &all_children,
        &all_roles,
    );

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_exact_parent_filter() {
    let (temp_path, db, rels) = create_example();

    let no_rel = db
        .get_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::exact("fununu")),
            None,
        ))
        .unwrap();
    check_output(&no_rel, &vec![], &vec![], &vec![]);

    let parent1 = db
        .get_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::exact("rent")),
            None,
        ))
        .unwrap();
    check_output(&parent1, &vec![], &vec![], &vec![]);

    let parent1 = db
        .get_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::exact("testparent1")),
            None,
        ))
        .unwrap();
    check_output(
        &parent1,
        &vec!["testparent1".to_string()],
        &get_children(&rels),
        &get_roles(&rels),
    );

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_child_filter() {
    let (temp_path, db, rels) = create_example();

    let all_parents = get_parents(&rels);
    let all_children = get_children(&rels);
    let all_roles = get_roles(&rels);

    let no_rel = db
        .get_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::partial("fununu")),
        ))
        .unwrap();
    check_output(&no_rel, &vec![], &vec![], &vec![]);

    let child1 = db
        .get_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::partial("ild1")),
        ))
        .unwrap();
    check_output(
        &child1,
        &all_parents,
        &vec!["testchild1".to_string()],
        &all_roles,
    );

    let child1 = db
        .get_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::partial("ild")),
        ))
        .unwrap();
    check_output(&child1, &all_parents, &all_children, &all_roles);

    let child1 = db
        .get_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::partial("testchild1")),
        ))
        .unwrap();
    check_output(
        &child1,
        &all_parents,
        &vec!["testchild1".to_string()],
        &all_roles,
    );

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_exact_child_filter() {
    let (temp_path, db, rels) = create_example();

    let no_rel = db
        .get_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::exact("fununu")),
        ))
        .unwrap();
    check_output(&no_rel, &vec![], &vec![], &vec![]);

    let child1 = db
        .get_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::exact("ild")),
        ))
        .unwrap();
    check_output(&child1, &vec![], &vec![], &vec![]);

    let child1 = db
        .get_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::exact("testchild1")),
        ))
        .unwrap();
    check_output(
        &child1,
        &get_parents(&rels),
        &vec!["testchild1".to_string()],
        &get_roles(&rels),
    );

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_parent_and_child_filter() {
    let (temp_path, db, rels) = create_example();

    let all_parents = get_parents(&rels);
    let all_children = get_children(&rels);
    let all_roles = get_roles(&rels);

    let no_rel = db
        .get_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("fununu")),
            Some(SqlSearchText::partial("ild")),
        ))
        .unwrap();
    check_output(&no_rel, &vec![], &vec![], &vec![]);

    let no_rel = db
        .get_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("rent")),
            Some(SqlSearchText::partial("fununu")),
        ))
        .unwrap();
    check_output(&no_rel, &vec![], &vec![], &vec![]);

    let parent1_child1 = db
        .get_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("rent1")),
            Some(SqlSearchText::partial("ild1")),
        ))
        .unwrap();
    check_output(
        &parent1_child1,
        &vec!["testparent1".to_string()],
        &vec!["testchild1".to_string()],
        &all_roles,
    );

    let parent1_child1 = db
        .get_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("rent")),
            Some(SqlSearchText::partial("ild")),
        ))
        .unwrap();
    check_output(&parent1_child1, &all_parents, &all_children, &all_roles);

    let parent1_child1 = db
        .get_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("testparent1")),
            Some(SqlSearchText::partial("testchild1")),
        ))
        .unwrap();
    check_output(
        &parent1_child1,
        &vec!["testparent1".to_string()],
        &vec!["testchild1".to_string()],
        &all_roles,
    );

    temp_path.close().unwrap();
}
