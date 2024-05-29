use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Descriptor(pub(crate) String);

impl Descriptor {
    pub const NONE: Descriptor = Descriptor(String::new());

    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Descriptor {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Descriptor {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
