use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label(pub(crate) String);

impl Label {
    pub const NONE: Label = Label(String::new());

    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn to_str(&self) -> &str {
        &self.0
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

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
