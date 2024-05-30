use std::fmt::Display;

use super::{
    day::Day, history_item_content::HistoryItemContent,
    history_item_properties::HistoryItemProperties, timestamp::Timestamp, year::Year,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryItem {
    pub timestamp: Timestamp,
    pub year: Year,
    pub day: Day,
    pub content: HistoryItemContent,
    pub properties: HistoryItemProperties,
}

impl PartialOrd for HistoryItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HistoryItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.year.cmp(&other.year) {
            std::cmp::Ordering::Equal => match self.day.cmp(&other.day) {
                std::cmp::Ordering::Equal => self.timestamp.cmp(&other.timestamp),
                other => other,
            },
            other => other,
        }
    }
}

impl Display for HistoryItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{}: {}",
            self.year.to_int(),
            self.day.to_int(),
            self.content.to_str()
        )
    }
}
