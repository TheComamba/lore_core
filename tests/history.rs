use lorecore::sql::history::HistoryItem;
use lorecore::sql::lore_database::LoreDatabase;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[test]
fn write_single_history_item() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();
    let item = HistoryItem {
        year: 2020,
        day: Some(1),
        label: "testlabel".to_string(),
        content: "testcontent".to_string(),
        is_concerns_others: false,
        is_secret: false,
        originator: None,
    };
    db.write_history_items(vec![item.clone()]).unwrap();
    let item_out = db.get_all_history_items().unwrap();
    assert!(item_out.len() == 1);
    assert!(item == item_out[0]);
    temp_path.close().unwrap();
}
