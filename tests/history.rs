use lorecore::sql::history::HistoryItem;
use lorecore::sql::lore_database::LoreDatabase;
use lorecore::timestamp::current_timestamp;
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
        timestamp: current_timestamp(),
        content: "testcontent".to_string(),
        properties: None,
    };
    db.write_history_items(vec![item.clone()]).unwrap();
    let item_out = db.get_all_history_items().unwrap();
    assert!(item_out.len() == 1);
    assert!(item == item_out[0]);
    temp_path.close().unwrap();
}

fn create_example() -> (
    tempfile::TempPath,
    LoreDatabase,
    Vec<HistoryItem>,
    Vec<i32>,
    Vec<Option<i32>>,
    Vec<String>,
) {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();

    let years = vec![-13, 0, 2021];
    let days = vec![Some(1), None];
    let contents = vec!["testcontent1".to_string(), "testcontent2".to_string()];
    let properties = vec![Some("{\"is_secret\": true}".to_string()), None];
    let mut items: Vec<HistoryItem> = Vec::new();
    for year in years.iter() {
        for day in days.iter() {
            for content in contents.iter() {
                for property in properties.iter() {
                    items.push(HistoryItem {
                        year: *year,
                        day: day.clone(),
                        timestamp: current_timestamp(),
                        content: content.clone(),
                        properties: property.clone(),
                    });
                }
            }
        }
    }

    db.write_history_items(items.clone()).unwrap();
    (temp_path, db, items, years, days, contents)
}

#[test]
fn write_many_history_items() {
    let (temp_path, db, items, _, _, _) = create_example();

    let items_out = db.get_all_history_items().unwrap();
    assert!(items.len() == items_out.len());
    for item in items.iter() {
        assert!(items_out.contains(item));
    }
    temp_path.close().unwrap();
}

#[test]
fn get_all_years() {
    let (temp_path, db, _, years, _, _) = create_example();

    let years_out = db.get_all_years().unwrap();
    assert!(years.len() == years_out.len());
    for year in years.iter() {
        assert!(years_out.contains(year));
    }
    temp_path.close().unwrap();
}

#[test]
fn get_days() {
    let (temp_path, db, _, years, days, _) = create_example();

    for year in years.iter() {
        let days_out = db.get_days(*year).unwrap();
        assert!(days.len() == days_out.len());
        for day in days.iter() {
            assert!(days_out.contains(day));
        }
    }
    temp_path.close().unwrap();
}

#[test]
fn get_timestamps() {
    let (temp_path, db, items, _, _, _) = create_example();

    todo!();
    temp_path.close().unwrap();
}
