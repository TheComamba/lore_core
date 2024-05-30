use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use crate::errors::LoreCoreError;

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

impl TryFrom<&str> for Year {
    type Error = LoreCoreError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<i32>() {
            Ok(value) => Ok(Self(value)),
            Err(_) => Err(LoreCoreError::InputError(format!(
                "Unable to parse \"{}\" as year",
                value
            ))),
        }
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Add<i32> for Year {
    type Output = Year;

    fn add(self, rhs: i32) -> Self::Output {
        Year(self.0 + rhs)
    }
}

impl Sub<i32> for Year {
    type Output = Year;

    fn sub(self, rhs: i32) -> Self::Output {
        Year(self.0 - rhs)
    }
}
