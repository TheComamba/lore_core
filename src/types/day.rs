use diesel::expression::Expression;
use diesel::sql_types::{Integer, Nullable};
use diesel::Insertable;
use std::fmt::Display;

use crate::sql::schema::history_items;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Day(pub Option<i32>);

impl Day {
    pub const NONE: Self = Day(None);
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => value.fmt(f),
            None => "".fmt(f),
        }
    }
}

impl Expression for Day {
    type SqlType = Nullable<Integer>;
}

impl Insertable<history_items::table> for Day {
    type Values = Option<i32>;

    fn values(self) -> Self::Values {
        self.0
    }
}
