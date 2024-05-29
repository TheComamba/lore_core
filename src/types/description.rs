use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Description(pub(crate) String);

impl Description {
    pub const NONE: Description = Description(String::new());

    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn to_optional_string(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.clone())
        }
    }

    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Description {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Description {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Option<String>> for Description {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(value) => Self(value),
            None => Self::NONE,
        }
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
