use lorecore::sql::lore_database::LoreDatabase;
use lorecore::sql::search_params::{HistoryItemSearchParams, SqlSearchText};
use lorecore::timestamp::current_timestamp;
use lorecore::types::day::Day;
use lorecore::types::history::HistoryItem;
use lorecore::types::year::Year;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[test]
fn write_single_history_item() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();
    let item = HistoryItem {
        year: 2020.into(),
        day: 1.into(),
        timestamp: current_timestamp(),
        content: "testcontent".to_string(),
        properties: None,
    };
    db.write_history_items(vec![item.clone()]).unwrap();
    let item_out = db
        .read_history_items(HistoryItemSearchParams::empty())
        .unwrap();
    assert!(item_out.len() == 1);
    assert!(item == item_out[0]);
    temp_path.close().unwrap();
}

fn create_example() -> (tempfile::TempPath, LoreDatabase, Vec<HistoryItem>) {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();

    let years: Vec<Year> = vec![(-13 as i32).into(), 0.into(), 2021.into()];
    let days: Vec<Day> = vec![1.into(), Day::NONE];
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
        .read_history_items(HistoryItemSearchParams::empty())
        .unwrap();
    assert!(items_out == items);
    temp_path.close().unwrap();
}

#[test]
fn get_all_history_items() {
    let (temp_path, db, items) = create_example();

    let items_out = db
        .read_history_items(HistoryItemSearchParams::empty())
        .unwrap();
    assert!(items_out == items);

    temp_path.close().unwrap();
}

#[test]
fn get_history_items_by_year() {
    let (temp_path, db, items) = create_example();
    let year = items[0].year;
    let expected_items: Vec<_> = items.into_iter().filter(|item| item.year == year).collect();

    let items_out = db
        .read_history_items(HistoryItemSearchParams::new(Some(year), None, None, None))
        .unwrap();
    assert!(items_out == expected_items);

    temp_path.close().unwrap();
}

#[test]
fn get_history_items_by_day() {
    let (temp_path, db, items) = create_example();
    let day = items[0].day;
    let expected_items: Vec<_> = items.into_iter().filter(|item| item.day == day).collect();

    let items_out = db
        .read_history_items(HistoryItemSearchParams::new(None, Some(day), None, None))
        .unwrap();
    assert!(items_out == expected_items);

    temp_path.close().unwrap();
}

#[test]
fn get_history_items_without_specified_day() {
    let (temp_path, db, items) = create_example();
    let expected_items: Vec<_> = items
        .into_iter()
        .filter(|item| item.day == Day::NONE)
        .collect();

    let items_out = db
        .read_history_items(HistoryItemSearchParams::new(
            None,
            Some(Day::NONE),
            None,
            None,
        ))
        .unwrap();
    assert!(items_out == expected_items);

    temp_path.close().unwrap();
}

