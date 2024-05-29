use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HistoryItemContent(pub(crate) String);

impl HistoryItemContent {
    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for HistoryItemContent {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for HistoryItemContent {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for HistoryItemContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
