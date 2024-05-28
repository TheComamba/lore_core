#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HistoryItem {
    pub timestamp: i64,
    pub year: i32,
    pub day: Option<i32>,
    pub content: String,
    pub properties: Option<String>,
}
