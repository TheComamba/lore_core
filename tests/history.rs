use lorecore::sql::history::HistoryItem;
use lorecore::sql::lore_database::LoreDatabase;
use lorecore::sql::search_params::{HistoryItemSearchParams, SqlSearchText};
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
    let item_out = db
        .get_history_items(HistoryItemSearchParams::empty())
        .unwrap();
    assert!(item_out.len() == 1);
    assert!(item == item_out[0]);
    temp_path.close().unwrap();
}

fn create_example() -> (tempfile::TempPath, LoreDatabase, Vec<HistoryItem>) {
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
    items.sort();

    db.write_history_items(items.clone()).unwrap();
    (temp_path, db, items)
}

#[test]
fn write_many_history_items() {
    let (temp_path, db, items) = create_example();

    let items_out = db
        .get_history_items(HistoryItemSearchParams::empty())
        .unwrap();
    assert!(items_out == items);
    temp_path.close().unwrap();
}

#[test]
fn get_all_history_items() {
    let (temp_path, db, items) = create_example();

    let items_out = db
        .get_history_items(HistoryItemSearchParams::empty())
        .unwrap();
    assert!(items_out == items);

    temp_path.close().unwrap();
}

#[test]
fn get_history_items_by_year() {
    let (temp_path, db, items) = create_example();
    let year = items[0].year;
    let expected_items: Vec<_> = items.iter().filter(|item| item.year == year).collect();

    let items_out = db
        .get_history_items(HistoryItemSearchParams::new(Some(year), None, None, None))
        .unwrap();
    assert!(items_out == expected_items);

    temp_path.close().unwrap();
}

#[test]
fn get_history_items_by_day() {
    let (temp_path, db, items) = create_example();
    let day = items[0].day;
    let expected_items: Vec<_> = items.iter().filter(|item| item.day == day).collect();

    let items_out = db
        .get_history_items(HistoryItemSearchParams::new(None, day, None, None))
        .unwrap();
    assert!(items_out == expected_items);

    temp_path.close().unwrap();
}

#[test]
fn get_history_item_by_timestamp() {
    let (temp_path, db, items) = create_example();
    let timestamp = items[0].timestamp;

    let items_out = db
        .get_history_items(HistoryItemSearchParams::new(
            None,
            None,
            Some(timestamp),
            None,
        ))
        .unwrap();
    assert!(items_out.len() == 1);
    assert!(items_out[0] == items[0]);

    temp_path.close().unwrap();
}

#[test]
fn get_history_itmes_with_content_filter() {
    let (temp_path, db, items) = create_example();
    let content = "tent1".to_string();
    let expected_items: Vec<_> = items
        .iter()
        .filter(|item| item.content.contains(content.as_str()))
        .collect();
    let content_search = SqlSearchText::partial(&content);

    let items_out = db
        .get_history_items(HistoryItemSearchParams::new(
            None,
            None,
            None,
            Some(content_search),
        ))
        .unwrap();
    assert!(items_out == expected_items);

    temp_path.close().unwrap();
}

#[test]
fn get_history_itmes_with_exact_content_filter() {
    let (temp_path, db, items) = create_example();
    let content = "testcontent1".to_string();
    let expected_items: Vec<_> = items
        .iter()
        .filter(|item| item.content == content)
        .collect();
    let content_search = SqlSearchText::exact(&content);

    let items_out = db
        .get_history_items(HistoryItemSearchParams::new(
            None,
            None,
            None,
            Some(content_search),
        ))
        .unwrap();
    assert!(items_out == expected_items);

    temp_path.close().unwrap();
}

#[test]
fn get_history_items_by_year_and_day() {
    let (temp_path, db, items) = create_example();
    let year = items[0].year;
    let day = items[0].day;
    let expected_items: Vec<_> = items
        .iter()
        .filter(|item| item.year == year && item.day == day)
        .collect();

    let items_out = db
        .get_history_items(HistoryItemSearchParams::new(Some(year), day, None, None))
        .unwrap();
    assert!(items_out == expected_items);

    temp_path.close().unwrap();
}

#[test]
fn search_for_non_existing_year() {
    let (temp_path, db, _items) = create_example();

    let year = 65537;
    let items_out = db
        .get_history_items(HistoryItemSearchParams::new(Some(year), None, None, None))
        .unwrap();
    assert!(items_out.len() == 0);

    temp_path.close().unwrap();
}

#[test]
fn search_for_non_existing_day() {
    let (temp_path, db, _items) = create_example();

    let day = Some(65537);
    let items_out = db
        .get_history_items(HistoryItemSearchParams::new(None, day, None, None))
        .unwrap();
    assert!(items_out.len() == 0);

    temp_path.close().unwrap();
}

#[test]
fn search_for_non_existing_timestamp() {
    let (temp_path, db, _items) = create_example();

    let timestamp = 65537;
    let items_out = db
        .get_history_items(HistoryItemSearchParams::new(
            None,
            None,
            Some(timestamp),
            None,
        ))
        .unwrap();
    assert!(items_out.len() == 0);

    temp_path.close().unwrap();
}

#[test]
fn search_for_non_existing_content() {
    let (temp_path, db, _items) = create_example();

    let content = "nonexistingcontent".to_string();
    let content_search = SqlSearchText::partial(&content);
    let items_out = db
        .get_history_items(HistoryItemSearchParams::new(
            None,
            None,
            None,
            Some(content_search),
        ))
        .unwrap();
    assert!(items_out.len() == 0);

    temp_path.close().unwrap();
}
