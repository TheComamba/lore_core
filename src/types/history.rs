use super::{day::Day, timestamp::Timestamp, year::Year};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HistoryItem {
    pub timestamp: Timestamp,
    pub year: Year,
    pub day: Day,
    pub content: String,            //TODO: Use stronger typing
    pub properties: Option<String>, //TODO: Use stronger typing
}
