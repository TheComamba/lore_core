use std::fmt::Display;

use super::label::Label;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Parent(pub(crate) String);

impl Parent {
    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Parent {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Parent {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Label> for Parent {
    fn from(value: Label) -> Self {
        Self(value.to_string())
    }
}

impl Display for Parent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
