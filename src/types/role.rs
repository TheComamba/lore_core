use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Role(pub(crate) String);

impl Role {
    pub const NONE: Role = Role(String::new());

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

    pub fn is_protected(&self) -> bool {
        self.0.starts_with('_')
    }
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Role {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Option<String>> for Role {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(value) => Self(value),
            None => Self::NONE,
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
