use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Day(pub(crate) Option<u32>);

impl Day {
    pub const NONE: Day = Day(None);

    pub fn to_int(&self) -> u32 {
        self.0.unwrap_or(0)
    }

    pub fn to_optional_int(&self) -> Option<u32> {
        self.0
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }
}

impl From<u32> for Day {
    fn from(value: u32) -> Self {
        if value > 0 {
            Self(Some(value))
        } else {
            Self(None)
        }
    }
}

impl From<i32> for Day {
    fn from(value: i32) -> Self {
        if value > 0 {
            Self(Some(value as u32))
        } else {
            Self(None)
        }
    }
}

impl From<Option<u32>> for Day {
    fn from(value: Option<u32>) -> Self {
        Self(value)
    }
}

impl From<Option<i32>> for Day {
    fn from(value: Option<i32>) -> Self {
        match value {
            Some(value) => Self(Some(value as u32)),
            None => Self(None),
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => value.fmt(f),
            None => "".fmt(f),
        }
    }
}
