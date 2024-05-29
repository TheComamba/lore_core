use super::day::Day;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HistoryItem {
    pub timestamp: i64, //TODO: Use stronger typing
    pub year: i32,      //TODO: Use stronger typing
    pub day: Day,
    pub content: String,            //TODO: Use stronger typing
    pub properties: Option<String>, //TODO: Use stronger typing
}
