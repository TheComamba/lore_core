use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Year(pub(crate) i32);

impl Year {
    pub fn to_int(&self) -> i32 {
        self.0
    }
}

impl From<i32> for Year {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
