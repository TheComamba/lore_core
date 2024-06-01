use std::{fmt::Display, ops::Add};

use crate::errors::LoreCoreError;

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

impl TryFrom<&str> for Day {
    type Error = LoreCoreError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<u32>() {
            Ok(value) => Ok(Self(Some(value))),
            Err(_) => Err(LoreCoreError::InputError(format!(
                "Unable to parse \"{}\" as day",
                value
            ))),
        }
    }
}

impl TryFrom<String> for Day {
    type Error = LoreCoreError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
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

impl Add<u32> for Day {
    type Output = Day;

    fn add(self, rhs: u32) -> Self::Output {
        if let Some(value) = self.0 {
            Day(Some(value + rhs))
        } else {
            Day(None)
        }
    }
}
