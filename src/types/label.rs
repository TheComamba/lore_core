use std::fmt::Display;

use super::{child::Child, parent::Parent};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label(pub(crate) String);

impl Label {
    pub fn to_str(&self) -> &str {
        &self.0
    }

    pub fn is_protected(&self) -> bool {
        self.0.starts_with('_')
    }
}

impl From<&str> for Label {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Label {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Child> for Label {
    fn from(value: Child) -> Self {
        Self(value.to_string())
    }
}

impl From<Parent> for Label {
    fn from(value: Parent) -> Self {
        Self(value.to_string())
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
