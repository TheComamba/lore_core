//! This module contains the types used in the C API.
//! The only types for members are integers and c_char pointers.

pub(crate) mod entity;
pub(crate) mod history;
pub(crate) mod relationship;

pub use entity::CEntityColumn;
pub use history::CHistoryItem;
pub use relationship::CEntityRelationship;
