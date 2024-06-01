use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum LoreCoreError {
    FileError(String),
    InputError(String),
    SqlError(String),
}
impl Display for LoreCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub(super) fn sql_loading_error<E>(
    loadee: &str,
    params: Vec<(&str, &dyn Debug)>,
    err: E,
) -> LoreCoreError
where
    E: Display,
{
    let mut message = "Loading ".to_string() + loadee + " ";
    for (i, (name, value)) in params.iter().enumerate() {
        if i == 0 {
            message += " for parameters ";
        } else {
            message += ", "
        }
        message += name;
        message += "='";
        message += &format!("{:?}", value);
        message += "'";
    }
    message += " failed: ";
    message += &err.to_string();
    LoreCoreError::SqlError(message)
}
