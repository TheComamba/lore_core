use super::{day::Day, year::Year};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HistoryItem {
    pub timestamp: i64, //TODO: Use stronger typing
    pub year: Year,
    pub day: Day,
    pub content: String,            //TODO: Use stronger typing
    pub properties: Option<String>, //TODO: Use stronger typing
}
