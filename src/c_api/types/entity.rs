use crate::{
    c_api::auxil::{
        char_pointer_to_optional_string, char_pointer_to_string, optional_string_to_char_pointer,
        string_to_char_pointer,
    },
    errors::LoreCoreError,
    types::entity::EntityColumn,
};

#[repr(C)]
#[derive(Clone)]
pub struct CEntityColumn {
    pub label: *const libc::c_char,
    pub descriptor: *const libc::c_char,
    pub description: *const libc::c_char,
}

fn to_c_entity_column(column: &EntityColumn) -> Result<CEntityColumn, LoreCoreError> {
    Ok(CEntityColumn {
        label: string_to_char_pointer(column.label.to_str()),
        descriptor: string_to_char_pointer(&column.descriptor.to_str()),
        description: string_to_char_pointer(&column.description.to_str()),
    })
}

impl TryFrom<EntityColumn> for CEntityColumn {
    type Error = LoreCoreError;

    fn try_from(value: EntityColumn) -> Result<Self, Self::Error> {
        to_c_entity_column(&value)
    }
}

unsafe fn to_entity_column(column: &CEntityColumn) -> Result<EntityColumn, LoreCoreError> {
    Ok(EntityColumn {
        label: char_pointer_to_string(column.label)?.into(),
        descriptor: char_pointer_to_string(column.descriptor)?.into(),
        description: char_pointer_to_string(column.description)?.into(),
    })
}

impl TryFrom<&CEntityColumn> for EntityColumn {
    type Error = LoreCoreError;

    fn try_from(value: &CEntityColumn) -> Result<Self, Self::Error> {
        unsafe { to_entity_column(value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_column_roundtrips() {
        let labels = vec!["testlabel", "_testlabel2"];
        let descriptors = vec!["testdescriptor", "_protected"];
        let descriptions = vec![
            None,
            Some("Test description".to_string()),
            Some("\\_\"'%$&!{[]}".to_string()),
        ];
        for label in labels {
            for descriptor in &descriptors {
                for description in &descriptions {
                    let column_before = EntityColumn {
                        label: label.into(),
                        descriptor: (*descriptor).into(),
                        description: description.clone().into(),
                    };
                    let c_column = to_c_entity_column(&column_before).unwrap();
                    let column_after = unsafe { to_entity_column(&c_column).unwrap() };
                    assert_eq!(column_before, column_after);
                }
            }
        }
    }
}
