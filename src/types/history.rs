use super::{day::Day, history_item_content::HistoryItemContent, timestamp::Timestamp, year::Year};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HistoryItem {
    pub timestamp: Timestamp,
    pub year: Year,
    pub day: Day,
    pub content: HistoryItemContent,
    pub properties: Option<String>, //TODO: Use stronger typing
}
