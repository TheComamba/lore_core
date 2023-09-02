use lorecore::sql::lore_database::LoreDatabase;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[test]
fn path_as_string() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();
    let path_out = db.path_as_string();
    assert!(path_in == PathBuf::from(path_out));
    temp_path.close().unwrap();
}
