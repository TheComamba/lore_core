use super::{
    auxil::{char_pointer_to_string, string_to_char_pointer},
    types::CEntityColumn,
};
use crate::{errors::LoreCoreError, sql::lore_database::LoreDatabase};

pub(super) fn c_read_entity_columns(
    db_path: *const libc::c_char,
) -> Result<Vec<CEntityColumn>, LoreCoreError> {
    let db_path = char_pointer_to_string(db_path)?;
    let db = LoreDatabase::open(db_path.into())?;
    let mut columns = Vec::new();
    let labels = db.get_all_entity_labels()?;
    for label in labels {
        let descriptors = db.get_descriptors(&Some(&label))?;
        for descriptor in descriptors {
            let description = db.get_description(&label, &descriptor)?;
            columns.push(CEntityColumn {
                label: string_to_char_pointer(&label),
                descriptor: string_to_char_pointer(&descriptor),
                description: string_to_char_pointer(&description),
            });
        }
    }
    Ok(columns)
}
