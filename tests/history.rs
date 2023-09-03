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
        properties: None,
    };
    db.write_history_items(vec![item.clone()]).unwrap();
    let item_out = db.get_all_history_items().unwrap();
    assert!(item_out.len() == 1);
    assert!(item == item_out[0]);
    temp_path.close().unwrap();
}

#[test]
fn write_many_history_items() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();

    let contents = vec!["testcontent1".to_string(), "testcontent2".to_string()];
    let days = vec![Some(1), None];
    let properties = vec![Some("{\"is_secret\": true}".to_string()), None];
    let mut items: Vec<HistoryItem> = Vec::new();
    for content in contents.iter() {
        for property in properties.iter() {
            for day in days.iter() {
                let unique_label = day.map(|d| d.to_string()).unwrap_or("".to_string())
                    + content
                    + &property.clone().map(|o| o).unwrap_or("".to_string());
                items.push(HistoryItem {
                    year: 2020,
                    day: day.clone(),
                    label: unique_label,
                    content: content.clone(),
                    properties: property.clone(),
                });
            }
        }
    }

    db.write_history_items(items.clone()).unwrap();

    let items_out = db.get_all_history_items().unwrap();
    assert!(items.len() == items_out.len());
    for item in items.iter() {
        assert!(items_out.contains(item));
    }
    temp_path.close().unwrap();
}