#[test]
fn get_history_item_by_timestamp() {
    let (temp_path, db, items) = create_example();
    let timestamp = items[0].timestamp;

    let items_out = db
        .read_history_items(HistoryItemSearchParams::new(
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
        .into_iter()
        .filter(|item| item.content.contains(content.as_str()))
        .collect();
    let content_search = SqlSearchText::partial(&content);

    let items_out = db
        .read_history_items(HistoryItemSearchParams::new(
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
        .into_iter()
        .filter(|item| item.content == content)
        .collect();
    let content_search = SqlSearchText::exact(&content);

    let items_out = db
        .read_history_items(HistoryItemSearchParams::new(
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
        .into_iter()
        .filter(|item| item.year == year && item.day == day)
        .collect();

    let items_out = db
        .read_history_items(HistoryItemSearchParams::new(
            Some(year),
            Some(day),
            None,
            None,
        ))
        .unwrap();
    assert!(items_out == expected_items);

    temp_path.close().unwrap();
}

#[test]
fn search_for_non_existing_year() {
    let (temp_path, db, _items) = create_example();

    let year = 65537.into();
    let items_out = db
        .read_history_items(HistoryItemSearchParams::new(Some(year), None, None, None))
        .unwrap();
    assert!(items_out.len() == 0);

    temp_path.close().unwrap();
}

#[test]
fn search_for_non_existing_day() {
    let (temp_path, db, _items) = create_example();

    let day = Some(65537.into());
    let items_out = db
        .read_history_items(HistoryItemSearchParams::new(None, day, None, None))
        .unwrap();
    assert!(items_out.len() == 0);

    temp_path.close().unwrap();
}

#[test]
fn search_for_non_existing_timestamp() {
    let (temp_path, db, _items) = create_example();

    let timestamp = 65537;
    let items_out = db
        .read_history_items(HistoryItemSearchParams::new(
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
        .read_history_items(HistoryItemSearchParams::new(
            None,
            None,
            None,
            Some(content_search),
        ))
        .unwrap();
    assert!(items_out.len() == 0);

    temp_path.close().unwrap();
}

#[test]
fn test_write_read_history_after_db_deletion() {
    let (temp_path, db, _) = create_example();
    temp_path.close().unwrap();

    let write_result = db.write_history_items(vec![HistoryItem {
        year: 2020.into(),
        day: 1.into(),
        timestamp: current_timestamp(),
        content: "testcontent".to_string(),
        properties: None,
    }]);
    assert!(
        write_result.is_err(),
        "Expected an error when writing to a deleted database"
    );

    let read_result = db.read_history_items(HistoryItemSearchParams::new(None, None, None, None));
    assert!(
        read_result.is_err(),
        "Expected an error when reading from a deleted database"
    );
}

#[test]
fn test_setting_year() {
    let (temp_path, db, mut items) = create_example();
    let item = items.pop().unwrap();
    let old_year = item.year;
    let new_year = old_year + 12345;

    db.redate_history_item(item.timestamp, new_year, item.day)
        .unwrap();

    let updated_item = db
        .read_history_items(HistoryItemSearchParams::new(
            None,
            None,
            Some(item.timestamp),
            None,
        ))
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(updated_item.year, new_year);
    assert_ne!(updated_item.year, old_year);

    temp_path.close().unwrap();
}

#[test]
fn test_setting_day_to_some() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();

    let item = HistoryItem {
        year: 12.into(),
        day: Day::NONE,
        timestamp: current_timestamp(),
        content: "testcontent".to_string(),
        properties: None,
    };

    db.write_history_items(vec![item.clone()].clone()).unwrap();

    let old_day = item.day;
    let new_day = 12345.into();

    db.redate_history_item(item.timestamp, item.year, new_day)
        .unwrap();

    let updated_item = db
        .read_history_items(HistoryItemSearchParams::new(
            Some(item.year),
            Some(new_day),
            Some(item.timestamp),
            None,
        ))
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(updated_item.day, new_day);
    assert_ne!(updated_item.day, old_day);

    temp_path.close().unwrap();
}

#[test]
fn test_setting_day_to_none() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();

    let item = HistoryItem {
        year: 12.into(),
        day: 34.into(),
        timestamp: current_timestamp(),
        content: "testcontent".to_string(),
        properties: None,
    };

    db.write_history_items(vec![item.clone()].clone()).unwrap();

    let old_day = item.day;
    let new_day = Day::NONE;

    db.redate_history_item(item.timestamp, item.year, new_day)
        .unwrap();

    let updated_item = db
        .read_history_items(HistoryItemSearchParams::new(
            Some(item.year),
            Some(new_day),
            Some(item.timestamp),
            None,
        ))
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(updated_item.day, new_day);
    assert_ne!(updated_item.day, old_day);

    temp_path.close().unwrap();
}

#[test]
fn test_delete_history_item() {
    let (temp_path, db, mut items) = create_example();
    let item = items.pop().unwrap();

    db.delete_history_item(item.timestamp).unwrap();

    let items_out = db
        .read_history_items(HistoryItemSearchParams::new(None, None, None, None))
        .unwrap();

    assert!(!items_out.contains(&item));

    temp_path.close().unwrap();
}

#[test]
fn test_change_history_item_content() {
    let (temp_path, db, mut items) = create_example();
    let item = items.pop().unwrap();
    let new_content = "New_Content".to_string();

    db.change_history_item_content(item.timestamp, &new_content)
        .unwrap();

    let updated_item = db
        .read_history_items(HistoryItemSearchParams::new(
            None,
            None,
            Some(item.timestamp),
            None,
        ))
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(updated_item.content, new_content);

    temp_path.close().unwrap();
}

#[test]
fn test_changing_history_item_properties_to_some() {
    let (temp_path, db, mut items) = create_example();
    let item = items.pop().unwrap();
    let new_properties = Some("{\"is_secret\": false}".to_string());

    db.change_history_item_properties(item.timestamp, &new_properties)
        .unwrap();

    let updated_item = db
        .read_history_items(HistoryItemSearchParams::new(
            None,
            None,
            Some(item.timestamp),
            None,
        ))
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(updated_item.properties, new_properties);

    temp_path.close().unwrap();
}

#[test]
fn test_changing_history_item_properties_to_none() {
    let (temp_path, db, mut items) = create_example();
    let item = items.pop().unwrap();
    let new_properties = None;

    db.change_history_item_properties(item.timestamp, &new_properties)
        .unwrap();

    let updated_item = db
        .read_history_items(HistoryItemSearchParams::new(
            None,
            None,
            Some(item.timestamp),
            None,
        ))
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(updated_item.properties, new_properties);

    temp_path.close().unwrap();
}
