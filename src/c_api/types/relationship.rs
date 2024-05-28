use crate::{
    c_api::auxil::{
        char_pointer_to_optional_string, char_pointer_to_string, optional_string_to_char_pointer,
        string_to_char_pointer,
    },
    errors::LoreCoreError,
    sql::relationship::EntityRelationship,
};

#[repr(C)]
#[derive(Clone)]
pub struct CEntityRelationship {
    pub parent: *const libc::c_char,
    pub child: *const libc::c_char,
    pub role: *const libc::c_char,
}

fn to_c_relationship(rel: &EntityRelationship) -> Result<CEntityRelationship, LoreCoreError> {
    Ok(CEntityRelationship {
        parent: string_to_char_pointer(&rel.parent),
        child: string_to_char_pointer(&rel.child),
        role: optional_string_to_char_pointer(&rel.role),
    })
}

impl TryFrom<EntityRelationship> for CEntityRelationship {
    type Error = LoreCoreError;

    fn try_from(value: EntityRelationship) -> Result<Self, Self::Error> {
        to_c_relationship(&value)
    }
}

unsafe fn to_relationship(rel: &CEntityRelationship) -> Result<EntityRelationship, LoreCoreError> {
    Ok(EntityRelationship {
        parent: char_pointer_to_string(rel.parent)?,
        child: char_pointer_to_string(rel.child)?,
        role: char_pointer_to_optional_string(rel.role)?,
    })
}

impl TryFrom<&CEntityRelationship> for EntityRelationship {
    type Error = LoreCoreError;

    fn try_from(value: &CEntityRelationship) -> Result<Self, Self::Error> {
        unsafe { to_relationship(value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relationship_roundtrips() {
        let parents = vec!["testparent", "_testparent2"];
        let children = vec!["testchild", "_testchild2"];
        let roles = vec![
            None,
            Some("Test role".to_string()),
            Some("\\_\"'%$&!{[]}".to_string()),
        ];
        for parent in &parents {
            for child in &children {
                for role in &roles {
                    let rel_before = EntityRelationship {
                        parent: parent.to_string(),
                        child: child.to_string(),
                        role: role.clone(),
                    };
                    let c_rel = to_c_relationship(&rel_before).unwrap();
                    let rel_after = unsafe { to_relationship(&c_rel).unwrap() };
                    assert_eq!(rel_before, rel_after);
                }
            }
        }
    }
}
