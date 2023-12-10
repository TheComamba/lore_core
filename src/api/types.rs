use super::auxil::{
    char_pointer_to_optional_string, char_pointer_to_string, optional_string_to_char_pointer,
    string_to_char_pointer,
};
use crate::{
    errors::LoreCoreError,
    sql::{entity::EntityColumn, history::HistoryItem, relationships::EntityRelationship},
};

#[repr(C)]
#[derive(Clone)]
pub struct CEntityColumn {
    pub label: *const libc::c_char,
    pub descriptor: *const libc::c_char,
    pub description: *const libc::c_char,
}

#[repr(C)]
#[derive(Clone)]
pub struct CEntityRelationship {
    pub parent: *const libc::c_char,
    pub child: *const libc::c_char,
    pub role: *const libc::c_char,
}

#[repr(C)]
#[derive(Clone)]
pub struct CHistoryItem {
    pub timestamp: i64,
    pub year: i32,
    pub day: i32,
    pub content: *const libc::c_char,
    pub properties: *const libc::c_char,
}

pub(super) fn to_c_entity_column(column: &EntityColumn) -> Result<CEntityColumn, LoreCoreError> {
    Ok(CEntityColumn {
        label: string_to_char_pointer(&column.label),
        descriptor: string_to_char_pointer(&column.descriptor),
        description: optional_string_to_char_pointer(&column.description),
    })
}

pub(super) unsafe fn to_entity_column(
    column: &CEntityColumn,
) -> Result<EntityColumn, LoreCoreError> {
    Ok(EntityColumn {
        label: char_pointer_to_string(column.label)?,
        descriptor: char_pointer_to_string(column.descriptor)?,
        description: char_pointer_to_optional_string(column.description)?,
    })
}

pub(super) fn to_c_history_item(item: &HistoryItem) -> Result<CHistoryItem, LoreCoreError> {
    Ok(CHistoryItem {
        timestamp: item.timestamp,
        year: item.year,
        day: if let Some(day) = item.day { day } else { 0 },
        content: string_to_char_pointer(&item.content),
        properties: optional_string_to_char_pointer(&item.properties),
    })
}

pub(super) unsafe fn to_history_item(item: &CHistoryItem) -> Result<HistoryItem, LoreCoreError> {
    Ok(HistoryItem {
        timestamp: item.timestamp,
        year: item.year,
        day: if item.day > 0 { Some(item.day) } else { None },
        content: char_pointer_to_string(item.content)?,
        properties: char_pointer_to_optional_string(item.properties)?,
    })
}

pub(super) fn to_c_relationship(
    rel: &EntityRelationship,
) -> Result<CEntityRelationship, LoreCoreError> {
    Ok(CEntityRelationship {
        parent: string_to_char_pointer(&rel.parent),
        child: string_to_char_pointer(&rel.child),
        role: optional_string_to_char_pointer(&rel.role),
    })
}

pub(super) unsafe fn to_relationship(
    rel: &CEntityRelationship,
) -> Result<EntityRelationship, LoreCoreError> {
    Ok(EntityRelationship {
        parent: char_pointer_to_string(rel.parent)?,
        child: char_pointer_to_string(rel.child)?,
        role: char_pointer_to_optional_string(rel.role)?,
    })
}

#[cfg(test)]
mod tests {
    use crate::timestamp::current_timestamp;

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
                        label: label.to_string(),
                        descriptor: descriptor.to_string(),
                        description: description.clone(),
                    };
                    let c_column = to_c_entity_column(&column_before).unwrap();
                    let column_after = unsafe { to_entity_column(&c_column).unwrap() };
                    assert_eq!(column_before, column_after);
                }
            }
        }
    }

    #[test]
    fn history_item_roundtrips() {
        let years = vec![-13, 2021];
        let days = vec![None, Some(14)];
        let contents = vec!["", "Test content", "\\_\"'%$&!{[]}"];
        let properties = vec![
            None,
            Some("{\"is_secret\":true}".to_string()),
            Some("{\"additional_concerns\":[\"\\entityref{some_label}\"]}".to_string()),
        ];
        for year in years {
            for day in &days {
                for content in &contents {
                    for property in &properties {
                        let item_before = HistoryItem {
                            timestamp: current_timestamp(),
                            year,
                            day: *day,
                            content: content.to_string(),
                            properties: property.clone(),
                        };
                        let c_item = to_c_history_item(&item_before).unwrap();
                        let item_after = unsafe { to_history_item(&c_item).unwrap() };
                        assert_eq!(item_before, item_after);
                    }
                }
            }
        }
    }

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
