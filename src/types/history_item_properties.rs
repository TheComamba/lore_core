use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryItemProperties(pub(crate) HashMap<String, Value>);

impl HistoryItemProperties {
    pub fn none() -> HistoryItemProperties {
        HistoryItemProperties(HashMap::new())
    }

    pub fn to_map(&self) -> &HashMap<String, Value> {
        &self.0
    }
}

impl From<HashMap<String, Value>> for HistoryItemProperties {
    fn from(value: HashMap<String, Value>) -> Self {
        Self(value)
    }
}

impl From<&String> for HistoryItemProperties {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for HistoryItemProperties {
    fn from(value: &str) -> Self {
        match serde_json::from_str(value) {
            Ok(value) => Self(value),
            Err(_) => Self::none(),
        }
    }
}

impl Display for HistoryItemProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(&self.0).unwrap().fmt(f)
    }
}
