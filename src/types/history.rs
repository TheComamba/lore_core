use super::day::Day;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HistoryItem {
    pub timestamp: i64,
    pub year: i32,
    pub day: Day,
    pub content: String,
    pub properties: Option<String>,
}
